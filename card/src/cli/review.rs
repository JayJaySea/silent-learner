use crate::{
    Menu,
    Choice,
};

use std::{
    collections::VecDeque,
    io::*,
};

use card_manager::card::{
    Card,
    CardManager,
};

use colored::*;

pub struct ReviewMenu {
    card: Card,
    card_manager: CardManager,
    for_review: VecDeque<Card>,
    marked: Vec<Card>,
    options: Vec<Choice>,
    revealed: bool,
}

impl ReviewMenu {
    pub fn new() -> ReviewMenu {
        let mut menu = ReviewMenu {
            card: Card::new(),
            card_manager: CardManager::new(),
            for_review: VecDeque::new(),
            marked: Vec::new(),
            options: 
                vec![
                    Choice::Remembered,
                    Choice::Forgotten,
                    Choice::Save,
                    Choice::Quit,
                ],
            revealed: false,
        };

        menu.prepare_cards_for_review();

        menu
    }
    
    pub fn save_progress(&mut self) {
        while let Some(c) = self.for_review.pop_front() {
            self.marked.push(c);
        }

        self.card_manager.save_progress(&self.marked);
    }

    pub fn mark_card(&mut self, mark: u8) {
        if let Some(mut c) = self.for_review.pop_front() {
            c.with_next_review(self.calculate_next_review(c.level(), mark));
            c.with_level(self.calculate_new_level(c.level(), mark));

            if mark == 1 {
                self.marked.push(c);
            }
            else {
                self.for_review.push_back(c);
            }
        }

        self.revealed = false;
    }

    fn calculate_new_level(&self, level: u64, mark: u8) -> u64 {
        (level + 1)*(mark as u64)
    }

    fn calculate_next_review(&self, level: u64, mark: u8) -> u64 {
        self.card.next_review() + u64::pow(2, level as u32)*(mark as u64)
    }

    fn prepare_cards_for_review(&mut self) {
        match self.card_manager.cards() {
            Some(cards) => {
                for card in cards {
                    if card.next_review() <= self.card.next_review() {
                        self.for_review.push_back(card.clone());
                    }
                }
            }

            None => todo!("Handle no cards to review"),
        }
    }
}

impl Menu for ReviewMenu {
    fn run(&mut self) {
        ReviewMenu::clear();
        println!("{}", "Review session".red().bold());
        let card = match self.for_review.front() {
            Some(c) => c,
            None => {
                println!("{}","No more cards to review for today. Good job!".green().bold());
                self.save_progress();
                self.options = Vec::new();
                return;
            },
        };

        if !self.revealed {
            self.revealed = true;
            println!("{} {}", "Question:".blue().bold(), card.question());
            println!("{}", "Answer:".blue().bold());
            print!("{}", "Press enter to reveal answer ".yellow().bold());
            stdout().flush().unwrap();
            self.input();
        }

        ReviewMenu::clear();
        println!("{}", "Review session".red().bold());
        println!("{} {}", "Question:".blue().bold(), card.question());
        println!("{} {}", "Answer:".blue().bold(), card.answer());
        stdout().flush().unwrap();
    }

    fn options(&self) -> &Vec<Choice> {
        return &self.options;
    }
}
