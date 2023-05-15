mod storage;

#[cfg(feature = "with-rocksdb")]
mod rocks;
mod id;

#[cfg(feature = "with-rocksdb")]
pub use rocks::RocksDB;
pub use storage::Storage;
pub use id::IdGen;
