/* --------------------------------------------------------------------------------------------- */

pub trait Grid {
  fn at(&self, x: usize, y: usize) -> bool;
  fn set(&mut self, x: usize, y: usize, value: bool);
  fn count_live_neighbours(&self, x: usize, y: usize) -> u8;
  fn count_live_cells(&self) -> u64;
  fn nb_lines(&self) -> usize;
  fn nb_columns(&self) -> usize;

  fn box_clone(&self) -> Box<Grid>;
}

/* --------------------------------------------------------------------------------------------- */

impl Clone for Box<Grid> {
  fn clone(&self) -> Box<Grid> {
    self.box_clone()
  }
}

/* --------------------------------------------------------------------------------------------- */
