pub trait BufferTrait {
    fn new() -> Self;
    fn add_byte_to_end(&mut self, value: u8);
    fn add_bytes_to_end(&mut self, value: &[u8]);
    fn as_bytes(&self) -> &[u8];
}
