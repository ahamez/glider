/* --------------------------------------------------------------------------------------------- */

use super::grid::{Grid, RowCol};

/* --------------------------------------------------------------------------------------------- */

pub struct Universe<G> {
  pub generation: u64,
  pub live_cells: u64,
  pub grid: G,
}

/* --------------------------------------------------------------------------------------------- */

impl<G: Grid> Universe<G> {

  pub fn new(grid: G) -> Self {
    let live_cells = grid.count_live_cells();

    Universe{
      grid,
      generation: 0,
      live_cells,
    }
  }

  pub fn tick(&self) -> Self {

    let mut next_grid = G::new(self.grid.nb_rows(), self.grid.nb_columns());
    let mut live_cells = 0;

    for row in 0 .. self.grid.nb_rows() {
      for col in 0 .. self.grid.nb_columns() {
        if self.tick_cell(row, col) {
          live_cells += 1;
          next_grid.set(RowCol{row, col}, true);
        }
      }
    }

    Universe {
      grid: next_grid,
      generation: self.generation + 1,
      live_cells
    }
  }

  pub fn at(&self, row: usize, col: usize) -> bool {
    self.grid.at(RowCol{row, col})
  }

  fn tick_cell(&self, row: usize, col: usize) -> bool {
    match (self.grid.at(RowCol{row, col}), self.grid.count_live_neighbours(RowCol{row, col})) {
      (true , 2 ... 3) => true,
      (false, 3      ) => true,
      (_    , _      ) => false,
    }
  }
}

/* --------------------------------------------------------------------------------------------- */
/* --------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod test {

  use super::*;
  use glider::dense_grid::DenseGrid;

  #[test]
  fn test_tick() {
    let u = Universe::new(DenseGrid::new_from(&vec![
        //   0      1      2      3      4
      vec![true , false, true , false, false], // 0
      vec![false, false, true , false, false], // 1
      vec![false, false, true , false, false], // 2
      vec![false, false, false, true , true ], // 3
      vec![false, false, false, true , true ], // 4
    ]));

    // Any live cell with fewer than two live neighbours dies, as if caused by underpopulation.
    assert!(!u.tick_cell(0, 0));
    assert!(!u.tick_cell(0, 2));
    // Any live cell with two or three live neighbours lives on to the next generation.
    assert!( u.tick_cell(1, 2));
    assert!( u.tick_cell(4, 4));
    // Any live cell with more than three live neighbours dies, as if by overpopulation.
    assert!(!u.tick_cell(3, 3));

    // Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
    assert!( u.tick_cell(3, 2));
    // Other dead cells.
    assert!(!u.tick_cell(4, 0));

    // let u = Universe::new(g);
    assert_eq!(u.live_cells, 8);
    assert_eq!(u.generation, 0);

    let v = u.tick();
    assert_eq!(v.live_cells, 8); // 8 -3 (dying) +3 (spawning)
    assert_eq!(v.generation, 1);
  }

}

/* --------------------------------------------------------------------------------------------- */
