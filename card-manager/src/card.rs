use std::{
    ffi::OsString,
    error::Error,
    fs::{
        File,
        OpenOptions,
    },
    time::{ 
        SystemTime, 
        UNIX_EPOCH
    },
    env::*,
    process,
};

use csv::{
    Reader,
    Writer,
};



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

pub enum Mode {
    Review,
    Add,
}

type Record = (String, String, Option<u64>, f64, f64);

pub struct CardManager{
    cards: Option<Vec<Record>>,
}

impl CardManager {
    const PATH: &'static str = "../../data/cards.csv";

    pub fn new(mode: Mode) -> CardManager {
        match mode {
            Mode::Review => 
                CardManager { 
                    cards: CardManager::load_cards(),
                },

            Mode::Add => 
                CardManager { 
                    cards: None,
                },
        }
    }


    fn load_cards() -> Option<Vec<Record>> {
        println!("{}", CardManager::PATH);
        let file = match File::open(CardManager::PATH) {
            Ok(f) => f,
            Err(_) => todo!(),
        };

        let mut cards = Vec::new();

        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b';')
            .from_reader(file);


        for result in rdr.deserialize() {
            let record: Record = match result {
                Ok(record) => record,
                Err(err) => {
                    println!("Error reading CSV from <stdin>: {}", err);
                    process::exit(1);
                },
            };
            
            cards.push(record);
        }

        if cards.is_empty() {
            None
        }
        else {
            Some(cards)
        }
    }
    
    pub fn save_card(card: &Card) {
        let path = format!("{}/{}", current_exe().unwrap().as_path().parent().unwrap().display(), CardManager::PATH);
        //panic!("{}", path);
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)
            .unwrap();


        let mut wtr = csv::WriterBuilder::new()
            .delimiter(b';')
            .from_writer(file);

        wtr.serialize((card.next_review(), card.level(), card.label(), card.question(), card.answer())).unwrap();
        wtr.flush().unwrap();
    }
}
