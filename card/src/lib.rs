use colored::*;
use std::io::*;

pub mod cli {
    pub mod add;
    pub mod review;
}

pub enum Choice {
    Forgotten,
    Remembered,
    Accept,
    Discard,
    Quit,
    Save,
}

pub trait Menu {
    fn run(&mut self);


    fn choice(&self) -> Option<Choice> {
        let mut input = String::new();

        for choice in self.options() {
            println!("{}", self.translate_choice(choice));
        }

        print!("{}", "Enter a choice: ".blue().bold());
        stdout().flush().unwrap();

        std::io::stdin().read_line(&mut input).expect("Error: Failed to take standard input!"); 

        match input.trim() {
            "f" => Some(Choice::Forgotten),
            "r" => Some(Choice::Remembered),
            "a" => Some(Choice::Accept),
            "d" => Some(Choice::Discard),
            "q" => Some(Choice::Quit),
            "s" => Some(Choice::Save),
            _ => None,
        }
    }

    fn options(&self) -> &Vec<Choice>;

    fn translate_choice(&self, choice: &Choice) -> String {
        let letter = match choice {
                Choice::Forgotten => format!("{}", "f".yellow().bold()),
                Choice::Remembered => format!("{}", "r".green().bold()),
                Choice::Accept => format!("{}", "a".green().bold()),
                Choice::Discard => format!("{}", "d".yellow().bold()),
                Choice::Save => format!("{}", "s".purple().bold()),
                Choice::Quit => format!("{}", "q".red().bold()),
            }; 

        let description = match choice { Choice::Forgotten => format!("{}", "Card forgotten".yellow()),
                Choice::Remembered => format!("{}", "Card remembered".green().bold()),
                Choice::Accept => format!("{}", "Accept (save) card".green().bold()),
                Choice::Discard => format!("{}", "Discard card".yellow().bold()),
                Choice::Save => format!("{}", "Save and quit".purple().bold()),
                Choice::Quit => format!("{}", "Discard and quit".red().bold()),
            }; 

        format!("{}{}{} {}", "[".blue().bold(), letter, "]".blue().bold(), description)
    }

    fn input(&self) -> String {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Error: Failed to take standard input!"); 

        input.trim().to_string()
    }

    fn clear() {
        print!("\x1B[2J\x1B[1;1H");   
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
