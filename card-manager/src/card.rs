use std::{
    fs::{
        create_dir_all,
        File,
        OpenOptions,
    },
    time::{ 
        SystemTime, 
        UNIX_EPOCH
    },
    io::{
        Error,
        ErrorKind,
    },
    process,
};

use dirs::*;


pub type Record = (u64, u64, String, String, String);
pub struct CardManager{
    cards: Option<Vec<Card>>,
}

impl CardManager {
    const DIR: &'static str = "silent-learner";
    const PATH: &'static str = "silent-learner/cards";
    const TITLE: (&'static str, &'static str, &'static str, &'static str, &'static str) = ("NextReview", "Level", "Label", "Question", "Answer");

    pub fn new() -> CardManager {
        CardManager { 
            cards: CardManager::load_cards(),
        }
    }

    pub fn save_card(card: &Card) {
        let file = CardManager::write_cards_file();    
        let mut wtr = csv::WriterBuilder::new()
            .delimiter(b';')
            .from_writer(file);

        wtr.serialize((card.next_review(), card.level(), card.label(), card.question(), card.answer())).unwrap();
        wtr.flush().unwrap();
    }

    pub fn cards(&self) -> &Option<Vec<Card>> {
        &self.cards
    }


    pub fn save_progress(&self, marked: &Vec<Card>) {
        let path = format!("{}/{}{}", data_local_dir().unwrap().display() , CardManager::PATH, ".csv");
        let file = OpenOptions::new()
            .truncate(true)
            .write(true)
            .create(true)
            .open(path).unwrap();

        let mut wtr = csv::WriterBuilder::new()
            .delimiter(b';')
            .from_writer(file);

        wtr.serialize(CardManager::TITLE).unwrap();
        
        if let Some(cards) = &self.cards {
            for i in 0..(cards.len() - marked.len()) {
                wtr.serialize((cards[i].next_review(), cards[i].level(), cards[i].label(), cards[i].question(), cards[i].answer())).unwrap();
            }
            for card in marked {
                wtr.serialize((card.next_review(), card.level(), card.label(), card.question(), card.answer())).unwrap();
            }
        }

        wtr.flush().unwrap();
    }

    pub fn get_labels(&self) -> Vec<String> {
        //TODO: do optymalizacji
        let mut labels = Vec::new();

        for card in self.cards.as_ref().unwrap() {
            labels.push(card.label().clone());
        }

        labels.sort();
        labels.dedup();

        labels
    }

    fn load_cards() -> Option<Vec<Card>> {
        let file = match CardManager::read_cards_file() {
            Some(f) => f,
            None => return None,
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
            
            let (next_review, level, label, question, answer) = record;
            let card = Card { next_review, level, label, question, answer, };

            cards.push(card);
        }

        if cards.is_empty() {
            None
        }
        else {
            cards.sort_by(|a, b| {
                b.next_review().cmp(&a.next_review())
            });
            CardManager::make_backup(&cards);
            Some(cards)
        }
    }

    fn make_backup(cards: &Vec<Card>) {
        let path = format!("{}/{}/{}/{}{}{}", data_local_dir().unwrap().display() , CardManager::DIR, "backup", "cards", CardManager::get_today_str(), ".bak");
        let file = match OpenOptions::new()
            .truncate(true)
            .write(true)
            .create(true)
            .open(path.clone()) {
                Ok(f) => f,
                Err(e) => match e.kind() {
                    ErrorKind::NotFound => match CardManager::create_data_files() {
                        Ok(_) =>
                            OpenOptions::new()
                                .truncate(true)
                                .write(true)
                                .create(true)
                                .open(path).expect("Couldn't open file anyways"),
                        Err(e) => panic!("Can't create data directory! Error: {:?}", e),
                    },
                    other_error => panic!("Problem opening the file: {:?}", other_error),
                },
            };

        let mut wtr = csv::WriterBuilder::new()
            .delimiter(b';')
            .from_writer(file);

        wtr.serialize(CardManager::TITLE).unwrap();

        for card in cards {
            wtr.serialize((card.next_review(), card.level(), card.label(), card.question(), card.answer())).unwrap();
        }
        
        wtr.flush().unwrap();
    }

    fn get_today_str() -> String {
        match (Card::today() + 3) % 7 {
            0 => String::from("_monday"),
            1 => String::from("_tuesday"),
            2 => String::from("_wednesday"),
            3 => String::from("_thursday"),
            4 => String::from("_friday"),
            5 => String::from("_saturday"),
            6 => String::from("_sunday"),
            _ => unreachable!(),
        }
    }

    fn read_cards_file() -> Option<File> { 
        let path = format!("{}/{}{}", data_local_dir().unwrap().display() , CardManager::PATH, ".csv");
        match File::open(path) {
            Ok(f) => Some(f),
            Err(e) => match e.kind() {
                ErrorKind::NotFound => match CardManager::create_data_files() {
                    Ok(_) => None,
                    Err(e) => panic!("Can't create data directory! Error: {:?}", e),
                },
                other_error => panic!("Problem opening the file: {:?}", other_error),
            },
        }
    } 

    fn write_cards_file() -> File {
        let path = format!("{}/{}{}", data_local_dir().unwrap().display() , CardManager::PATH, ".csv");
        match OpenOptions::new()
            .append(true)
            .create(true)
            .open(path) 
            {
                Ok(f) => f,
                Err(e) => match e.kind() {
                    ErrorKind::NotFound => match CardManager::create_data_files() {
                        Ok(f) => f,
                        Err(e) => panic!("Can't create data directory! Error: {:?}", e),
                    },
                    other_error => panic!("Problem opening the file: {:?}", other_error),
                },
            }
    }

    fn create_data_files() -> Result<File, Error> {
        let path = format!("{}/{}", data_local_dir().unwrap().display() , CardManager::DIR);
        let backup_path = format!("{}/{}", path , "backup");
        create_dir_all(path)?;
        create_dir_all(backup_path)?;

        let path = format!("{}/{}{}", data_local_dir().unwrap().display() , CardManager::PATH, ".csv");

        match OpenOptions::new()
            .append(true)
            .create(true)
            .open(path.as_str()) {
                Ok(f) => {
                    let mut wtr = csv::WriterBuilder::new()
                        .delimiter(b';')
                        .from_writer(f);

                    wtr.serialize(CardManager::TITLE).unwrap();
                    wtr.flush().unwrap();

                    OpenOptions::new()
                        .append(true)
                        .create(true)
                        .open(path)
                },
                Err(e) => Err(e),
            }
    }
}

#[derive(Clone, Debug)]
pub struct Card {
    next_review: u64,
    level: u64,
    label: String,
    question: String,
    answer: String,
}

impl Card {
    pub fn new() -> Card {
        Card {
            next_review: Card::today(),
            level: 0,
            label: String::new(),
            question: String::new(),
            answer: String::new(),
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

