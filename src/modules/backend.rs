use std::{fs, io::BufRead, path::PathBuf};

use memmap::Mmap;

use super::{log::Segment, parser::Lexer};

pub struct Backend {
    segments: Vec<Segment>,
    segment_len: Option<usize>,
}

impl Backend {
    pub fn init() -> Self {
        Self {
            segments: Vec::new(),
            segment_len: None,
        }
    }

    pub async fn parse_file(&mut self, path: PathBuf) {
        let file = fs::File::open(path)
            .map_err(|err| eprintln!("{}:{} Error: {err}", file!(), line!()))
            .unwrap();
        let mapped_file = unsafe { Mmap::map(&file).unwrap() };
        let mut lexer = Lexer::new(mapped_file.lines());
        let seg_len = lexer.get_len();
        self.segment_len = Some(seg_len);
        self.segments = Vec::with_capacity(seg_len);
        while let Some(segment) = lexer.next_segment() {
            self.segments.push(segment);
        }
    }

    pub fn poll_progress(&self) -> Option<f32> {
        self.segment_len
            .map(|seg_len| self.segments.len() as f32 / seg_len as f32)
    }
}

// pub async fn parse_file(path: PathBuf) -> Vec<Segment> {
//     let file = fs::File::open(path).unwrap();
//     let mapped_file = unsafe { Mmap::map(&file).unwrap() };
//     let lines = mapped_file.lines();
//     let mut lexer = Lexer::new(lines);
//     println!("Len: {:#?}", lexer.get_len());
//     let mut segment_array = Vec::with_capacity(lexer.get_len());
//     //     println!("Done Parsing: segment array len = {}", segment_array.len());
//     segment_array
// }

