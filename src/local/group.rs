use miette::{Context, Result};
use rust_i18n::t;

use crate::{config, ui::Ui};

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


pub async fn update_group(ui_handle: &dyn Ui,name: String) -> Result<()> {
    let transaction = Transaction::start().wrap_err(t!("error.ctx.transcation.init"))?;
    let group = transaction.group(&name)?;
    for entry in &group.files {
        if crate::local::file::check_update(entry, &group.name).await? {
            let prompt = t!("group.prompt.update_file_or_not", path = &entry.path);
            if ui_handle.input_yes_or_no(Some(&prompt), false)? {
                crate::local::file::update_file_from_entry(ui_handle, &group.name, entry).await?;
            }
        }
    }
    Ok(())
}
