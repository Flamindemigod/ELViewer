// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(non_snake_case)]

use memmap::Mmap;
use modules::{log::{data_structs::zone::Trial, log::Log}, parser::Lexer};
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
    async fn get_trials() -> Vec<Trial>;
    async fn get_log_start() -> u32;
    async fn test() -> Vec<Log>;
}

#[derive(Clone, Default)]
struct ApiImpl {
    state: Arc<Mutex<ParserState>>,
    logs: Arc<Mutex<Vec<Log>>>,
}

#[taurpc::resolvers]
impl Api for ApiImpl {
        async fn test(self) -> Vec<Log>{
            self.logs.lock().unwrap().clone()
        }

    async fn upload(self, path: PathBuf) {
        let file = File::open(path).unwrap();
        let mapped_file = unsafe { Mmap::map(&file).unwrap() };
        let mut lexer = Lexer::new(mapped_file.lines());
        self.logs.lock().unwrap().clear();
        self.logs.lock().unwrap().push(*Log::init(&mut lexer, self.state.clone()).unwrap());
        *self.state.lock().unwrap() = ParserState::Processed;
        // println!("Log: {:#?}", self.logs.lock().unwrap());
        drop(lexer);
        drop(mapped_file);
        drop(file);
        println!("Parsed Log");
    }


    async fn poll_state(self) -> ParserState {
        self.state.lock().unwrap().clone()
    }

    async fn get_trials(self) -> Vec<Trial> {
        self.logs.lock().unwrap().iter().flat_map(|f| f.zones.iter().filter_map(|z| z.trial.clone())).collect()
    }
    async fn get_log_start(self) -> u32 {
        self.logs.lock().unwrap().first().unwrap().start.time_since_epoch_s
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
