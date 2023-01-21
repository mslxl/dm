use crate::env::get_depository_dir;

pub fn lock()->Result<(), ()>{
  let dir = get_depository_dir();
  let lock = dir.join(".lock");
  if lock.exists() {
    Err(())
  }else{
    Ok(())
  }
}

pub fn unlock(){
  let dir = get_depository_dir();
  let lock = dir.join(".lock");
  if lock.exists() {
    std::fs::remove_file(lock).unwrap()
  }
}