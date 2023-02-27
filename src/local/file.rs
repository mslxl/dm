use std::path::{Path, PathBuf};

use miette::{Context, IntoDiagnostic, Result};
use rust_i18n::t;

use crate::{
    env::{self, get_group_dir, to_depositiory_path, SpecDir},
    error::{DMError, GroupErrorKind},
    ui::Ui,
};

use super::{DMPath, ItemEntryKind, TomlItemEntry, Transaction};

fn recongize_spec_path(path: PathBuf, try_recongized: bool, ui_handle: &dyn Ui) -> Result<DMPath> {
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
        let use_pos = ui_handle.choose(
            Some(&t!("file.add.prompt_which_path")),
            matched_option.iter().map(|x| x.as_str()).collect(),
        )? - 1;

        // User choose 'None' option
        if use_pos == -1 {
            return recongize_spec_path(path, false, ui_handle);
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

async fn update_file_from_entry(
    ui_handle: &dyn Ui,
    group_name: &str,
    entry: &TomlItemEntry,
) -> Result<()> {
    if entry.manaul {
        todo!("invoke update script configuration in entry, let script update depository");
        return Ok(());
    }

    let src = entry
        .get_platform_install_path()
        .unwrap()
        .parse(&SpecDir::new()?)?;
    let dst = get_group_dir(group_name).unwrap().join(&entry.path);
    ui_handle.msg(crate::ui::MsgLevel::Info, format!("Update {:?}", dst));
    tokio::fs::create_dir_all(dst.parent().unwrap())
        .await
        .into_diagnostic()
        .wrap_err(t!("error.ctx.io.copy2depository"))?;
    tokio::fs::copy(src, dst)
        .await
        .into_diagnostic()
        .wrap_err(t!("error.ctx.io.copy2depository"))?;
    Ok(())
}

pub async fn add_file<P: AsRef<Path>>(
    ui_handle: &dyn Ui,
    path: P,
    group_name: &str,
    try_recongize: bool,
    manaul_install: bool,
) -> Result<()> {
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

    let dm_path = recongize_spec_path(path.clone(), try_recongize, ui_handle)?;

    let mut file_entry = TomlItemEntry::new(
        kind,
        to_depositiory_path(path).to_str().unwrap().to_string(),
        manaul_install,
    );
    file_entry.insert_platform_install_path(dm_path);
    update_file_from_entry(ui_handle, group_name, &file_entry)
        .await
        .wrap_err(t!("error.ctx.io.update_file"))?;

    group.files.push(file_entry);

    std::mem::drop(group);
    transaction
        .commit()
        .wrap_err(t!("error.ctx.transcation.commit"))
}
