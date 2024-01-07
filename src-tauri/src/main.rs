// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use scraper::{Html, Selector};

#[tauri::command]
fn generate_new_set() -> Result<String, String> {
    let text: String;

    match reqwest::blocking::get("https://technikinformatyk.pl/kwalifikacja-inf-03-egzamin-online/") {
        Ok(v) => match v.text() {
            Ok(v) => text = v,
            Err(e) => return Err(e.to_string()),
        },
        Err(e) => return Err(e.to_string()),
    };

    let mut questions: Vec<String> = vec![];
    
    let fragment = Html::parse_document(text.as_str());

    for i in 1..41 {
        // question
        let question_text_selector: Selector;

        match Selector::parse(&format!("#pyt{} > .pytanietext", i).to_string()) {
            Ok(selector) => question_text_selector = selector,
            Err(e) => return Err(e.to_string()),
        };

        let mut question_text: String;

        match fragment.select(&question_text_selector).next() {
            Some(v) => question_text = v.inner_html(),
            None => return Err(String::from("'fragment.select(&question_text_selector).next()' returned None.")),
        };

        let question_identifier_length = "Pytanie ".len() + i.to_string().len() + 1;

        question_text = question_text[question_identifier_length..].to_string()
            .replace("<font>", "")
            .replace("</font>", "")
            .trim()
            .to_string();

        questions.push(question_text);
    };

    Ok(questions.join("\n").to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![generate_new_set])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
