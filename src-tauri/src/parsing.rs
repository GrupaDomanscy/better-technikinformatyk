use anyhow::anyhow;
use scraper::{Selector, Html, Node};

use crate::models::Question;

fn get_question(fragment: &Html, question_index: isize) -> anyhow::Result<String> {
    let question_text_selector: Selector;

    match Selector::parse(&format!("#pyt{} > .pytanietext", question_index).to_string()) {
        Ok(selector) => question_text_selector = selector,
        Err(e) => return Err(anyhow!("{}", e)),
    };

    let mut question_text: String;

    match fragment.select(&question_text_selector).next() {
        Some(v) => {
            question_text = v.children()
                .filter_map(|node| match node.value() {
                    Node::Text(text) => Some(&text[..]),
                    _ => None,
                })
                .collect::<Vec<_>>()
                .join("");
        },
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

fn get_question_code(fragment: &Html, question_index: isize) -> anyhow::Result<Option<String>> {
    let question_text_selector: Selector;

    match Selector::parse(&format!("#pyt{} > .pytanietext > code", question_index).to_string()) {
        Ok(selector) => question_text_selector = selector,
        Err(e) => return Err(anyhow!("{}", e)),
    };

    let mut question_text: String;

    match fragment.select(&question_text_selector).next() {
        Some(v) => question_text = v.inner_html(),
        None => return Ok(None)
    };

    question_text = question_text.trim()
        .to_string();

    return Ok(Some(question_text));
}

fn get_image(fragment: &Html,index: isize) -> anyhow::Result<Option<String>> {
    let img_selector: Selector;

    match Selector::parse(&format!("#pyt{} > img", index).to_string()) {
        Ok(selector) => img_selector = selector,
        Err(e) => return Err(anyhow!("{}", e)),
    };

    match fragment.select(&img_selector).next() {
        Some(v) => match v.value().attr("src") {
            Some(v) => return Ok(Some(v.to_string())),
            None => return Ok(None),
        }
        None => return Ok(None)
    };
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

pub async fn generate_new_set() -> anyhow::Result<Vec<Question>> {
    let text: String;

    match reqwest::get("https://technikinformatyk.pl/kwalifikacja-inf-03-egzamin-online/").await {
        Ok(v) => match v.text().await {
            Ok(v) => text = v,
            Err(e) => return Err(anyhow!(e)),
        },
        Err(e) => return Err(anyhow!(e)),
    };

    let fragment = Html::parse_document(text.as_str());

    let mut questions: Vec<Question> = vec![];

    for i in 1..41 {
        // code
        let code: Option<String>;

        match get_question_code(&fragment, i) {
            Ok(v) => code = v,
            Err(e) => return Err(anyhow!(e)),
        };

        // question
        let question_text: String;

        match get_question(&fragment, i) {
            Ok(v) => question_text = v,
            Err(e) => return Err(anyhow!(e)),
        };

        let mut answers: Vec<String> = vec![];

        match get_answers(&fragment, i) {
            Ok(v) => v.iter().for_each(|item| answers.push(item.to_string())),
            Err(e) => return Err(anyhow!(e)),
        };

        let image: Option<String>;

        match get_image(&fragment, i) {
            Ok(v) => image = v,
            Err(e) => return Err(anyhow!(e)),
        };

        let question = Question::new(question_text, code, image, answers);

        questions.push(question);
    };

    return Ok(questions);
}
