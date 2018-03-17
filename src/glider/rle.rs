/* --------------------------------------------------------------------------------------------- */

#[derive(Debug, PartialEq)]
pub enum RleEntry {
  Live(usize),
  Dead(usize),
  NewRow(usize),
}

/* --------------------------------------------------------------------------------------------- */

#[derive(Debug)]
pub struct Rle {
  pub pattern: Vec<RleEntry>,
}

/* --------------------------------------------------------------------------------------------- */

impl Rle {
  pub fn dimension(&self) -> (usize, usize) {

    if self.pattern.is_empty() {
      return (0, 0)
    }

    let mut max_cols = 0;
    let mut cols = 0;
    let mut rows = 0;

    for entry in &self.pattern {
      match entry {
        &RleEntry::Live(nb) => {
          cols += nb;
          max_cols = usize::max(cols, max_cols);
        }
        &RleEntry::Dead(nb) => {
          cols += nb;
          max_cols = usize::max(cols, max_cols);
        }
        &RleEntry::NewRow(nb)  => {
          rows += nb;
          cols = 0;
        }
      };
    }

    if let RleEntry::NewRow(_) = self.pattern[self.pattern.len() - 1]  {
      (rows, max_cols)
    }
    else {
      (rows + 1, max_cols)
    }
  }
}

/* --------------------------------------------------------------------------------------------- */

#[test]
fn test_dimension() {
  {
    let rle = Rle {
      pattern: vec![]
    };

    assert_eq!(rle.dimension(), (0, 0));
  }
  {
    let rle = Rle {
      pattern: vec![
        RleEntry::NewRow(10),
      ]
    };

    assert_eq!(rle.dimension(), (10, 0));
  }
  {
    let rle = Rle {
      pattern: vec![
        RleEntry::Live(1),
        RleEntry::Dead(2),
      ]
    };

    assert_eq!(rle.dimension(), (1, 3));
  }
  {
    let rle = Rle {
      pattern: vec![
        RleEntry::Live(1),
        RleEntry::Dead(2),
        RleEntry::NewRow(1),
      ]
    };

    assert_eq!(rle.dimension(), (1, 3));
  }
  {
    // 3o$2bo$bo!
    let rle = Rle {
      pattern: vec![
        RleEntry::Live(3),
        RleEntry::NewRow(1),
        RleEntry::Dead(2),
        RleEntry::Live(1),
        RleEntry::NewRow(1),
        RleEntry::Dead(1),
        RleEntry::Live(1),
        RleEntry::NewRow(1),
      ]
    };

    assert_eq!(rle.dimension(), (3, 3));
  }
  {
    // 3o$2bo$bo
    let rle = Rle {
      pattern: vec![
        RleEntry::Live(3),
        RleEntry::NewRow(1),
        RleEntry::Dead(2),
        RleEntry::Live(1),
        RleEntry::NewRow(1),
        RleEntry::Dead(1),
        RleEntry::Live(1),
      ]
    };

    assert_eq!(rle.dimension(), (3, 3));
  }
}

/* --------------------------------------------------------------------------------------------- */
