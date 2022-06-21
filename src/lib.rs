mod segment;

use std::collections::BTreeMap;
use std::fs::{self};
use std::io::Seek;
use std::path::PathBuf;

use rmp_serde::{Deserializer, Serializer};
use segment::{get_segment_path, get_sorted_segments_num, Segment};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct KvDb {
    // directory that contains data
    data_dir: PathBuf,
    // Active segment to write in
    active_segment: Segment,
    // All the segments
    segments: Vec<Segment>,
    // Index that stores position of a key in a segment
    index: BTreeMap<String, CommandPos>,
}

impl KvDb {
    /// Creates a new KvDB instance.
    pub fn new(data_dir: impl Into<PathBuf>) -> Self {
        // create the data directory if it doesn't exist
        let data_dir = data_dir.into();
        let _ = fs::create_dir_all(&data_dir).unwrap();

        let mut index = BTreeMap::new();

        let sorted_list = get_sorted_segments_num(&data_dir);

        let active_segment_num = sorted_list.last().unwrap();
        let mut segments = vec![];
        for num in &sorted_list {
            if num != active_segment_num {
                let segment = Segment::open(&get_segment_path(&data_dir, num));
                load_segment_in_index(&segment, &index);
                segments.push(segment);
            }
        }

        let active_segment = Segment::open(&get_segment_path(&data_dir, active_segment_num));
        load_segment_in_index(&active_segment, &index);

        Self {
            data_dir,
            active_segment,
            segments: segments,
            index,
        }
    }

    /// Set a key-value pair in the database.
    pub fn set(&mut self, key: String, value: String) {
        let set = Command::set(key.clone(), value);
        let message = rmp_serde::to_vec(&set).unwrap();
        let _ = rmp_serde::encode::write(&mut self.active_segment.writer, &message).unwrap();

        // TODO: get the len of the current file and update the index
        // let pos = message.len();
        // let _ = self.index.insert(key, pos);
    }

    /// Get a value from the database.
    pub fn get(&self, key: String) -> Option<String> {
        unimplemented!()
    }
}

// Command represents a command that can be send to the KvDb.
#[derive(Debug, Serialize, Deserialize)]
enum Command {
    Set { key: String, value: String },
    Get { key: String },
}

impl Command {
    // Returns a new Set command.
    pub fn set(key: String, value: String) -> Self {
        Command::Set { key, value }
    }
    // Returns a new Get command.
    pub fn get(key: String) -> Self {
        Command::Get { key }
    }
}

#[derive(Debug)]
struct CommandPos {
    segment_num: u64,
    pos: u64,
    len: u64,
}

fn load_segment_in_index(segment: &Segment, index: &BTreeMap<String, CommandPos>) {
    // deserialize a segment and fill the index
    unimplemented!()
}

#[cfg(test)]
mod test {
    // fn get_db() -> super::KvDb {
    //     super::KvDb::new()
    // }

    // #[test]
    // fn set_and_get() {
    //     let mut db = get_db();
    //     db.set("key".into(), "value 1".into());
    //     assert_eq!(db.get("key".into()), Some("value 1".into()));
    // }
}
