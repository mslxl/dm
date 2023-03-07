use async_trait::async_trait;
use miette::{IntoDiagnostic, Result};
use std::path::PathBuf;
use tokio::{
    fs::File,
    io::{AsyncReadExt, BufReader},
};

use super::TomlItemEntry;

async fn backup_file(path: &PathBuf) -> Result<()> {
    let mut bak_path = path.clone();
    bak_path.set_extension(
        path.extension()
            .map(|x| x.to_str().unwrap())
            .unwrap_or("")
            .to_string()
            + ".bak",
    );
    if bak_path.exists() {
        tokio::fs::remove_file(&bak_path).await.into_diagnostic()?;
    }
    tokio::fs::rename(path, bak_path).await.into_diagnostic()?;
    Ok(())
}

#[async_trait]
pub trait Updater {
    async fn is_diff(
        &mut self,
        entry: &TomlItemEntry,
        src: &PathBuf,
        dst: &PathBuf,
    ) -> Result<bool>;
    async fn update(&mut self, entry: &TomlItemEntry, src: &PathBuf, dst: &PathBuf) -> Result<()>;
    async fn install(&mut self, entry: &TomlItemEntry, src: &PathBuf, dst: &PathBuf) -> Result<()>;
}

struct ManualUpdater;

#[async_trait]
impl Updater for ManualUpdater {
    async fn is_diff(&mut self, _: &TomlItemEntry, src: &PathBuf, dst: &PathBuf) -> Result<bool> {
        todo!()
    }

    async fn update(&mut self, _: &TomlItemEntry, src: &PathBuf, dst: &PathBuf) -> Result<()> {
        todo!()
    }
    async fn install(&mut self, _: &TomlItemEntry, src: &PathBuf, dst: &PathBuf) -> Result<()> {
        todo!()
    }
}
struct NormalUpdater;

#[async_trait]
impl Updater for NormalUpdater {
    /// 逐位比较文件
    async fn is_diff(&mut self, _: &TomlItemEntry, src: &PathBuf, dst: &PathBuf) -> Result<bool> {
        let src = File::open(src).await.into_diagnostic()?;
        let dst = File::open(dst).await.into_diagnostic()?;
        if src.metadata().await.into_diagnostic()?.len()
            != dst.metadata().await.into_diagnostic()?.len()
        {
            return Ok(true);
        }
        let mut reader1 = BufReader::new(src);
        let mut reader2 = BufReader::new(dst);

        let mut buf1 = [0; 1024];
        let mut buf2 = [0; 1024];
        loop {
            let n1 = reader1.read(&mut buf1).await.into_diagnostic()?;
            let n2 = reader2.read(&mut buf2).await.into_diagnostic()?;
            if n1 != n2 {
                break Ok(true);
            }
            if n1 == 0 {
                break Ok(false);
            }
            if buf1 != buf2 {
                break Ok(true);
            }
        }
    }

    async fn update(&mut self, _: &TomlItemEntry, src: &PathBuf, dst: &PathBuf) -> Result<()> {
        tokio::fs::create_dir_all(dst.parent().unwrap())
            .await
            .into_diagnostic()?;
        if dst.exists() {
            backup_file(dst).await?;
        }
        tokio::fs::copy(src, dst).await.into_diagnostic()?;
        Ok(())
    }
    async fn install(&mut self, _: &TomlItemEntry, src: &PathBuf, dst: &PathBuf) -> Result<()> {
        let parent_dir = dst.parent().unwrap();
        if !parent_dir.exists() {
            tokio::fs::create_dir_all(parent_dir)
                .await
                .into_diagnostic()?;
        }

        if dst.exists() {
            backup_file(dst).await?;
        }
        tokio::fs::copy(src, dst).await.into_diagnostic()?;

        Ok(())
    }
}

pub fn construct_updater(entry: &TomlItemEntry) -> Result<Box<dyn Updater>> {
    if entry.manaul {
        Ok(Box::new(ManualUpdater))
    } else {
        Ok(Box::new(NormalUpdater))
    }
}
