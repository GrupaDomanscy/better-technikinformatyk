// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

mod models;
mod parsing;

struct ParserQuestionsState(pub Mutex<Vec<models::Question>>);

fn main() {
    tauri::Builder::default()
        .manage(ParserQuestionsState(Default::default()))
        .invoke_handler(tauri::generate_handler![parsing::generate_new_set, parsing::get_question_from_state])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
