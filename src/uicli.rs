use dm::ui::{Ui, MsgLevel};
use miette::{IntoDiagnostic, Result};
use rust_i18n::t;
use std::io::Write;

use owo_colors::OwoColorize;

pub struct Cli;
impl Ui for Cli {
    fn msg(&self, level: MsgLevel, msg: String) {
        match level {
            MsgLevel::Error => print!("{} ", "[Error]".red()),
            MsgLevel::Warn => print!("{} ", "[Warn]".yellow()),
            MsgLevel::Info => print!("{} ", "[Info]".bright_white()),
        }
        println!("{}", msg)
    }

    fn input(&self, prompt: Option<&str>) -> Result<String> {
        if let Some(msg) = prompt {
            print!("{}: ", msg);
        }
        let mut input = String::new();
        std::io::stdout().flush().into_diagnostic()?;
        std::io::stdin().read_line(&mut input).into_diagnostic()?;
        Ok(input)
    }

    fn choose(&self, prompt: Option<&str>, item: Vec<&str>) -> Result<i32> {
        loop {
            if item.len() > 10 {
                for (idx, select) in item.iter().enumerate() {
                    print!("[{:2}]: {}\t", idx.bright_white(), select);
                }
            } else {
                for (idx, select) in item.iter().enumerate() {
                    println!("{:2}. {}", idx.bright_white(), select);
                }
            }
            let pos = self.input_i32(prompt)?;
            if pos >= 0 && pos < item.len().try_into().unwrap() {
                break Ok(pos);
            } else {
                self.msg(
                    MsgLevel::Error,
                    t!("error.prompt.missing_choose", pos = &pos.to_string()),
                )
            }
        }
    }
}
