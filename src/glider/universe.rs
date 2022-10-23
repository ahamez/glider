/* --------------------------------------------------------------------------------------------- */

use super::grid::{Grid, RowCol};
use super::rule::Rule;

/* --------------------------------------------------------------------------------------------- */

pub struct Universe<G> {
    pub generation: u64,
    pub live_cells: u64,
    pub grid: G,
    pub rule: Rule,
}

/* --------------------------------------------------------------------------------------------- */

impl<G: Grid> Universe<G> {
    pub fn new(grid: G, rule: Rule) -> Self {
        let live_cells = grid.count_live_cells();

        Universe {
            generation: 0,
            live_cells,
            grid,
            rule,
        }
    }

    pub fn tick(&self) -> Self {
        let mut next_grid = G::new(self.grid.nb_rows(), self.grid.nb_columns());
        let mut live_cells = 0;

        for row in 0..self.grid.nb_rows() {
            for col in 0..self.grid.nb_columns() {
                if self.tick_cell(row, col) {
                    live_cells += 1;
                    next_grid.set(RowCol { row, col }, true);
                }
            }
        }

        Universe {
            generation: self.generation + 1,
            live_cells,
            grid: next_grid,
            rule: self.rule,
        }
    }

    pub fn at(&self, row: usize, col: usize) -> bool {
        self.grid.at(RowCol { row, col })
    }

    fn tick_cell(&self, row: usize, col: usize) -> bool {
        self.rule.lives(
            self.grid.at(RowCol { row, col }),
            self.grid.count_live_neighbours(RowCol { row, col }),
        )
    }
}

/* --------------------------------------------------------------------------------------------- */
/* --------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod test {

    use crate::glider::dense_grid::DenseGrid;
    use crate::glider::rle::{Rle, RleEntry};
    use crate::glider::rule::Rule;
    use crate::glider::universe::Universe;

    #[test]
    fn test_tick() {
        let rle = Rle {
            pattern: vec![
                RleEntry::Live(1),
                RleEntry::Dead(1),
                RleEntry::Live(1),
                RleEntry::Dead(2),
                RleEntry::NewRow(1),
                RleEntry::Dead(2),
                RleEntry::Live(1),
                RleEntry::Dead(2),
                RleEntry::NewRow(1),
                RleEntry::Dead(2),
                RleEntry::Live(1),
                RleEntry::Dead(2),
                RleEntry::NewRow(1),
                RleEntry::Dead(3),
                RleEntry::Live(2),
                RleEntry::NewRow(1),
                RleEntry::Dead(3),
                RleEntry::Live(2),
                RleEntry::NewRow(1),
            ],
        };
        let u = Universe::new(
            DenseGrid::new_from_rle(&rle, 5, 5),
            Rule::new(vec![3], vec![2, 3]),
        );

        // Any live cell with fewer than two live neighbours dies, as if caused by underpopulation.
        assert!(!u.tick_cell(0, 0));
        assert!(!u.tick_cell(0, 2));
        // Any live cell with two or three live neighbours lives on to the next generation.
        assert!(u.tick_cell(1, 2));
        assert!(u.tick_cell(4, 4));
        // Any live cell with more than three live neighbours dies, as if by overpopulation.
        assert!(!u.tick_cell(3, 3));

        // Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
        assert!(u.tick_cell(3, 2));
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
