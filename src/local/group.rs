use miette::{Context, Result};
use rust_i18n::t;

use crate::config;

use super::Transaction;

pub async fn create_group(name: String, nouse:bool) -> Result<()>{
    let mut transaction = Transaction::start().wrap_err(t!("error.ctx.transcation.init"))?;
    transaction.create_group(&name)?;

    if !nouse {
        let use_profile = &config::CONFIG.lock().await.using_profile;
        let pos = transaction
            .global
            .registery
            .profile
            .iter()
            .position(|entry| &entry.name == use_profile)
            .unwrap();
        transaction
            .global
            .registery
            .profile
            .get_mut(pos)
            .unwrap()
            .group
            .push(name);
    }
    transaction.commit().wrap_err(t!("error.ctx.transcation.commit"))
}

