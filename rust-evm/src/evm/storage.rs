use std::collections::HashMap;
use primitive_types::U256;

pub struct Storage {
    data: HashMap<U256, U256>,
}

impl Storage {
    pub fn new() -> Self {
        Storage { data: HashMap::new() }
    }

    pub fn store(&mut self, key: U256, value: U256) {
        self.data.insert(key, value);
    }

    pub fn load(&self, key: U256) -> U256 {
        *self.data.get(&key).unwrap_or(&U256::zero())
    }
}
