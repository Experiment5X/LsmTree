use std::collections::BTreeMap;
use std::convert::TryInto;
use std::vec::Vec;

pub const MAX_MEM_TABLE_KEYS: usize = 10;

fn to_array<T>(v: Vec<T>) -> [T; MAX_MEM_TABLE_KEYS] {
    v.try_into().unwrap_or_else(|v: Vec<T>| {
        panic!(
            "Expected a Vec of length {} but it was {}",
            MAX_MEM_TABLE_KEYS,
            v.len()
        )
    })
}

pub struct Segment {
    file_name: String,
    keys: [String; MAX_MEM_TABLE_KEYS],
    values: [String; MAX_MEM_TABLE_KEYS],
}

impl Segment {
    pub fn new_from_tree(mem_table: &BTreeMap<String, String>) -> Segment {
        Segment {
            file_name: "hello".to_string(),
            keys: to_array(mem_table.keys().cloned().collect()),
            values: to_array(mem_table.keys().cloned().collect()),
        }
    }

    pub fn write_to_file(&self) {
        ()
    }
}
