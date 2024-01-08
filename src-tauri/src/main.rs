// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};

use models::Question;

mod models;
mod parsing;

struct TestStateShape {
    answers: Vec<usize>,
    questions: Vec<models::Question>
}

impl TestStateShape {
    pub fn new() -> TestStateShape{
        return Self{
            answers: vec![],
            questions: vec![]
        };
    }

    fn questions(&self) -> &Vec<models::Question> {
        return &self.questions;
    }

    fn set_questions(&mut self, new_questions: Vec<models::Question>) {
        self.questions = new_questions;
    }
}

type TestState = Arc<Mutex<TestStateShape>>;

#[tauri::command]
async fn generate_new_set<'a>(state: tauri::State<'a, TestState>) -> Result<(), String> {
    let state = state.clone();

    match parsing::generate_new_set().await {
        Ok(v) => {
            let mut state = state.lock().unwrap();
            (*state).set_questions(v);
        },
        Err(e) => return Err(e.to_string()),
    };

    Ok(())
}

#[tauri::command]
fn get_question_from_state(state: tauri::State<TestState>) -> Option<String> {
    let state = state.clone();
    let state = state.lock().unwrap();

    let questions = state.questions();
    let question_obj = questions.get(0).unwrap();
    let question = question_obj.question().clone();

    return Some(question);
}

#[tauri::command]
fn get_question_from_state_by_index(index: usize, state: tauri::State<TestState>) -> Option<Question> {
    let state = state.clone();
    let state = state.lock().unwrap();
    
    let val = state.questions.get(index);

    if val.is_none() {
        return None;
    }

    return Some(val.unwrap().clone());
}

#[tauri::command]
fn get_question_count_from_state(state: tauri::State<TestState>) -> usize {
    let state = state.clone();
    let state = state.lock().unwrap();

    return state.questions().len();
}

#[tauri::command]
fn get_all_questions_from_state(state: tauri::State<TestState>) -> Vec<Question> {
    let state = state.clone();
    let state = state.lock().unwrap();

    let questions = state.questions();
    let questions = questions.clone();
    return questions;
}

fn main() {
    tauri::Builder::default()
        .manage(Arc::new(Mutex::new(TestStateShape::new())))
        .invoke_handler(tauri::generate_handler![
            generate_new_set, 
            get_question_from_state, 
            get_all_questions_from_state, 
            get_question_from_state_by_index,
            get_question_count_from_state,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
