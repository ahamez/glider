pub mod glider;
use glider::dense_grid::DenseGrid;
use glider::rle::{Rle, RleEntry};
use glider::universe::Universe;

/* --------------------------------------------------------------------------------------------- */

fn main() {

  // Glider pattern
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
  let grid = DenseGrid::new_from_rle(&rle, 100, 100);
  let mut u = Universe::new(grid);


  u.tick();
}

/* --------------------------------------------------------------------------------------------- */
