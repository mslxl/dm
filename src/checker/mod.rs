use crate::{cfg::file::GroupFileConfigurationHelper, error::Error};

pub fn check_configuration(config: &dyn GroupFileConfigurationHelper) -> Option<Error> {
    if config.is_encrypt() {
        if config.is_hard_link() {
            return Some(
                Error::err(String::from("Could not encrypt with hard-link"))
                    .suggest(String::from("Disable encrypt or hand-link")),
            );
        }
        if config.is_soft_link() {
            return Some(
                Error::err(String::from("Could not encrypt with soft-link"))
                    .suggest(String::from("Disable encrypt or soft-link")),
            );
        }
    }

    None
}
