use std::path::{PathBuf, Path};
use miette::Result;

enum Task{
  Map(Box<dyn TaskMap>),
  Reduce(Box<dyn TaskReduce>),
  Flatten(Box<dyn TaskFlatten>)
}

trait TaskMap{
  fn accept(&self, src: PathBuf, dst: PathBuf) -> Result<bool>;
}

trait TaskReduce{
  fn accept(&self, src:[PathBuf], dst: PathBuf) -> Result<bool>;
}

trait TaskFlatten{
  fn accept(&self, src: PathBuf) -> Result<(bool, Vec<PathBuf>)>;
}