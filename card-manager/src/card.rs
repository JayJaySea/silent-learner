use std::time::{ SystemTime, UNIX_EPOCH};

pub struct Card {
    question: String,
    answer: String,
    label: String,
    next_review: u64,
    level: u64,
}

impl Card {
    pub fn new() -> Card {
        Card {
            question: String::new(),
            answer: String::new(),
            label: String::new(),
            next_review: Card::today(),
            level: 0
        }
    }

    pub fn with_question(&mut self, question: &str) -> &mut Card {
        self.question = question.to_string();
        self
    }

    pub fn with_answer(&mut self, answer: &str) -> &mut Card {
        self.answer = answer.to_string();
        self
    }

    pub fn with_label(&mut self, label: &str) -> &mut Card {
        self.label = label.to_string();
        self
    }

    pub fn with_next_review(&mut self, next_review: u64) -> &mut Card {
        self.next_review = next_review;
        self
    }

    pub fn with_level(&mut self, level: u64) -> &mut Card {
        self.level = level;
        self
    }

    pub fn question(&self) -> &String { &self.question }

    pub fn answer(&self) -> &String { &self.answer }

    pub fn label(&self) -> &String { &self.label }

    pub fn next_review(&self) -> u64 { self.next_review }

    pub fn level(&self) -> u64 { self.level }

    fn today() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards").as_secs() / 86400
    }
}
