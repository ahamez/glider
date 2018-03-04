/* --------------------------------------------------------------------------------------------- */

pub type Grid = Vec<Vec<bool>>;

/* --------------------------------------------------------------------------------------------- */

pub fn make_empty(width: usize, height: usize) -> Vec<Vec<bool>> {
  let mut grid = Vec::with_capacity(width + 2);

  for _x in 0..width {
    let mut line = Vec::with_capacity(height + 2);
    for _y in 0..height {
      line.push(false);
    }
    grid.push(line);
  }

  grid
}

/* --------------------------------------------------------------------------------------------- */
