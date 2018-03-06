/* --------------------------------------------------------------------------------------------- */

// TODO. Make this a trait
//  - random access
//  - build
//  - count_live_neighbours

#[derive(Clone)]
pub struct Grid {
  grid: Vec<Vec<bool>>,
  nb_lines: usize,
  nb_columns: usize,
}

/* --------------------------------------------------------------------------------------------- */

impl Grid {

  fn new_empty(nb_lines: usize, nb_columns: usize) -> Self {
    let mut line = Vec::new();
    line.resize(nb_columns + 2, false);

    let mut grid = Vec::new();
    grid.resize(nb_lines + 2, line);

    Grid{grid, nb_columns, nb_lines}
  }

  pub fn new_from(g: &Vec<Vec<bool>>) -> Self {
    assert!(g.len() >= 1);
    assert!(g[0].len() >= 1);
    // TODO. Check size consistency

    let mut grid = Self::new_empty(g.len() + 2, g[0].len() + 2);

    for x in 0 .. g.len() {
      for y in 0 .. g[0].len() {
        grid.set(x, y, g[x][y]);
      }
    }

    grid
  }

  pub fn at(&self, x: usize, y: usize) -> bool {
    self.grid[x+1][y+1]
  }

  pub fn set(&mut self, x: usize, y: usize, value: bool) -> &mut Self {
    self.grid[x+1][y+1] = value;
    self
  }

  pub fn count_live_neighbours(&self, x: usize, y: usize) -> u8 {
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

  pub fn nb_lines(&self) -> usize {
    self.nb_lines
  }

  pub fn nb_columns(&self) -> usize {
    self.nb_columns
  }

  pub fn count_live_cells(&self) -> u64 {
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
    let g = Grid::new_from(&vec![
      vec![false],
    ]);

    assert_eq!(g.count_live_neighbours(0, 0), 0);
  }
  {
    // 1x1 universe
    let g = Grid::new_from(&vec![
      vec![true],
    ]);

    assert_eq!(g.count_live_neighbours(0, 0), 0);
  }
  {
    // 2x2 universe
    let g = Grid::new_from(&vec![
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
    let g = Grid::new_from(&vec![
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
