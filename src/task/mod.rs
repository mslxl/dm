enum Task{
  Map(Box<dyn TaskMap>),
  Reduce(Box<dyn TaskReduce>)
}

trait TaskMap{
  fn accept(&self, data: &[u8]) -> Vec<u8>;
}

trait TaskReduce{
  fn accept(&self, data: Vec<&[u8]>) -> Vec<u8>;
}