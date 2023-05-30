/// The id generation trait used
pub trait IdGen {
    /// Generate the next id based on the sharding id
    fn next_id(&mut self, sharding_id: u64) -> u64;
    /// Generate the next ids based on the sharding id and the num of required ids
    fn next_ids(&mut self, _sharding_id: u64, _num: usize) -> Vec<u64> {
        todo!("not implemented yet")
    }
}
