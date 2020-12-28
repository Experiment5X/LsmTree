use std::collections::HashMap;

pub struct Store {
    mem_table: HashMap<String, String>,
}

impl Store {
    pub fn new() -> Store {
        Store {
            mem_table: HashMap::new(),
        }
    }

    pub fn put(&mut self, key: String, value: String) {
        self.mem_table.insert(key, value);
    }

    pub fn iter(
        &self,
    ) -> std::collections::hash_map::Iter<'_, std::string::String, std::string::String> {
        self.mem_table.iter()
    }
}
