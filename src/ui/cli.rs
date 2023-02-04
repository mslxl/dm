use std::io::Write;
use miette::{Result, IntoDiagnostic};

use owo_colors::OwoColorize;

use super::{MsgLevel, Ui};

pub struct Cli;
impl Ui for Cli {
    fn msg(&self, level: MsgLevel, msg: String) {
        match level {
            MsgLevel::Error => print!("[{}] ", "Error".red()),
            MsgLevel::Warn => print!("[{}] ", "Warn".yellow()),
            MsgLevel::Info => print!("[{}] ", "Info".bright_white()),
        }
        println!("{}", msg)
    }

    fn input(&self, prompt: Option<String>) -> Result<String> {
      if let Some(msg) = prompt{
        print!("{}: ", msg);
      }
      let mut input = String::new();
      std::io::stdout().flush();
      std::io::stdin().read_line(&mut input).into_diagnostic()?;
      Ok(input)
    }
}
