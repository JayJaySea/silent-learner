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
    SetLabel,
    NewLabel,
    Quit,
    Save,
}

pub enum MenuType {
    Review,
    Add,
}

pub trait Menu {
    fn run(&mut self);

    fn choice(&self, label: &str) -> Option<Choice> {
        let mut input = String::new();

        if self.options().is_empty() {
            return Some(Choice::Quit);
        }

        for choice in self.options() {
            println!("{}", self.translate_choice(choice));
        }

        print!("{}", label.blue().bold());
        stdout().flush().unwrap();

        std::io::stdin().read_line(&mut input).expect("Error: Failed to take standard input!"); 

        match input.trim() {
            "f" => Some(Choice::Forgotten),
            "r" => Some(Choice::Remembered),
            "a" => Some(Choice::Accept),
            "d" => Some(Choice::Discard),
            "l" => Some(Choice::SetLabel),
            "n" => Some(Choice::NewLabel),
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
                Choice::SetLabel => format!("{}", "l".white().bold()),
                Choice::NewLabel => format!("{}", "n".cyan().bold()),
                Choice::Save => format!("{}", "s".purple().bold()),
                Choice::Quit => format!("{}", "q".red().bold()),
            }; 

        let description = match choice { 
                Choice::Forgotten => format!("{}", "Mark as forgotten".yellow().bold()),
                Choice::Remembered => format!("{}", "Mark as remembered".green().bold()),
                Choice::Accept => format!("{}", "Accept (save) and continue".green().bold()),
                Choice::Discard => format!("{}", "Discard and continue".yellow().bold()),
                Choice::SetLabel => format!("{}", "Set label".white().bold()),
                Choice::NewLabel => format!("{}", "New label".cyan().bold()),
                Choice::Save => format!("{}", "Save and quit".purple().bold()),
                Choice::Quit => format!("{}", "Quit".red().bold()),
            }; 

        format!("{}{}{} {}", "[".blue().bold(), letter, "]".blue().bold(), description)
    }

    fn random_choice(&self, options: &Vec<String>, label: &str) -> usize {
        let mut color = 0;

        for (index, option) in options.iter().enumerate() {
            if color == 0 {
                println!("{}{}{} {}",
                         "[".blue().bold(),
                         (index + 1).to_string().as_str().yellow().bold(),
                         "]".blue().bold(),
                         option.as_str().yellow().bold());
            }
            else {
                println!("{}{}{} {}", 
                         "[".blue().bold(), 
                         (index + 1).to_string().as_str().white().bold(),
                         "]".blue().bold(),
                         option.as_str().white().bold());
            }
            color = (color+1)%2;
        }
        print!("{}", label.blue().bold());
        stdout().flush().unwrap();

        Self::one_to_(options.len())
    }

    fn one_to_(n: usize) -> usize {
        let mut line = String::new();
        let mut input:usize = 0;

        std::io::stdin().read_line(&mut line).expect("Error: Failed to take standard input!"); 

        match line.trim().parse::<usize>() {
            Ok(x) 
                if x > 0 && x <= n
                => { input = x; },

            Ok(_) | Err(_) => (),
        }

        input
    }

    fn input_if_empty(&self, empty: bool) -> Option<String> {
        if empty {
            Some(self.input())
        }
        else {
            None
        }
    }

    fn input(&self) -> String {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Error: Failed to take standard input!"); 

        input.trim().to_string()
    }

    fn clear(&self) {
        //TODO: Add windows version
        print!("\x1B[2J\x1B[1;1H");   
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
