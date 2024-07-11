// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(non_snake_case)]

use memmap::Mmap;
use modules::{
    data_structs::log::{BeginTrial, EndTrial, Segment, SegmentType, Trialinit, ZoneInfo},
    parser::Lexer,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
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
    async fn get_trials() -> Vec<(BeginTrial, Option<EndTrial>)>;
}

#[derive(Clone, Default)]
struct ApiImpl {
    state: Arc<Mutex<ParserState>>,
    segments: Arc<Mutex<Option<Vec<Segment>>>>,
    zone_info_segments: Arc<Mutex<BTreeMap<usize, ZoneInfo>>>,
    trial_begin_segments: Arc<Mutex<BTreeMap<usize, BeginTrial>>>,
    trial_init_segments: Arc<Mutex<BTreeMap<usize, Trialinit>>>,
    trial_end_segments: Arc<Mutex<BTreeMap<usize, EndTrial>>>,
}

#[taurpc::resolvers]
impl Api for ApiImpl {
    async fn upload(self, path: PathBuf) {
        let file = File::open(path).unwrap();
        let mapped_file = unsafe { Mmap::map(&file).unwrap() };
        let mut lexer = Lexer::new(mapped_file.lines());
        let seg_len = lexer.get_len();
        let mut seg_array: Vec<Segment> = Vec::new();
        let mut zone_segs: BTreeMap<usize, ZoneInfo> = BTreeMap::new();
        let mut trial_begin_segs: BTreeMap<usize, BeginTrial> = BTreeMap::new();
        let mut trial_init_segs: BTreeMap<usize, Trialinit> = BTreeMap::new();
        let mut trial_end_segs: BTreeMap<usize, EndTrial> = BTreeMap::new();
        let mut count = 0;
        while let Some(segment) = lexer.next_segment() {
            count += 1;
            *self.state.lock().unwrap() = ParserState::Processing(count as f64 / seg_len as f64);

            match segment.line {
                SegmentType::ZoneInfo(x) => {
                    let _ = zone_segs.insert(segment.time, x);
                }
                SegmentType::TrialInit(x) => {
                    let _ = trial_init_segs.insert(segment.time, x);
                }

                SegmentType::BeginTrial(x) => {
                    let _ = trial_begin_segs.insert(segment.time, x);
                }

                SegmentType::EndTrial(x) => {
                    let _ = trial_end_segs.insert(segment.time, x);
                }

                _ => seg_array.push(segment),
            };
        }
        *self.segments.lock().unwrap() = Some(seg_array);
        *self.zone_info_segments.lock().unwrap() = zone_segs;
        *self.trial_begin_segments.lock().unwrap() = trial_begin_segs;
        *self.trial_init_segments.lock().unwrap() = trial_init_segs;
        *self.trial_end_segments.lock().unwrap() = trial_end_segs;
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
    async fn get_segment(self) -> SegmentType {
        todo!("Just a type check thing")
    }

    async fn get_trials(self) -> Vec<(BeginTrial, Option<EndTrial>)> {
        // self.trial_end_segments
        //     .lock()
        //     .unwrap()
        //     .values()
        //     .map(|f| f.to_owned())
        //     .collect::<Vec<_>>()
        self.trial_begin_segments
            .lock()
            .unwrap()
            .iter()
            .map(|(k, v)| {
                (
                    v.to_owned(),
                    self.trial_end_segments
                        .lock()
                        .unwrap()
                        .iter().skip_while(|(zk, _)| *zk <= k)
                        .find_map(|(_, zv)| if zv.id == v.id {Some(zv.to_owned())} else {None})
                        .take()
                )
            })
            .collect()
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
