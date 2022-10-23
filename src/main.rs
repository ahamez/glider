use std::fs::File;
use std::io::BufReader;

mod glider;
use clap::Parser;
use glider::dense_grid::DenseGrid;
use glider::render;
use glider::rle::Rle;
use glider::universe::Universe;

#[derive(Parser)]
struct Cli {
    rle_file: Option<String>,
}

/* --------------------------------------------------------------------------------------------- */

fn main() {
    let cli = Cli::parse();

    let file = File::open(cli.rle_file.unwrap()).unwrap();
    let (rle, rule) = Rle::read(BufReader::new(file)).unwrap();

    let grid_rows = 1000;
    let grid_cols = 1000;

    let grid = DenseGrid::new_from_rle(&rle, grid_rows, grid_cols);
    let u = Universe::new(grid, rule);

    render::render_universe(u);
}

/* --------------------------------------------------------------------------------------------- */
