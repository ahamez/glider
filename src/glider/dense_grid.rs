/* --------------------------------------------------------------------------------------------- */

use super::grid::{Grid, RowCol};
use super::rle::{Rle, RleEntry};

/* --------------------------------------------------------------------------------------------- */

#[derive(Clone, Debug)]
pub struct DenseGrid {
  grid: Vec<Vec<bool>>,
  nb_lines: usize,
  nb_columns: usize,
}

/* --------------------------------------------------------------------------------------------- */

impl DenseGrid {

  pub fn new_from(g: &Vec<Vec<bool>>) -> Self {
    assert!(g.len() >= 1);
    assert!(g[0].len() >= 1);
    // TODO. Check size consistency

    let mut grid = Self::new(g.len() + 2, g[0].len() + 2);

    for row in 0 .. g.len() {
      for col in 0 .. g[0].len() {
        grid.set(RowCol{row, col}, g[row][col]);
      }
    }

    grid
  }

  pub fn new_from_rle(rle: &Rle, rows: usize, columns: usize) -> Self {
    let bounds = rle.bounds();
    let rows = usize::max(rows, bounds.0);
    let columns = usize::max(columns, bounds.1);

    let mut grid = Self::new(rows, columns);

    let row_shift = rows/2 - bounds.0/2;
    let col_shift = columns/2 - bounds.1/2;

    let mut row = row_shift;
    let mut col = col_shift;

    for entry in &rle.pattern {
      match entry {
        &RleEntry::Live(nb) => {
          for col in col .. col + nb {
            grid.set(RowCol{row, col}, true);
          }
          col = col + nb;
        }
        &RleEntry::Dead(nb) => {
          col = col + nb;
        }
        &RleEntry::NewLine  => {
          row = row + 1;
          col = col_shift;
        }
      };
    }

    grid
  }
}

/* --------------------------------------------------------------------------------------------- */

impl Grid for DenseGrid {

  fn new(nb_lines: usize, nb_columns: usize) -> Self {
    let mut line = Vec::new();
    line.resize(nb_columns + 2, false);

    let mut grid = Vec::new();
    grid.resize(nb_lines + 2, line);

    DenseGrid{grid, nb_columns, nb_lines}
  }


  fn at(&self, rc: RowCol) -> bool {
    self.grid[rc.row + 1][rc.col + 1]
  }

  fn set(&mut self, rc: RowCol, value: bool) {
    self.grid[rc.row + 1][rc.col + 1] = value;
  }

  fn count_live_neighbours(&self, rc: RowCol) -> u8 {
    debug_assert!(self.grid.len() > 2);
    debug_assert!(self.grid[0].len() > 2);

    let x = rc.row + 1;
    let y = rc.col + 1;

      self.grid[x-1][y-1] as u8
    + self.grid[x-1][y  ] as u8
    + self.grid[x-1][y+1] as u8
    + self.grid[x  ][y-1] as u8
    + self.grid[x  ][y+1] as u8
    + self.grid[x+1][y-1] as u8
    + self.grid[x+1][y  ] as u8
    + self.grid[x+1][y+1] as u8
  }

  fn nb_lines(&self) -> usize {
    self.nb_lines
  }

  fn nb_columns(&self) -> usize {
    self.nb_columns
  }

  fn count_live_cells(&self) -> u64 {
    self.grid.iter()
      .fold(0, |acc, ref col| acc + col.iter().fold(0, |acc, ref cell| acc + **cell as u64))
  }
}

/* --------------------------------------------------------------------------------------------- */
/* --------------------------------------------------------------------------------------------- */

#[test]
fn test_count_live_neighbours() {
  {
    // 1x1 universe
    let g = DenseGrid::new_from(&vec![
      vec![false],
    ]);

    assert_eq!(g.count_live_neighbours(RowCol{row: 0, col: 0}), 0);
  }
  {
    // 1x1 universe
    let g = DenseGrid::new_from(&vec![
      vec![true],
    ]);

    assert_eq!(g.count_live_neighbours(RowCol{row: 0, col: 0}), 0);
  }
  {
    // 2x2 universe
    let g = DenseGrid::new_from(&vec![
        //   0       1
      vec![true , false], // 0
      vec![false, true ], // 1
    ]);

    assert_eq!(g.count_live_neighbours(RowCol{row: 0, col: 0}), 1);
    assert_eq!(g.count_live_neighbours(RowCol{row: 0, col: 1}), 2);
    assert_eq!(g.count_live_neighbours(RowCol{row: 1, col: 0}), 2);
    assert_eq!(g.count_live_neighbours(RowCol{row: 1, col: 1}), 1);
  }
  {
    // 3x3 universe
    let g = DenseGrid::new_from(&vec![
        //   0      1      2
      vec![true , false, true ], // 0
      vec![false, true , false], // 1
      vec![false, false, false], // 2
    ]);

    assert_eq!(g.count_live_neighbours(RowCol{row: 0, col: 0}), 1);
    assert_eq!(g.count_live_neighbours(RowCol{row: 0, col: 1}), 3);
    assert_eq!(g.count_live_neighbours(RowCol{row: 0, col: 2}), 1);
    assert_eq!(g.count_live_neighbours(RowCol{row: 1, col: 0}), 2);
    assert_eq!(g.count_live_neighbours(RowCol{row: 1, col: 1}), 2);
    assert_eq!(g.count_live_neighbours(RowCol{row: 1, col: 2}), 2);
  }
}

/* --------------------------------------------------------------------------------------------- */

#[test]
fn test_new_from_rle() {

  // 3o$2bo$bo!
  let rle = Rle {
    pattern: vec![
      RleEntry::Live(3), 
      RleEntry::NewLine,
      RleEntry::Dead(2),
      RleEntry::Live(1),
      RleEntry::NewLine,
      RleEntry::Dead(1),
      RleEntry::Live(1),
    ]
  };

  let bounds = rle.bounds();
  let g = DenseGrid::new_from_rle(&rle, bounds.0, bounds.1);

  assert_eq!(g.at(RowCol{row: 0, col: 0}), true);
  assert_eq!(g.at(RowCol{row: 0, col: 1}), true);
  assert_eq!(g.at(RowCol{row: 0, col: 2}), true);

  assert_eq!(g.at(RowCol{row: 1, col: 0}), false);
  assert_eq!(g.at(RowCol{row: 1, col: 1}), false);
  assert_eq!(g.at(RowCol{row: 1, col: 2}), true);

  assert_eq!(g.at(RowCol{row: 2, col: 0}), false);
  assert_eq!(g.at(RowCol{row: 2, col: 1}), true);
  assert_eq!(g.at(RowCol{row: 2, col: 2}), false);
}

/* --------------------------------------------------------------------------------------------- */
