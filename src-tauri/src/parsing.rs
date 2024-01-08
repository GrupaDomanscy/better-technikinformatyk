use std::usize;

use anyhow::anyhow;
use scraper::{Selector, Html, Node};

use crate::models::{Question, Answer};

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

fn get_listpyt_attribute(fragment: &Html) -> anyhow::Result<Vec<String>> {
    let selector_str: String = "input[name=\"listpyt\"]".to_string();
    let selector: Selector;

    match Selector::parse(&selector_str) {
        Ok(v) => selector = v,
        Err(e) => return Err(anyhow!(e.to_string()))
    };

    let mut iterator = fragment.select(&selector);
    let input;

    match iterator.next() {
        Some(v) => input = v,
        None => return Err(anyhow!("Input with 'listpyt' value does not exist.")),
    };

    let attr = input.attr("value");

    if attr.is_none() {
        return Err(anyhow!("Input with 'listpyt' value does not have 'value' attribute set."));
    }

    let vec = attr.unwrap()
        .split("-")
        .filter(|item| item.len() != 0)
        .map(|item| item.to_string())
        .collect::<Vec<String>>();

    return Ok(vec)
}

fn get_answers(fragment: &Html, index: isize) -> anyhow::Result<Vec<Answer>> {
    let answers_str_selector: String = format!("#pyt{} > .odpcont", index).to_string();
    let answers_selector: Selector;

    let answers_str_input_selector: String = format!("#pyt{} > .odpcont > input", index).to_string();
    let answers_input_selector: Selector;

    match Selector::parse(&answers_str_selector) {
        Ok(v) => answers_selector = v,
        Err(e) => return Err(anyhow!(e.to_string()))
    };

    match Selector::parse(&answers_str_input_selector) {
        Ok(v) => answers_input_selector = v,
        Err(e) => return Err(anyhow!(e.to_string()))
    };

    let mut answers: Vec<Answer> = vec![];

    let mut answers_iterator = fragment.select(&answers_selector);
    let mut answer_inputs_iterator = fragment.select(&answers_input_selector);

    loop {
        let answer = answers_iterator.next();
        let answer_input = answer_inputs_iterator.next();

        if answer.is_none() {
            break;
        }

        if answer_input.is_none() {
            return Err(anyhow!("Input #pyt{}, does not have the same number of answers as inputs to click", index));
        }

        let answer_text = answer.unwrap().text().collect::<Vec<_>>().join("");
        let answer_id = answer_input.unwrap().attr("value");

        if answer_id.is_none() {
            return Err(anyhow!("Input #pyt{}, does not have a value attribute on one of its answer inputs", index));
        }

        let answer_id = answer_id.unwrap().to_string();

        answers.push(Answer::new(answer_id, answer_text));
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

    let listpyt_attr = get_listpyt_attribute(&fragment)?;

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

        let answers: Vec<Answer>;

        match get_answers(&fragment, i) {
            Ok(v) => answers = v,
            Err(e) => return Err(anyhow!(e)),
        };

        let image: Option<String>;

        match get_image(&fragment, i) {
            Ok(v) => image = v,
            Err(e) => return Err(anyhow!(e)),
        };

        let question_listpyt_attr: String;

        match listpyt_attr.get((i - 1) as usize) {
            Some(v) => question_listpyt_attr = v.clone(),
            None => return Err(anyhow!("'listpyt' attribute value has not been found for question {}", i))
        };

        let question = Question::new(question_text, code, image, answers, question_listpyt_attr);

        questions.push(question);
    };

    return Ok(questions);
}
