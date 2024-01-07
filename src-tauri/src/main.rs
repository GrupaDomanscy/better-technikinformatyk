// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::anyhow;
use scraper::{Html, Selector};

fn get_question(fragment: &Html, question_index: isize) -> anyhow::Result<String> {
    let question_text_selector: Selector;

    match Selector::parse(&format!("#pyt{} > .pytanietext", question_index).to_string()) {
        Ok(selector) => question_text_selector = selector,
        Err(e) => return Err(anyhow!("{}", e)),
    };

    let mut question_text: String;

    match fragment.select(&question_text_selector).next() {
        Some(v) => question_text = v.text().collect::<Vec<_>>().join("").to_string(),
        None => return Err(anyhow!("'fragment.select(&question_text_selector).next()' returned None.")),
    };

    let question_identifier_length = "Pytanie ".len() + question_index.to_string().len() + 1;

    question_text = question_text[question_identifier_length..].to_string()
        .replace("<font>", "")
        .replace("</font>", "")
        .trim()
        .to_string();

    return Ok(question_text);
}

fn get_answers(fragment: &Html, index: isize) -> anyhow::Result<Vec<String>> {
    let answers_str_selector: String = format!("#pyt{} > .odpcont", index).to_string();
    let answers_selector: Selector;

    match Selector::parse(&answers_str_selector) {
        Ok(v) => answers_selector = v,
        Err(e) => return Err(anyhow!(e.to_string()))
    };

    let mut answers: Vec<String> = vec![];

    let mut answers_iterator = fragment.select(&answers_selector);

    loop {
        let answer = answers_iterator.next();

        if answer.is_none() {
            break;
        }

        answers.push(answer.unwrap().text().collect::<Vec<_>>().join(""));
    }

    return Ok(answers);
}

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
        let question_text: String;

        match get_question(&fragment, i) {
            Ok(v) => question_text = v,
            Err(e) => return Err(e.to_string()),
        };

        questions.push(question_text);

        match get_answers(&fragment, i) {
            Ok(v) => v.iter().for_each(|item| questions.push(item.to_string())),
            Err(e) => return Err(e.to_string()),
        };

        questions.push("\n".to_string());
    };

    Ok(questions.join("\n").to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![generate_new_set])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
