use miette::{IntoDiagnostic, Result};
use rust_i18n::t;

pub mod cli;
pub trait Ui {
    fn msg(&self, level: MsgLevel, msg: String);
    fn input(&self, prompt: Option<String>) -> Result<String>;
    fn input_i32(&self, prompt: Option<String>) -> Result<i32> {
        loop {
            let text = self.input(prompt.clone())?;
            if let Ok(num) = text.parse::<i32>() {
                break Ok(num);
            } else {
                self.msg(MsgLevel::Error, t!("error.prompt.nan"))
            }
        }
    }
    fn input_yes_or_no(&self, prompt: Option<String>, default: bool) -> Result<bool> {
        let addition = if default { "Y/n" } else { "y/N" };
        let prompt = prompt.map(|v| format!("{} [{}]", v, addition));
        loop {
            let input = self.input(prompt.clone())?.to_uppercase();
            let text = input.trim();
            if text.is_empty() {
                break Ok(default);
            }
            match text.chars().nth(0).unwrap() {
                'Y' => break Ok(true),
                'N' => break Ok(false),
                _ => self.msg(MsgLevel::Error, t!("error.prompt.not_bool")),
            }
        }
    }
}

pub fn ui() -> Box<dyn Ui> {
    Box::new(cli::Cli)
}

pub enum MsgLevel {
    Error,
    Warn,
    Info,
}
