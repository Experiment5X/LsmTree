use std::collections::BTreeMap;
use std::collections::HashMap;

pub mod segment;

pub struct Store {
    full_store: HashMap<String, String>,
    mem_table: BTreeMap<String, String>,
}

impl Store {
    pub fn new() -> Store {
        Store {
            full_store: HashMap::new(),
            mem_table: BTreeMap::new(),
        }
    }

    pub fn put(&mut self, key: String, value: String) {
        self.full_store.insert(key.clone(), value.clone());
        self.mem_table.insert(key.clone(), value.clone());

        if self.mem_table.len() >= segment::MAX_MEM_TABLE_KEYS {
            self.persist_mem_table_to_segment();
            self.mem_table = BTreeMap::new();

            println!("Wrote mem table to segment");
        }
    }

    pub fn iter(
        &self,
    ) -> std::collections::hash_map::Iter<'_, std::string::String, std::string::String> {
        self.full_store.iter()
    }

    pub fn persist_mem_table_to_segment(&self) {
        let seg = segment::Segment::new_from_tree(&self.mem_table);

        seg.write_to_file().expect("write failed");
    }
}
