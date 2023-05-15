use crate::storage::Storage;
use rocksdb::DB;

pub struct RocksDB {
    db: DB,
}

impl RocksDB {
    pub fn open(path: String) -> Self {
        let db = DB::open_default(path).expect("cannot launch rocksDB");
        Self { db }
    }
}

impl Storage for RocksDB {
    fn get_u64(&self, key: &[u8]) -> Option<u64> {
        self.db
            .get(key)
            .map(|f| {
                f.map(|v| {
                    let array: [u8; 8] = v.try_into().unwrap();
                    u64::from_be_bytes(array)
                })
            })
            .expect("db cannot be reached")
    }

    fn save_u64(&mut self, key: &[u8], val: u64) {
        self.db
            .put(key, val.to_be_bytes())
            .expect("save failed storing key");
    }
}
