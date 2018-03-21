use std::fs::File;
use std::io::BufReader;

#[macro_use]
extern crate clap;

extern crate sdl2;

mod glider;
use glider::dense_grid::DenseGrid;
use glider::rle::Rle;
use glider::render;
use glider::universe::Universe;

/* --------------------------------------------------------------------------------------------- */

fn main() {

  let matches = clap_app!(myapp =>
    (@arg RLE_FILE: +required "Sets the input file to use")
  ).get_matches();

  let file = File::open(matches.value_of("RLE_FILE").unwrap()).unwrap();
  let (rle, rule) = Rle::read(BufReader::new(file)).unwrap();

  let grid_rows = 1000;
  let grid_cols = 1000;

  let grid = DenseGrid::new_from_rle(&rle, grid_rows, grid_cols);
  let u = Universe::new(grid, rule);

  render::render_universe(u);
}

/* --------------------------------------------------------------------------------------------- */
