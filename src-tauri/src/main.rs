// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(non_snake_case)]

use memmap::Mmap;
use modules::{
    data_structs::log::{Segment, SegmentType},
    parser::Lexer,
};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::BufRead,
    path::PathBuf,
    sync::{Arc, Mutex},
};

mod modules;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[derive(Clone, Serialize, Deserialize, specta::Type)]
enum ParserState {
    None,
    Processing(f64),
    Processed,
}

impl Default for ParserState {
    fn default() -> Self {
        Self::None
    }
}

#[taurpc::procedures(export_to = "../src/types.ts")]
trait Api {
    async fn poll_state() -> ParserState;
    async fn upload(path: PathBuf);
    async fn get_trial_count() -> u32;
    async fn get_segment() -> SegmentType;
}

#[derive(Clone, Default)]
struct ApiImpl {
    state: Arc<Mutex<ParserState>>,
    segments: Arc<Mutex<Option<Vec<Segment>>>>,
}

#[taurpc::resolvers]
impl Api for ApiImpl {
    async fn upload(self, path: PathBuf) {
        let file = File::open(path).unwrap();
        let mapped_file = unsafe { Mmap::map(&file).unwrap() };
        let mut lexer = Lexer::new(mapped_file.lines());
        let seg_len = lexer.get_len();
        let mut seg_array: Vec<Segment> = Vec::with_capacity(seg_len);
        let mut count = 0;
        while let Some(segment) = lexer.next_segment() {
            count += 1;
            *self.state.lock().unwrap() = ParserState::Processing(count as f64 / seg_len as f64);
            seg_array.push(segment);
        }
        *self.segments.lock().unwrap() = Some(seg_array);
        *self.state.lock().unwrap() = ParserState::Processed;
    }

    async fn poll_state(self) -> ParserState {
        self.state.lock().unwrap().clone()
    }

    async fn get_trial_count(self) -> u32 {
        let segs = self.segments.lock().unwrap();
        segs.as_ref()
            .unwrap()
            .iter()
            .filter(|f| matches!(f.line, SegmentType::BeginTrial(_)))
            .count() as u32
    }
    async fn get_segment(self) -> SegmentType{
        todo!("Just a type check thing")
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(taurpc::create_ipc_handler(
            ApiImpl::default().into_handler(),
        ))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
