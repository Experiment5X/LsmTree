extern crate file_utils;

use std::collections::BTreeMap;
use std::convert::TryInto;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Read;
use std::slice;
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
    pub keys: [String; MAX_MEM_TABLE_KEYS],
    pub values: [String; MAX_MEM_TABLE_KEYS],
}

impl Segment {
    pub fn new_from_tree(mem_table: &BTreeMap<String, String>) -> Segment {
        Segment {
            file_name: "hello".to_string(),
            keys: to_array(mem_table.keys().cloned().collect()),
            values: to_array(mem_table.values().cloned().collect()),
        }
    }

    pub fn new_from_file(file_name: String) -> Result<Segment, Error> {
        let mut segment_file = File::open("test.seg")?;

        let mut buffer: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
        segment_file.read(&mut buffer)?;
        let key_count = usize::from_le_bytes(buffer);

        if key_count != MAX_MEM_TABLE_KEYS {
            return Err(Error::from(ErrorKind::InvalidData));
        }

        let keys = Segment::read_strings(&mut segment_file)?;
        let values = Segment::read_strings(&mut segment_file)?;

        Ok(Segment {
            file_name: file_name,
            keys: keys,
            values: values,
        })
    }

    fn read_strings(file: &mut File) -> Result<[String; MAX_MEM_TABLE_KEYS], Error> {
        let strings = to_array(
            (0..MAX_MEM_TABLE_KEYS)
                .map(|_| {
                    let mut buffer: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
                    match file.read(&mut buffer) {
                        Ok(_) => (),
                        Err(error) => return Err(error),
                    }
                    let string_length = usize::from_le_bytes(buffer);

                    let mut string_buffer = Vec::<u8>::with_capacity(string_length);
                    let string_value = unsafe {
                        let buffer = slice::from_raw_parts_mut(
                            string_buffer.as_mut_ptr() as *mut u8,
                            string_length,
                        );
                        file.read_exact(buffer)?;
                        string_buffer.set_len(string_length);

                        let string_value = match String::from_utf8(string_buffer) {
                            Ok(string) => string,
                            Err(_) => "".to_string(),
                        };

                        string_value
                    };

                    Ok(string_value)
                })
                .filter_map(Result::ok)
                .collect(),
        );

        Ok(strings)
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
