use crate::KonoIdGenerator;
pub use interfaces::RocksDB;
use interfaces::Storage;

impl<S: Storage> KonoIdGenerator<S> {
    /// Create a KonoIdGenerator with rocksdb.
    ///
    /// # Argument
    ///
    /// * `path` The path to store the data
    /// * `increment` The incremental change to the primary key
    /// * `base` The starting number for the primary key
    ///
    /// # Example
    ///
    /// ```rust
    /// use kono::KonoIdGenerator;
    /// use interfaces::RocksDB;
    ///
    /// let id_gen = KonoIdGenerator::<RocksDB>::new_with_rocksdb(String::from(".rocksdb"), 2, 0, 10, vec![0,0,0]);
    /// std::fs::remove_dir_all(String::from(".rocksdb")).unwrap();
    /// ```
    pub fn new_with_rocksdb(
        path: String,
        increment: u64,
        base: u64,
        shard_bits: u8,
        key: Vec<u8>
    ) -> KonoIdGenerator<RocksDB> {
        let storage = RocksDB::open(path);
        KonoIdGenerator::new(storage, increment, base, shard_bits, key)
    }
}
