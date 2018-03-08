/* --------------------------------------------------------------------------------------------- */

#[derive(Debug)]
pub enum RleEntry {
  Live(usize),
  Dead(usize),
  NewLine,
}

/* --------------------------------------------------------------------------------------------- */

#[derive(Debug)]
pub struct Rle {
  pub pattern: Vec<RleEntry>,
}

/* --------------------------------------------------------------------------------------------- */

impl Rle {
  pub fn bounds(&self) -> (usize, usize) {

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
        &RleEntry::NewLine  => {
          rows += 1;
          cols = 0;
        }
      };
    }
  
    if let RleEntry::NewLine = self.pattern[self.pattern.len() - 1]  {
      (rows, max_cols)
    }
    else {
      (rows + 1, max_cols)
    }
  }
}

/* --------------------------------------------------------------------------------------------- */

#[test]
fn test_bounds() {
  {
    let rle = Rle {
      pattern: vec![]
    };

    assert_eq!(rle.bounds(), (0, 0));
  }
  {
    let rle = Rle {
      pattern: vec![
        RleEntry::NewLine,
      ]
    };

    assert_eq!(rle.bounds(), (1, 0));
  }
  {
    let rle = Rle {
      pattern: vec![
        RleEntry::Live(1),
        RleEntry::Dead(2),
      ]
    };

    assert_eq!(rle.bounds(), (1, 3));
  }
  {
    let rle = Rle {
      pattern: vec![
        RleEntry::Live(1),
        RleEntry::Dead(2),
        RleEntry::NewLine,
      ]
    };

    assert_eq!(rle.bounds(), (1, 3));
  }
  {
    // 3o$2bo$bo!
    let rle = Rle {
      pattern: vec![
        RleEntry::Live(3), 
        RleEntry::NewLine,
        RleEntry::Dead(2),
        RleEntry::Live(1),
        RleEntry::NewLine,
        RleEntry::Dead(1),
        RleEntry::Live(1),
        RleEntry::NewLine,
      ]
    };
    
    assert_eq!(rle.bounds(), (3, 3));
  }
  {
    // 3o$2bo$bo!
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
    
    assert_eq!(rle.bounds(), (3, 3));
  }
}

/* --------------------------------------------------------------------------------------------- */
