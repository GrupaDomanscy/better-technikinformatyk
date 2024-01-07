#[derive(Debug)]
pub struct Question {
    question: String,
    code: Option<String>,
    image: Option<String>,
    answers: Vec<String>,
}

impl Question {
    pub fn new(question: String, code: Option<String>, image: Option<String>, answers: Vec<String>) -> Question {
        return Question{
            question,
            code,
            image,
            answers
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

    pub fn answers(&self) -> &Vec<String> {
        return &self.answers;
    }
}

