// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};

mod models;
mod parsing;

struct ParserQuestionsState(pub Arc<Mutex<Vec<models::Question>>>);

#[tauri::command]
async fn generate_new_set<'a>(state: tauri::State<'a, ParserQuestionsState>) -> Result<(), String> {
    let state = state.0.clone();

    match parsing::generate_new_set().await {
        Ok(v) => {
            let mut state = state.lock().unwrap();
            *state = v;
        },
        Err(e) => return Err(e.to_string()),
    };

    Ok(())
}


#[tauri::command]
fn get_question_from_state(state: tauri::State<ParserQuestionsState>) -> Option<String> {
    let questions = &state.0.lock().unwrap();

    let question_obj = questions.get(0).unwrap();

    let question = question_obj.question().clone();

    return Some(question);
}

fn main() {
    tauri::Builder::default()
        .manage(ParserQuestionsState(Default::default()))
        .invoke_handler(tauri::generate_handler![generate_new_set, get_question_from_state])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
