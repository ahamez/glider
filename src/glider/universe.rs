/* --------------------------------------------------------------------------------------------- */

use super::grid::Grid;

/* --------------------------------------------------------------------------------------------- */

#[derive(Debug)]
pub struct Universe {
  grid: Grid,
  generation: u64,
  live_cells: u64,
}

/* --------------------------------------------------------------------------------------------- */

impl Universe {
  
  pub fn new(grid: Grid) -> Self {
    let live_cells = grid.count_live_cells();

    Universe{
      grid,
      generation: 0,
      live_cells,
    }
  }

  pub fn tick(&self) -> Self {
    let mut next_grid = self.grid.clone();
    let mut live_cells = 0;

    for x in 0 .. self.grid.nb_lines() {
      for y in 0 .. self.grid.nb_columns() {
        if tick_cell(&self.grid, x, y) {
          live_cells += 1;
          next_grid.set(x, y, true);
        }
      }
    }
    
    Universe {
      grid: next_grid,
      generation: self.generation + 1,
      live_cells
    }
  }

  pub fn generation(&self) -> u64 {
    self.generation
  }

  pub fn live_cells(&self) -> u64 {
    self.live_cells
  }
}

/* --------------------------------------------------------------------------------------------- */

fn tick_cell(grid: &Grid, x: usize, y: usize) -> bool {
  match (grid.at(x, y), grid.count_live_neighbours(x, y)) {
    (true , 2 ... 3) => true,
    (false, 3      ) => true,
    (_    , _      ) => false,
  }
}

/* --------------------------------------------------------------------------------------------- */
/* --------------------------------------------------------------------------------------------- */

#[test]
fn test_tick() {
  let g = Grid::new_from(&vec![
      //   0      1      2      3      4  
    vec![true , false, true , false, false], // 0
    vec![false, false, true , false, false], // 1
    vec![false, false, true , false, false], // 2
    vec![false, false, false, true , true ], // 3
    vec![false, false, false, true , true ], // 4
  ]);

  // Any live cell with fewer than two live neighbours dies, as if caused by underpopulation.
  assert!(!tick_cell(&g, 0, 0));
  assert!(!tick_cell(&g, 0, 2));
  // Any live cell with two or three live neighbours lives on to the next generation.
  assert!( tick_cell(&g, 1, 2));
  assert!( tick_cell(&g, 4, 4));
  // Any live cell with more than three live neighbours dies, as if by overpopulation.
  assert!(!tick_cell(&g, 3, 3));

  // Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
  assert!( tick_cell(&g, 3, 2));
  // Other dead cells.
  assert!(!tick_cell(&g, 4, 0));

  let u = Universe::new(g);
  assert_eq!(u.live_cells, 8);
  assert_eq!(u.generation, 0);

  let v = u.tick();
  assert_eq!(v.live_cells, 8); // 8 -3 (dying) +3 (spawning)
  assert_eq!(v.generation, 1);
}
