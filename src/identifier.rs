pub trait Identifier {
  fn get_id(&self) -> u64;
  fn set_id(&mut self, id: u64);
}
