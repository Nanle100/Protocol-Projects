pub struct Memory {
    data: Vec<u8>,
}

//memory is most commonly used to store values that cannot be stored in the Stack, such as arrays and strings.
impl Memory {
    pub fn new() -> Self {
        Memory { data: vec![] }
    }

    pub fn store(&mut self, offset: usize, value: &[u8]) {
        if self.data.len() < offset + value.len() {
            self.data.resize(offset + value.len(), 0);
        }
        self.data[offset..offset + value.len()].copy_from_slice(value);
    }

    pub fn load(&self, offset: usize, size: usize) -> Vec<u8> {
        if offset + size > self.data.len() {
            return vec![0; size];
        }
        self.data[offset..offset + size].to_vec()
    }
}
