use anyhow::anyhow;
use scraper::{Selector, Html};

use crate::{ParserQuestionsState, models::Question};

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
pub async fn generate_new_set<'a>(state: tauri::State<'a, crate::ParserQuestionsState>) -> Result<(), String> {
    let text: String;

    match reqwest::get("https://technikinformatyk.pl/kwalifikacja-inf-03-egzamin-online/").await {
        Ok(v) => match v.text().await {
            Ok(v) => text = v,
            Err(e) => return Err(e.to_string()),
        },
        Err(e) => return Err(e.to_string()),
    };

    let fragment = Html::parse_document(text.as_str());

    let mut questions: Vec<Question> = vec![];

    for i in 1..41 {
        // question
        let question_text: String;

        match get_question(&fragment, i) {
            Ok(v) => question_text = v,
            Err(e) => return Err(e.to_string()),
        };

        let mut answers: Vec<String> = vec![];

        match get_answers(&fragment, i) {
            Ok(v) => v.iter().for_each(|item| answers.push(item.to_string())),
            Err(e) => return Err(e.to_string()),
        };

        let question = Question::new(question_text, None, None, answers);

        questions.push(question);
    };

    let mut nt = state.0.lock().unwrap();
    *nt = questions;

    return Ok(());
}

#[tauri::command]
pub fn get_question_from_state(state: tauri::State<ParserQuestionsState>) -> Option<String> {
    let questions = &state.0.lock().unwrap();

    let question_obj = questions.get(0).unwrap();

    let question = question_obj.question().clone();

    return Some(question);
}

