use crate::store::segment::Segment;
use std::collections::BTreeMap;

// the number of values in a compressible block
const COMPRESSIBLE_BLOCK_SIZE: usize = 3;

pub struct SegmentIndex {
    pub sparse_index: BTreeMap<String, usize>,
}

impl SegmentIndex {
    pub fn from_segment(segment: Segment) -> SegmentIndex {
        let key_header_size = 8 + segment.keys.iter().fold(0, |s, k| s + 8 + k.len());
        let mut current_value_offset = 0;
        let mut sparse_index = BTreeMap::new();

        for (value_index, value) in segment.values.iter().enumerate() {
            let value_offset = key_header_size + current_value_offset;

            if value_index % COMPRESSIBLE_BLOCK_SIZE == 0 || value_index == segment.values.len() {
                let key = segment.keys[value_index].clone();
                sparse_index.insert(key, value_offset);
            }

            current_value_offset += 8 + value.len();
        }

        // each value in the sparse index points to the start of a compressible block
        SegmentIndex {
            sparse_index: sparse_index,
        }
    }
}
