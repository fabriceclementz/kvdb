use std::fs::{self, File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::PathBuf;

pub struct KvDb {
    // directory that contains data
    data_dir: PathBuf,
    // writer of the current segment
    writer: BufWriter<File>,
}

impl KvDb {
    /// Creates a new KvDB instance.
    pub fn new(data_dir: impl Into<PathBuf>) -> Self {
        // create the data directory if it doesn't exist
        let data_dir = data_dir.into();
        let _ = fs::create_dir_all(&data_dir).unwrap();

        // TODO: find the latest segment and open it for writing
        let current_segment = data_dir.join(format!("{}.log", "segment-1"));

        let current_segment_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(current_segment)
            .unwrap();

        let writer = BufWriter::new(current_segment_file);

        Self { data_dir, writer }
    }

    /// Set a key-value pair in the database.
    pub fn set(&mut self, key: String, value: String) {
        let set = Command::set(key, value);

        // TODO: serialize set and write it to the current segment

        self.writer
            .write_all(format!("{} {}\n", key, value).as_bytes())
            .unwrap();
    }

    /// Get a value from the database.
    pub fn get(&self, key: String) -> Option<String> {
        unimplemented!()
    }
}

// Command represents a command that can be send to the KvDb.
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

#[cfg(test)]
mod test {
    fn get_db() -> super::KvDb {
        super::KvDb::new()
    }

    #[test]
    fn set_and_get() {
        let mut db = get_db();
        db.set("key".into(), "value 1".into());
        assert_eq!(db.get("key".into()), Some("value 1".into()));
    }
}
