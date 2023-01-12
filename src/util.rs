use std::{io::Write, path::PathBuf};

pub fn rel_to_depositiory_path(group_name: &str, path: PathBuf) -> PathBuf {
    let mut path = path.canonicalize().unwrap();
    if path.is_relative() {
        path = path.canonicalize().unwrap();
    }
    let path = path.to_str().unwrap();
    if path.starts_with("/") {
        // Unix path
        PathBuf::from(format!("depository/{}/ROOT/", group_name)).join(path.split_at(0).1)
    } else if path.starts_with("\\\\?\\") {
        // MSDOS path
        let filepath = &path[4..];
        let (disk, path) = filepath.split_once(":\\").unwrap();
        PathBuf::from(format!("depository\\{}\\{}\\{}", group_name, disk, path))
    } else {
        todo!("Unsupported filesystem")
    }
}

pub fn question(promopt: &str, choice: &[(char, &str)]) -> char {
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
