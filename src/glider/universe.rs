use super::grid::{make_empty, Grid};

/* --------------------------------------------------------------------------------------------- */

pub struct Universe {
  grid: Grid,
  width: usize,
  height: usize,
}

/* --------------------------------------------------------------------------------------------- */

impl Universe {
  
  pub fn new(width: usize, height: usize) -> Self {
    Universe{
      grid: make_empty(width, height),
      width,
      height,
    }
  }

  pub fn tick(&self) -> Self {
    let mut next_grid = self.grid.clone();

    for x in 1 .. self.width - 1 {
      for y in 1 .. self.height - 1 {
        next_grid[x][y] = tick_cell(&self.grid, x, y);
      }
    }
    
    Universe {
      grid: next_grid,
      .. *self
    }
  }
}

/* --------------------------------------------------------------------------------------------- */

fn tick_cell(grid: &Grid, x: usize, y: usize) -> bool {
  match (grid[x][y], count_live_neighbours(grid, x, y)) {
    (true , 2 ... 3) => true,
    (false, 3      ) => true,
    (_    , _      ) => false,
  }
}

/* --------------------------------------------------------------------------------------------- */

fn count_live_neighbours(grid: &Grid, x: usize, y: usize) -> u8 {
  debug_assert!(grid.len() > 2);
  debug_assert!(grid[0].len() > 2);

    grid[x-1][y-1] as u8
  + grid[x-1][y  ] as u8
  + grid[x-1][y+1] as u8
  + grid[x  ][y-1] as u8
  + grid[x  ][y+1] as u8
  + grid[x+1][y-1] as u8
  + grid[x+1][y  ] as u8
  + grid[x+1][y+1] as u8
}

/* --------------------------------------------------------------------------------------------- */
/* --------------------------------------------------------------------------------------------- */

#[test]
fn test_count_live_neighbours() {
  {
    // 1x1 universe
    let g = vec![
      vec![false, false, false],
      vec![false, false, false],
      vec![false, false, false],
    ];

    assert_eq!(count_live_neighbours(&g, 1, 1), 0);
  }
  {
    // 1x1 universe
    let g = vec![
      vec![false, false, false],
      vec![false, true, false],
      vec![false, false, false],
    ];

    assert_eq!(count_live_neighbours(&g, 1, 1), 0);
  }
  {
    // 2x2 universe
    let g = vec![
      vec![false, false, false, false],
      vec![false, true , false, false],
      vec![false, false, true , false],
      vec![false, false, false, false],
    ];

    assert_eq!(count_live_neighbours(&g, 1, 1), 1);
    assert_eq!(count_live_neighbours(&g, 1, 2), 2);
    assert_eq!(count_live_neighbours(&g, 2, 1), 2);
    assert_eq!(count_live_neighbours(&g, 2, 2), 1);
  }
  {
    // 2x3 universe
    let g = vec![
        //   0      1      2      3      4
      vec![false, false, false, false, false], // 0
      vec![false, true , false, true , false], // 1
      vec![false, false, true , false, false], // 2
      vec![false, false, false, false, false], // 3
    ];

    assert_eq!(count_live_neighbours(&g, 1, 1), 1);
    assert_eq!(count_live_neighbours(&g, 1, 2), 3);
    assert_eq!(count_live_neighbours(&g, 1, 3), 1);
    assert_eq!(count_live_neighbours(&g, 2, 1), 2);
    assert_eq!(count_live_neighbours(&g, 2, 2), 2);
    assert_eq!(count_live_neighbours(&g, 2, 3), 2);
  }
}

/* --------------------------------------------------------------------------------------------- */

#[test]
fn test_tick_cell() {
  let g = vec![
      //   0      1      2      3      4      5      6
    vec![false, false, false, false, false, false, false], // 0
    vec![false, true , false, true , false, false, false], // 1
    vec![false, false, false, true , false, false, false], // 2
    vec![false, false, false, true , false, false, false], // 3
    vec![false, false, false, false, true , true , false], // 4
    vec![false, false, false, false, true , true , false], // 5
    vec![false, false, false, false, false, false, false], // 6
  ];

  // Any live cell with fewer than two live neighbours dies, as if caused by underpopulation.
  assert!(!tick_cell(&g, 1, 1));
  assert!(!tick_cell(&g, 1, 3));
  // Any live cell with two or three live neighbours lives on to the next generation.
  assert!( tick_cell(&g, 2, 3));
  assert!( tick_cell(&g, 5, 5));
  // Any live cell with more than three live neighbours dies, as if by overpopulation.
  assert!(!tick_cell(&g, 4, 4));

  // Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
  assert!( tick_cell(&g, 4, 3));
  // Other dead cells.
  assert!(!tick_cell(&g, 5, 1));
}
