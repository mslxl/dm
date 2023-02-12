use std::{
    collections::HashMap,
    path::{PathBuf, Path},
};

use miette::{Context, IntoDiagnostic, Result};
use rust_i18n::t;

use crate::{
    env::{self, to_depositiory_path},
    error::{DMError, GroupErrorKind},
    ui::ui,
};

use super::{DMPath, ItemEntryKind, TomlItemEntry, Transaction};


fn convert_path_format(path: PathBuf, try_recongized: bool) -> Result<DMPath> {
    let value = if try_recongized {
        let spec_dir = env::SpecDir::new()?;
        let mut matched_path =
            spec_dir.match_path(dunce::canonicalize(&path).into_diagnostic()?)?;
        matched_path.sort_by_key(|(_, p)| p.to_string_lossy().len());
        matched_path.reverse();
        let mut matched_option: Vec<String> = matched_path
            .iter()
            .map(|(name, path)| format!("{}={}", name, path.to_str().unwrap()))
            .collect();
        matched_option.insert(0, String::from("None"));
        let use_pos = ui().choose(
            Some(&t!("file.add.prompt_which_path")),
            matched_option.iter().map(|x| x.as_str()).collect(),
        )? - 1;

        // User choose 'None' option
        if use_pos == -1 {
            return convert_path_format(path, false);
        }

        let (matched_path_name, matched_path) = matched_path
            .get::<usize>(use_pos.try_into().unwrap())
            .unwrap();
        DMPath::Dynamic(vec![
            matched_path_name.clone(),
            path.strip_prefix(matched_path)
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
        ])
    } else {
        DMPath::Normal(
            dunce::canonicalize(path)
                .into_diagnostic()?
                .to_str()
                .unwrap()
                .to_string(),
        )
    };
    Ok(value)
}

pub async fn add_file<P:AsRef<Path>>(path: P, group_name: &str, try_recongize: bool, manaul_install:bool) -> Result<()>{
    let path = path.as_ref().to_path_buf();
    let mut transaction = Transaction::start().wrap_err(t!("error.ctx.transcation.init"))?;
    let mut group = transaction
        .group_mut(group_name)?
        .ok_or(DMError::GroupError {
            kind: GroupErrorKind::NotExists,
            msg: t!("error.group.not_exists", name = group_name),
            advice: None,
        })
        .into_diagnostic()?;
    if path.is_symlink() {
        todo!("throw an error")
    }

    let kind = if path.is_file() {
        ItemEntryKind::File
    } else {
        ItemEntryKind::Dir
    };

    let dm_path = convert_path_format(path.clone(), try_recongize)?;
    let mut install_path = HashMap::new();
    install_path.insert(std::env::consts::OS.to_string(), dm_path);

    let file_entry = TomlItemEntry {
        kind,
        path: to_depositiory_path(path).to_str().unwrap().to_string(),
        manaul: manaul_install,
        install: install_path,
    };
    group.files.push(file_entry);

    std::mem::drop(group);
    transaction
        .commit()
        .wrap_err(t!("error.ctx.transcation.commit"))

}


