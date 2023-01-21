use std::io::Write;

use super::Ui;

pub struct CLI;

impl Ui for CLI {
    fn select(&self, promopt: &str, choice: &[(char, &str)]) -> char {
        println!("{}", promopt);
        let allow_input: Vec<char> = choice.iter().map(|v| v.0.to_ascii_uppercase()).collect();

        loop {
            for (key, desc) in choice {
                let key = key.to_ascii_uppercase();
                print!("[{}] {}  ", key, desc);
            }
            std::io::stdout().flush().unwrap();
            let mut user_input = String::new();
            std::io::stdin().read_line(&mut user_input).unwrap();
            if let Some(input) = user_input.chars().nth(0) {
                let input = input.to_ascii_uppercase();
                if allow_input.contains(&input) {
                    return input;
                }
            }
            self.msgbox_str("Illeagal input");
        }
    }

    fn msgbox_str(&self, msg: &str) {
        println!("{}", msg);
    }

    fn choice(&self, promot: &str, default: bool) -> bool {
        loop {
            print!("{}? ", promot);
            if default {
                print!("[Y/n]: ");
            } else {
                print!("[y/N]: ");
            }
            std::io::stdout().flush().unwrap();
            let mut user_input = String::new();
            std::io::stdin().read_line(&mut user_input).unwrap();
            if user_input.is_empty() {
                return default;
            }
            if let Some(input) = user_input.chars().nth(0) {
                let input = input.to_ascii_uppercase();
                if input == 'Y' {
                    return true;
                } else if input == 'N' {
                    return false;
                }
            }
            self.msgbox_str("Illeagal input");
        }
    }
}
