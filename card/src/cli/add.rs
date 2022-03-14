use crate::{
    Menu,
    Choice,
};

use std::io::*;

use card_manager::card::{
    Card,
    CardManager,
};

use colored::*;

pub struct AddMenu {
    card: Card,
    card_manager: CardManager,
    options: Vec<Choice>,
    expanded: bool,
} 

impl AddMenu {
    pub fn new() -> AddMenu {
        AddMenu {
            card: Card::new(),
            card_manager:CardManager::new(),
            options: 
                vec![
                    Choice::Accept,
                    Choice::Discard,
                    Choice::SetLabel,
                    Choice::NewLabel,
                    Choice::Save,
                    Choice::Quit,
                ],
            expanded: false
        }
    }

    pub fn save_card(&mut self) {
        //TODO: Puste karty nie powinny być zapisywane
        CardManager::save_card(&self.card);
        self.expanded = false;
    }

    pub fn new_card(&mut self) {
        let label = self.card.label().clone();
        self.card = Card::new();

        self.card.with_label(label.as_str());
        self.expanded = false;
    }

    pub fn set_card_label(&mut self) {
        self.clear();
        let mut choice = 0;
        let labels = self.card_manager.get_labels();
        while choice == 0 {
            self.clear();
            println!("{}", "Choose label:".blue().bold());
            stdout().flush().unwrap();
            choice = self.random_choice(&labels, "Enter a choice: ");
        }

        self.card.with_label(labels[choice - 1].as_str());
    }


    pub fn new_card_label(&mut self) {
        self.clear();
        print!("{}", "New label: ".bold().blue());
        stdout().flush().unwrap();
        self.card.with_label(self.input().as_str());
    }

    fn input_if(&self, condition: bool) -> Option<String> {
        if condition {
            Some(self.input())
        }
        else {
            None
        }
    }

}

impl Menu for AddMenu {
    fn run(&mut self) {
        self.clear();
        let mut label = String::new();

        if !self.card.label().trim().is_empty() {
            label = format!("{}{}{}", 
                "(".bold().white(),
                self.card.label().white().bold(),
                ")".bold().white(),
                );
        }

        println!("{} {}", "Add new cards".red().bold(), label);
        print!("{}", "Question: ".blue().bold());
        stdout().flush().unwrap();

        match self.input_if(!self.expanded) {
            Some(s) => {
                self.card.with_question(s.as_str());
                ()
            },
            None => println!("{}", self.card.question()),
        };

        print!("{}", "Answer: ".blue().bold());
        stdout().flush().unwrap();

        match self.input_if(!self.expanded) {
            Some(s) => {
                self.card.with_answer(s.as_str());
                ()
            },
            None => println!("{}", self.card.answer()),
        };

        self.expanded = true;

        stdout().flush().unwrap();
    }

    fn options(&self) -> &Vec<Choice> {
        return &self.options;
    }
}
