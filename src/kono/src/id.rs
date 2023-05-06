use interfaces::{IdGen, Storage};

extern crate test;

unsafe impl<S: Storage + Send> Send for KonoIdGenerator<S> {}

unsafe impl<S: Storage + Send> Sync for KonoIdGenerator<S> {}

const SHARDING_STORAGE_KEY: [u8; 10] = *b"shard_bits";

#[derive(Debug, Clone)]
struct BitsAndValue {
    bits: u64,
    value: u64,
}

impl BitsAndValue {
    fn new(bits: u64) -> Self {
        Self {
            bits,
            value: 1 << bits.clone(),
        }
    }
}

pub struct KonoIdGenerator<S: Storage> {
    storage: S,
    sharding: BitsAndValue,
    increment: u64,
    base: u64,
    max_id: u64,
    key: Vec<u8>,
}

impl<S: Storage> KonoIdGenerator<S> {
    fn check_with_previous_sharding_config(storage: &mut S, shard: &BitsAndValue) {
        if let Some(bits) = storage.get_u64(&SHARDING_STORAGE_KEY) {
            if shard.bits != bits {
                panic!(
                    "sharding not aligned, previous: {:?}, current: {:?}",
                    bits, shard.bits
                );
            }
        } else {
            storage.save_u64(&SHARDING_STORAGE_KEY, shard.clone().bits as u64);
        }
    }

    pub(crate) fn new(
        mut storage: S,
        increment: u64,
        base: u64,
        shard_bits: u8,
        key: Vec<u8>,
    ) -> Self {
        let sharding = BitsAndValue::new(shard_bits as u64);
        Self::check_with_previous_sharding_config(&mut storage, &sharding);
        log::info!("sharding config is {:?}", sharding.clone());

        let max_id = u64::MAX >> sharding.clone().bits;
        Self {
            storage,
            increment,
            sharding,
            base,
            max_id,
            key,
        }
    }
}

impl<S: Storage> IdGen for KonoIdGenerator<S> {
    fn next_id(&mut self, sharding_id: u64) -> u64 {
        let shard = sharding_id % self.sharding.value.clone();

        let id = self.storage.get_u64(&self.key).unwrap_or(self.base.clone());
        let next = id + self.increment.clone();
        if next > self.max_id {
            panic!("out of u64");
        }

        let mut result = next << self.sharding.bits.clone();
        result |= shard;

        // TODO: we should config the rocksdb to check the os cache management
        self.storage.save_u64(&self.key, next.clone());

        result
    }

    /// Generate the next ids based on the sharding id and the num of required ids
    fn next_ids(&mut self, sharding_id: u64, num: usize) -> Vec<u64> {
        let shard = sharding_id % self.sharding.value.clone();

        let mut next = self.storage.get_u64(&self.key).unwrap_or(self.base.clone());

        let r = (0..num).into_iter().map(|_i| {
            next += self.increment.clone();
            if next > self.max_id { panic!("out of u64"); }
            (next << self.sharding.bits.clone()) | shard
        }).collect();

        // TODO: we should config the rocksdb to check the os cache management
        self.storage.save_u64(&self.key, next.clone());

        r
    }
}

#[cfg(test)]
mod tests {
    use super::test::Bencher;

    #[test]
    #[cfg(feature = "with-rocksdb")]
    fn test_next_id() {
        use crate::rocks::RocksDB;
        use crate::KonoIdGenerator;
        use interfaces::IdGen;
        use std::fs;

        let path = String::from(".storage");
        fs::remove_dir_all(path.clone()).unwrap_or(());

        let mut id_gen =
            KonoIdGenerator::<RocksDB>::new_with_rocksdb(path.clone(), 2, 0, 10, vec![0, 0, 0]);
        assert_eq!(id_gen.next_id(0), 2048);
        assert_eq!(id_gen.next_id(0), 4096);

        fs::remove_dir_all(path).unwrap();
    }

    #[bench]
    #[cfg(feature = "with-rocksdb")]
    fn bench_next_id(b: &mut Bencher) {
        use crate::rocks::RocksDB;
        use crate::KonoIdGenerator;
        use interfaces::IdGen;
        use std::fs;

        let path = String::from(".storage2");
        fs::remove_dir_all(path.clone()).unwrap_or(());

        let mut id_gen =
            KonoIdGenerator::<RocksDB>::new_with_rocksdb(path.clone(), 2, 0, 10, vec![0, 0, 0]);

        b.iter(|| {
            (0..1000).fold(0, |_, _| {
                id_gen.next_id(10);
                0
            });
        });

        fs::remove_dir_all(path).unwrap();
    }
}
