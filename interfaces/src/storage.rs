pub trait Storage {
    fn get_u64(&self, key: &[u8]) -> Option<u64>;
    fn save_u64(&mut self, key: &[u8], val: u64);
}
