use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Answer {
    id: String,
    label: String,
}

impl Answer {
    pub fn new(id: String, label: String) -> Self {
        return Self{ id, label };
    }

    pub fn id(&self) -> &String {
        return &self.id;
    }

    pub fn label(&self) -> &String {
        return &self.label;
    }
}

#[derive(Debug, Serialize)]
pub struct Question {
    question: String,
    code: Option<String>,
    image: Option<String>,
    answers: Vec<Answer>,

    listpyt_attr: String,
}

impl Question {
    pub fn new(question: String, code: Option<String>, image: Option<String>, answers: Vec<Answer>, listpyt_attr: String) -> Question {
        return Question{
            question,
            code,
            image,
            answers,

            listpyt_attr
        };
    }

    pub fn question(&self) -> &String {
        return &self.question;
    }

    pub fn code(&self) -> &Option<String> {
        return &self.code;
    }

    pub fn image(&self) -> &Option<String> {
        return &self.image;
    }

    pub fn answers(&self) -> &Vec<Answer> {
        return &self.answers;
    }
}

impl Clone for Question {
    fn clone(&self) -> Self {
        return Self{
            question: self.question.clone(),
            code: self.code.clone(),
            image: self.image.clone(),
            answers: self.answers.clone(),
            listpyt_attr: self.listpyt_attr.clone(),
        };
    }
}

