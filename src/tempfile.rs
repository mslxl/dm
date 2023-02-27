use miette::{Context, IntoDiagnostic, Result};
use rust_i18n::t;
use std::fs::File;
use std::path::{PathBuf, Path};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Tempfile {
    path: PathBuf,
}

impl Tempfile {
    pub fn new() -> Result<Self> {
        let tmpdir = std::env::temp_dir();
        let mut filename = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .to_string();
        filename.push_str(".dm.tmp");
        let p = tmpdir.join(filename);
        File::create(&p)
            .into_diagnostic()
            .wrap_err(t!("error.ctx.io.temp"))?;
        Ok(Self{
          path: p
        })
    }

    pub fn get_path_buf(&self) -> &PathBuf {
      &self.path
    }
}

impl AsRef<Path> for Tempfile {
    fn as_ref(&self) -> &Path {
      &self.path
    }
}

impl Drop for Tempfile {
    fn drop(&mut self) {
      if self.path.exists() {
        std::fs::remove_file(&self.path);
      }
    }
}
