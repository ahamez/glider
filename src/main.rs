pub mod glider;
use glider::grid::Grid;
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
    ],

    nb_lines: 3,
    nb_columns: 3,
  };
  println!("{:?}", rle);

  let grid = Grid::new_from_rle(&rle);
  println!("{:?}", grid);

  let u = Universe::new(grid);
  println!("{:?}", u);
}

/* --------------------------------------------------------------------------------------------- */
