use directories::{BaseDirs, UserDirs};
use miette::{Context, IntoDiagnostic, Result};
use rust_i18n::t;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::path::{Path, PathBuf};
use std::{env, fs};

use crate::error::DMError;

fn env_option_to_result<T>(result: Option<T>) -> Result<T> {
    result
        .ok_or(DMError::EnvError {
            msg: t!("error.env.dir_not_certain.msg"),
            advice: Some(t!("error.env.dir_not_certain.advice")),
        })
        .into_diagnostic()
}

pub fn get_app_data_dir() -> Result<PathBuf> {
    let path = env::var("DM_DATA")
        .ok()
        .map(|p| PathBuf::from(p))
        .or(BaseDirs::new().map(|q| q.data_local_dir().to_path_buf().join("dm")));
    if let Some(path) = &path {
        if !path.exists() {
            fs::create_dir_all(path).into_diagnostic()?;
        }
    }
    env_option_to_result(path)
}

pub fn get_app_config_file() -> Result<PathBuf> {
    let path = env::var("DM_CONFIG_FILE")
        .ok()
        .map(|p| PathBuf::from(p))
        .or(BaseDirs::new().map(|q| q.config_dir().to_path_buf().join("dm.toml")));
    env_option_to_result(path)
}

pub fn get_group_dir(group_name: &str) -> Result<PathBuf> {
    Ok(get_app_data_dir()?.join(format!("{}/{}", "depository", group_name)))
}

pub struct SpecDir {
    platform: HashMap<&'static str, PathBuf>,
    env: HashMap<String, PathBuf>,
}
pub struct SpecDirTreeDisplay<'a>(&'a SpecDir);
impl SpecDir {
    pub fn new() -> Result<Self> {
        Ok(Self {
            platform: get_platform_spec_dir().wrap_err(t!("error.env.get_platform"))?,
            env: get_env_spec_dir().wrap_err(t!("error.env.gen_env"))?,
        })
    }
    pub fn display_tree(&self) -> SpecDirTreeDisplay<'_> {
        SpecDirTreeDisplay(self)
    }

    pub fn get_path(&self, name: &str) -> Option<&Path> {
        if name.is_empty() {
            None
        } else {
            match &name.chars().nth(0).unwrap() {
                '#' => self.platform.get(&name[1..]).map(PathBuf::as_path),
                '$' => self.env.get(&name[1..]).map(PathBuf::as_path),
                _ => self
                    .platform
                    .get(name)
                    .or(self.env.get(name))
                    .map(PathBuf::as_path),
            }
        }
    }
    pub fn match_path<P: AsRef<Path>>(&self, path: P) -> Result<Vec<(String, &PathBuf)>> {
        let path = dunce::canonicalize(path).into_diagnostic()?;
        let matches = self
            .platform
            .iter()
            .map(|(name, path)| (format!("#{}", name), path))
            .chain(
                self.env
                    .iter()
                    .map(|(name, path)| (format!("${}", name), path)),
            )
            .filter(|(_, p)| path.starts_with(p))
            .collect();
        Ok(matches)
    }
}
impl Display for SpecDirTreeDisplay<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.0.platform.is_empty() {
            writeln!(f, "├─ platform")?;
            for (idx, (name, path)) in self.0.platform.iter().enumerate() {
                if idx == self.0.platform.len() - 1 {
                    write!(f, "│   └─ ")?;
                } else {
                    write!(f, "│   ├─ ")?;
                }
                writeln!(f, "{} \t{}", name, path.to_str().unwrap())?;
            }
        }
        if !self.0.env.is_empty() {
            writeln!(f, "└─ env")?;
            for (idx, (name, path)) in self.0.env.iter().enumerate() {
                if idx == self.0.env.len() - 1 {
                    write!(f, "    └─ ")?;
                } else {
                    write!(f, "    ├─ ")?;
                }
                writeln!(f, "{} \t{}", name, path.to_str().unwrap())?;
            }
        }
        Ok(())
    }
}

fn get_env_spec_dir() -> Result<HashMap<String, PathBuf>> {
    let data = std::env::vars()
        .filter(|item| fs::metadata(&item.1).is_ok())
        .map(|item| (item.0, PathBuf::from(item.1)))
        .filter(|item| item.1.is_absolute())
        .map(|item| (item.0, dunce::simplified(&item.1).to_path_buf()))
        .collect();
    Ok(data)
}

fn get_platform_spec_dir() -> Result<HashMap<&'static str, PathBuf>> {
    macro_rules! with {
        ($obj: expr, $block: expr) => {{
            let _obj = $obj;
            macro_rules! tc {
                ($invoke: ident) => {
                    Some((stringify!($invoke), _obj.$invoke().to_path_buf()))
                };
                ($invoke: ident opt) => {
                    if let Some(value) = _obj.$invoke() {
                        Some((stringify!($invoke), value.to_path_buf()))
                    } else {
                        None
                    }
                };
            }
            $block
        }};
    }
    macro_rules! push_all_opt {
        ($vec: expr, $item: expr) => {
            if let Some(v) = $item{
                $vec.insert(v);
            }
        };
        ($vec: expr, $item:expr, $($items: expr),+) => {
            push_all_opt!($vec, $item);
            push_all_opt!($vec, $($items), +)
        };
    }
    let mut data = HashSet::new();
    with!(env_option_to_result(BaseDirs::new())?, {
        push_all_opt!(
            &mut data,
            tc!(home_dir),
            tc!(cache_dir),
            tc!(config_dir),
            tc!(data_dir),
            tc!(data_local_dir),
            tc!(executable_dir opt),
            tc!(runtime_dir opt),
            tc!(preference_dir),
            tc!(state_dir opt)
        );
    });
    with!(env_option_to_result(UserDirs::new())?, {
        push_all_opt!(
            &mut data,
            tc!(home_dir),
            tc!(audio_dir opt),
            tc!(desktop_dir opt),
            tc!(document_dir opt),
            tc!(download_dir opt),
            tc!(font_dir opt),
            tc!(picture_dir opt),
            tc!(public_dir opt),
            tc!(template_dir opt),
            tc!(video_dir opt)
        );
    });

    Ok(data.into_iter().collect())
}

pub fn to_depositiory_path<P: AsRef<Path>>(path: P) -> PathBuf {
    let path = std::fs::canonicalize(path).unwrap();
    let path = path.to_str().unwrap();
    if path.starts_with("/") {
        // Unix path
        PathBuf::from("ROOT/").join(path.split_at(0).1)
    } else if path.starts_with("\\\\?\\") {
        // MSDOS path
        let filepath = &path[4..];
        let (disk, path) = filepath.split_once(":\\").unwrap();
        PathBuf::from(format!("{}\\{}", disk, path))
    } else {
        panic!("Unsupported filesystem")
    }
}
