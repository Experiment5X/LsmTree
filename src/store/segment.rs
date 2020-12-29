use std::collections::BTreeMap;
use std::convert::TryInto;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::vec::Vec;

pub const MAX_MEM_TABLE_KEYS: usize = 5;
const DIRECTORY_PREFIX: &str = "./";

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
            values: to_array(mem_table.values().cloned().collect()),
        }
    }

    fn write_strings(&self, file: &mut File, strings: &[String]) -> Result<(), Error> {
        for string_to_write in strings.iter() {
            file.write(&string_to_write.len().to_ne_bytes())?;
            file.write(&string_to_write.clone().into_bytes())?;
        }

        Ok(())
    }

    pub fn write_to_file(&self) -> Result<(), Error> {
        let mut segment_file = File::create("test.seg")?;

        segment_file.write(&MAX_MEM_TABLE_KEYS.to_ne_bytes())?;

        self.write_strings(&mut segment_file, &self.keys)?;
        self.write_strings(&mut segment_file, &self.values)?;

        Ok(())
    }
}
