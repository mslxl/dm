mod cli;

pub trait Ui {
    fn choice(&self,promot: &str, default: bool) -> bool;
    fn select(&self,promopt: &str, choice: &[(char, &str)]) -> char;
    fn msgbox_str(&self,msg: &str);
}

pub fn get_ui()->Box<dyn Ui>{
  Box::new(cli::CLI)
}
