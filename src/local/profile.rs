use miette::{Context, IntoDiagnostic, Result};
use rust_i18n::t;

use crate::{
    config,
    error::{DMError, ProfileErrorKind}, ui::Ui,
};

use super::{TomlGlobalProfileEntry, Transaction};

pub async fn create_profile(name: String) -> Result<()> {
    let mut transaction = Transaction::start().wrap_err(t!("error.ctx.transcation.init"))?;

    let profile_list = &mut transaction.global.registery.profile;
    if profile_list
        .iter()
        .find(|entry| entry.name == name)
        .is_some()
    {
        Err(DMError::ProfileError {
            kind: ProfileErrorKind::DuplicateCreate,
            msg: t!("error.profile.duplicate.msg", name = &name),
            advice: Some(t!("error.profile.duplicate.advice")),
        })?;
    }
    profile_list.push(TomlGlobalProfileEntry::new(name.clone()));
    transaction
        .commit()
        .wrap_err(t!("error.ctx.transcation.commit"))?;
    Ok(())
}
pub async fn use_profile(name: String) -> Result<()> {
    let transaction = Transaction::start().wrap_err(t!("error.ctx.transcation.init"))?;
    if transaction
        .global
        .registery
        .profile
        .iter()
        .find(|entry| entry.name == name)
        .is_some()
    {
        let mut config_guard = config::CONFIG.lock().await;
        config_guard.using_profile = name;
        config_guard.save().wrap_err(t!("error.ctx.config.save"))
    } else {
        Err(DMError::ProfileError {
            kind: ProfileErrorKind::NotExists,
            msg: t!("error.profile.not_exists.msg", name = &name),
            advice: None,
        })
        .into_diagnostic()
    }
}

pub async fn delete(ui_handle:&dyn Ui, name: String, confirm_all: bool) -> Result<()> {

    if name == "default" {
        Err(DMError::ProfileError {
            kind: ProfileErrorKind::IlleagalOperation,
            msg: t!("error.profile.delete_def.msg"),
            advice: Some(t!("error.profile.delete_def.advice")),
        })
        .into_diagnostic()?;
    }
    if config::CONFIG.lock().await.using_profile == name {
        Err(DMError::ProfileError {
            kind: ProfileErrorKind::IlleagalOperation,
            msg: t!("error.profile.delete_using.msg"),
            advice: Some(t!("error.profile.delete_using.advice")),
        })
        .into_diagnostic()?;
    }
    let mut transaction = Transaction::start().wrap_err(t!("error.ctx.transcation.init"))?;
    if let Some(idx) = transaction
        .global
        .registery
        .profile
        .iter()
        .position(|entry| entry.name == name)
    {
        if !confirm_all
            && !ui_handle.input_yes_or_no(Some(&t!("profile.delete.confirm", name = &name)), false)?
        {
            return Ok(());
        }
        transaction.global.registery.profile.remove(idx);
        transaction
            .commit()
            .wrap_err(t!("error.ctx.transcation.commit"))?;
        Ok(())
    } else {
        Err(DMError::ProfileError {
            kind: ProfileErrorKind::NotExists,
            msg: t!("error.profile.delete_not_exists.msg", name = &name),
            advice: None,
        })
        .into_diagnostic()
    }
}
