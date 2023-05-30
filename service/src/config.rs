use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Config {
    /// The increment for the primary id
    pub increment: u64,
    /// The starting number for the key if not present
    pub starting: u64,
    pub sharding_bits: u8,
    pub address: String,
    pub channel_size: usize,
    pub rocksdb_path: Option<String>,
    pub rocksdb_storage_key: Option<Vec<u8>>,
}

unsafe impl Send for Config {}

unsafe impl Sync for Config {}
