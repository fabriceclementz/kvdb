use std::fs::{self, File, OpenOptions};
use std::io::{BufReader, BufWriter, Read};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Segment {
    pub writer: BufWriter<File>,
    pub reader: BufReader<File>,
}

impl Segment {
    pub fn open(path: &Path) -> Self {
        let segment_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open(&path)
            .unwrap();

        let writer = BufWriter::new(segment_file);
        let reader = BufReader::new(File::open(&path).unwrap());

        Segment { writer, reader }
    }
}

pub fn get_sorted_segments_num(data_dir: &PathBuf) -> Vec<u64> {
    let mut numbers: Vec<_> = fs::read_dir(data_dir)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.is_file() && path.extension() == Some("log".as_ref()))
        .map(|path| {
            let segment_number = path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .trim_start_matches("segment-")
                .trim_end_matches(".log")
                .parse::<u64>()
                .unwrap();

            segment_number
        })
        .collect();
    numbers.sort_unstable();
    numbers
}

pub fn get_segment_path(data_dir: &PathBuf, active_segment_num: &u64) -> PathBuf {
    data_dir.join(format!("segment-{}.log", active_segment_num))
}
