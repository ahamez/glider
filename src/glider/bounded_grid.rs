/* --------------------------------------------------------------------------------------------- */

use super::grid::Grid;
use super::rle::{Rle, RleEntry};

/* --------------------------------------------------------------------------------------------- */

#[derive(Clone, Debug)]
pub struct BoundedGrid {
  grid: Vec<Vec<bool>>,
  nb_lines: usize,
  nb_columns: usize,
}

/* --------------------------------------------------------------------------------------------- */

impl BoundedGrid {

  fn new(nb_lines: usize, nb_columns: usize) -> Self {
    let mut line = Vec::new();
    line.resize(nb_columns + 2, false);

    let mut grid = Vec::new();
    grid.resize(nb_lines + 2, line);

    BoundedGrid{grid, nb_columns, nb_lines}
  }

  pub fn new_from(g: &Vec<Vec<bool>>) -> Self {
    assert!(g.len() >= 1);
    assert!(g[0].len() >= 1);
    // TODO. Check size consistency

    let mut grid = Self::new(g.len() + 2, g[0].len() + 2);

    for x in 0 .. g.len() {
      for y in 0 .. g[0].len() {
        grid.set(x, y, g[x][y]);
      }
    }

    grid
  }

  pub fn new_from_rle(rle: &Rle) -> Self {
    let mut grid = Self::new(rle.nb_lines, rle.nb_columns);

    let mut x = 0;
    let mut y = 0;

    for entry in &rle.pattern {
      match entry {
        &RleEntry::Live(nb) => {
          for ypos in y .. y + nb {
            grid.set(x, ypos, true);
          }
          y = y + nb;
        }
        &RleEntry::Dead(nb) => {
          y = y + nb;
        }
        &RleEntry::NewLine  => {
          x = x + 1;
          y = 0;
        }
      };
    }

    grid
  }
}

/* --------------------------------------------------------------------------------------------- */

impl Grid for BoundedGrid {

  fn at(&self, x: usize, y: usize) -> bool {
    self.grid[x+1][y+1]
  }

  fn set(&mut self, x: usize, y: usize, value: bool) {
    self.grid[x+1][y+1] = value;
  }

  fn count_live_neighbours(&self, x: usize, y: usize) -> u8 {
    debug_assert!(self.grid.len() > 2);
    debug_assert!(self.grid[0].len() > 2);

    let x = x + 1;
    let y = y + 1;

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

  fn box_clone(&self) -> Box<Grid> {
    Box::new(self.clone())
  }
}

/* --------------------------------------------------------------------------------------------- */
/* --------------------------------------------------------------------------------------------- */

#[test]
fn test_count_live_neighbours() {
  {
    // 1x1 universe
    let g = BoundedGrid::new_from(&vec![
      vec![false],
    ]);

    assert_eq!(g.count_live_neighbours(0, 0), 0);
  }
  {
    // 1x1 universe
    let g = BoundedGrid::new_from(&vec![
      vec![true],
    ]);

    assert_eq!(g.count_live_neighbours(0, 0), 0);
  }
  {
    // 2x2 universe
    let g = BoundedGrid::new_from(&vec![
        //   0       1
      vec![true , false], // 0
      vec![false, true ], // 1
    ]);

    assert_eq!(g.count_live_neighbours(0, 0), 1);
    assert_eq!(g.count_live_neighbours(0, 1), 2);
    assert_eq!(g.count_live_neighbours(1, 0), 2);
    assert_eq!(g.count_live_neighbours(1, 1), 1);
  }
  {
    // 3x3 universe
    let g = BoundedGrid::new_from(&vec![
        //   0      1      2
      vec![true , false, true ], // 0
      vec![false, true , false], // 1
      vec![false, false, false], // 2
    ]);

    assert_eq!(g.count_live_neighbours(0, 0), 1);
    assert_eq!(g.count_live_neighbours(0, 1), 3);
    assert_eq!(g.count_live_neighbours(0, 2), 1);
    assert_eq!(g.count_live_neighbours(1, 0), 2);
    assert_eq!(g.count_live_neighbours(1, 1), 2);
    assert_eq!(g.count_live_neighbours(1, 2), 2);
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
    ],

    nb_lines: 3,
    nb_columns: 3,
  };

  let g = BoundedGrid::new_from_rle(&rle);

  assert_eq!(g.at(0, 0), true);
  assert_eq!(g.at(0, 1), true);
  assert_eq!(g.at(0, 2), true);

  assert_eq!(g.at(1, 0), false);
  assert_eq!(g.at(1, 1), false);
  assert_eq!(g.at(1, 2), true);

  assert_eq!(g.at(2, 0), false);
  assert_eq!(g.at(2, 1), true);
  assert_eq!(g.at(2, 2), false);
}

/* --------------------------------------------------------------------------------------------- */
