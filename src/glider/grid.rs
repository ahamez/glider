/* --------------------------------------------------------------------------------------------- */

#[derive(Clone, Debug)]
pub struct RowCol {
  pub row: usize,
  pub col: usize,
}

/* --------------------------------------------------------------------------------------------- */

pub trait Grid {

  fn new(nb_lines: usize, nb_columns: usize) -> Self;

  fn at(&self, rc: RowCol) -> bool;
  fn set(&mut self, rc: RowCol, value: bool);
  fn count_live_neighbours(&self, rc: RowCol) -> u8;
  fn count_live_cells(&self) -> u64;
  fn nb_lines(&self) -> usize;
  fn nb_columns(&self) -> usize;
}

/* --------------------------------------------------------------------------------------------- */
