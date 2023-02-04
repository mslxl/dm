use rust_i18n::t;
use miette::{Result, IntoDiagnostic};

pub mod cli;
pub trait Ui {
    fn msg(&self, level: MsgLevel, msg: String);
    fn input(&self, prompt: Option<String>) -> Result<String>;
    fn input_i32(&self, prompt: Option<String>)-> Result<i32>{
      loop{
        let text = self.input(prompt.clone())?;
        if let Ok(num)= text.parse::<i32>() {
          break Ok(num);
        }else{
          self.msg(MsgLevel::Error, t!("error.prompt.nan"))
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
