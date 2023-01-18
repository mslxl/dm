use std::{io::Write, path::PathBuf};

pub fn cli_question(promopt: &str, choice: &[(char, &str)]) -> char {
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
                return input
            }
        }
        println!("Illeagal input")
    }
}
