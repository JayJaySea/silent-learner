use crate::{
    Menu,
    Choice,
};

use std::io::*;

use card_manager::card::{
    Card,
    CardManager,
    Mode,
};

use colored::*;

pub struct AddMenu {
    card: Card,
    options: Vec<Choice>,
} 

impl Menu for AddMenu {
    fn run(&mut self) {
        AddMenu::clear();

        println!("{}", "CARD CREATOR".red().bold());
        print!("{}", "Question: ".blue().bold());
        stdout().flush().unwrap();

        match self.input_if_empty(self.card.question().is_empty()) {
            Some(s) => {
                self.card.with_question(s.as_str());
                ()
            },
            None => println!("{}", self.card.question()),
        };

        print!("{}", "Answer: ".blue().bold());
        stdout().flush().unwrap();

        match self.input_if_empty(self.card.answer().is_empty()) {
            Some(s) => {
                self.card.with_answer(s.as_str());
                ()
            },
            None => println!("{}", self.card.answer()),
        };

        stdout().flush().unwrap();
    }

    fn options(&self) -> &Vec<Choice> {
        return &self.options;
    }
}

impl AddMenu {
    pub fn new() -> AddMenu {
        AddMenu {
            card: Card::new(),
            options: 
                vec![
                    Choice::Accept,
                    Choice::Discard,
                    Choice::Save,
                    Choice::Quit,
                ]
        }
    }

    pub fn save_card(&self) {
        CardManager::save_card(&self.card);
    }

    pub fn new_card(&mut self) {
        self.card = Card::new();
    }

    fn input_if_empty(&self, empty: bool) -> Option<String> {
        if empty {
            Some(self.input())
        }
        else {
            None
        }
    }

}
