use std::collections::BTreeMap;

pub mod segment;
pub mod segment_index;

pub struct Store {
    mem_table: BTreeMap<String, String>,
    next_segment_index: i64,
    segments: Vec<segment_index::SegmentIndex>,
}

impl Store {
    pub fn new() -> Store {
        Store {
            mem_table: BTreeMap::new(),
            next_segment_index: 0,
            segments: Vec::new(),
        }
    }

    pub fn put(&mut self, key: String, value: String) {
        self.mem_table.insert(key.clone(), value.clone());

        if self.mem_table.len() >= segment::MAX_MEM_TABLE_KEYS {
            let segment_data = self.persist_mem_table_to_segment();
            self.mem_table = BTreeMap::new();

            let seg_index = segment_index::SegmentIndex::from_segment(segment_data);
            self.segments.push(seg_index);
        }
    }

    pub fn lookup(&self, key: String) -> Option<String> {
        match self.mem_table.get(&key) {
            Some(value) => Some(value.clone()),
            None => {
                for segment_index in (0..self.next_segment_index).rev() {
                    let current_segment = match segment::Segment::new_from_file(format!(
                        "segment-{}.seg",
                        segment_index
                    )) {
                        Ok(seg) => seg,
                        Err(_) => return None,
                    };

                    match current_segment.lookup(&key) {
                        Some(value) => return Some(value),
                        None => (),
                    };
                }

                None
            }
        }
    }

    pub fn persist_mem_table_to_segment(&mut self) -> segment::Segment {
        let seg = segment::Segment::new_from_tree(&self.mem_table);

        seg.write_to_file(self.next_segment_index)
            .expect("write failed");
        self.next_segment_index += 1;

        return seg;
    }
}
