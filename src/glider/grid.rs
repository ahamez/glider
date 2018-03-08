/* --------------------------------------------------------------------------------------------- */

#[derive(Clone, Debug)]
pub struct RowCol {
  pub row: usize,
  pub col: usize,
}

/* --------------------------------------------------------------------------------------------- */

pub trait Grid {
  fn at(&self, rc: RowCol) -> bool;
  fn set(&mut self, rc: RowCol, value: bool);
  fn count_live_neighbours(&self, rc: RowCol) -> u8;
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
