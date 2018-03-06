/* --------------------------------------------------------------------------------------------- */

#[derive(Debug)]
pub enum RleEntry {
  Live(usize),
  Dead(usize),
  NewLine,
}

#[derive(Debug)]
pub struct Rle {
  pub pattern: Vec<RleEntry>,
  pub nb_lines: usize,
  pub nb_columns: usize,
}

/* --------------------------------------------------------------------------------------------- */
