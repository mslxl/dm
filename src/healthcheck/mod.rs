use crate::{
    cfg::{file::GroupFileConfigurationHelper, transcation::Transcation},
    env::get_depository_dir,
};

pub fn health_check(group_name: String) {
    let transcation = Transcation::new(get_depository_dir());
    let group_helper = transcation.group(group_name.clone());
    let group_helper = if let Some(group_helper) = group_helper {
        group_helper
    } else {
        eprintln!(
            "Configuration file of group {} does not exists!",
            group_name
        );
        eprintln!("This may caused by deleting depository/{}/config.toml handly. Try to restore the file to recover health", group_name);
        return;
    };
    let files = group_helper.files();
    let depository_dir = get_depository_dir();
    for f in files {
        let depository_path = depository_dir.join(f.get_depository_path().unwrap());
        if let Err(err) = depository_path.try_exists() {
            eprintln!(
                "Fail to check {} file, but it was registered in configuration file: {}",
                f.get_depository_path().unwrap(),
                err.to_string()
            );
            continue;
        }
        if let Ok(false) = depository_path.try_exists() {
            eprintln!(
                "File {} does not exists, but it was registered in configuration file",
                f.get_depository_path().unwrap()
            );
            continue;
        }
        if f.is_encrypt() && (f.is_hard_link() || f.is_soft_link()) {
            eprintln!("File {} can't registered both as symbolic link and encrypted", f.get_depository_path().unwrap());
            continue;
        }

    }
}
