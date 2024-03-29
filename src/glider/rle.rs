use std::io::{self, Error, ErrorKind};
use std::io::{BufRead, BufReader, Read};

use super::rule::Rule;

/* --------------------------------------------------------------------------------------------- */

#[derive(Debug, Eq, PartialEq)]
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
            return (0, 0);
        }

        let mut max_cols = 0;
        let mut cols = 0;
        let mut rows = 0;

        for entry in &self.pattern {
            match *entry {
                RleEntry::Live(nb) => {
                    cols += nb;
                    max_cols = usize::max(cols, max_cols);
                }
                RleEntry::Dead(nb) => {
                    cols += nb;
                    max_cols = usize::max(cols, max_cols);
                }
                RleEntry::NewRow(nb) => {
                    rows += nb;
                    cols = 0;
                }
            };
        }

        if let RleEntry::NewRow(_) = self.pattern[self.pattern.len() - 1] {
            (rows, max_cols)
        } else {
            (rows + 1, max_cols)
        }
    }

    pub fn read<R: Read>(reader: BufReader<R>) -> io::Result<(Self, Rule)> {
        let mut pattern = vec![];
        let mut rule_b = vec![];
        let mut rule_s = vec![];

        'main_loop: for l in reader.lines() {
            let line = l?;

            if line.is_empty() || line.starts_with('#') {
                continue;
            } else if line.starts_with('x') {
                let all: Vec<_> = line.split(|c| c == '=' || c == ',').collect();
                if all.len() != 6 {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        format!("Unable to parse {}", line),
                    ));
                }

                let r: Vec<_> = all[5].trim().split('/').collect();
                if r.len() != 2 || !r[0].starts_with('B') || !r[1].starts_with('S') {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        format!("Unable to parse {}", line),
                    ));
                }

                for c in r[0].chars().skip(1) {
                    rule_b.push(c.to_string().parse::<u8>().unwrap());
                }

                for c in r[1].chars().skip(1) {
                    rule_s.push(c.to_string().parse::<u8>().unwrap());
                }
                println!("Rule: B{:?}/S{:?}", rule_b, rule_s);
            } else {
                let mut current_integer = String::from("");
                for c in line.chars() {
                    match c {
                        '!' => {
                            break 'main_loop;
                        }
                        n if n.is_numeric() => {
                            current_integer.push(n);
                        }
                        c => {
                            let nb = current_integer.parse::<usize>().unwrap_or(1);
                            pattern.push(match c {
                                'o' => RleEntry::Live(nb),
                                'b' => RleEntry::Dead(nb),
                                '$' => RleEntry::NewRow(nb),
                                x => {
                                    return Err(Error::new(
                                        ErrorKind::InvalidData,
                                        format!("Invalid '{}'", x),
                                    ))
                                }
                            });
                            current_integer.clear();
                        }
                    }
                }
            }
        }

        if rule_b.is_empty() && rule_s.is_empty() {
            println!("Use default rule B3/S23");
            Ok((Rle { pattern }, Rule::new(vec![3], vec![2, 3])))
        } else {
            Ok((Rle { pattern }, Rule::new(rule_b, rule_s)))
        }
    }
}

/* --------------------------------------------------------------------------------------------- */

#[test]
fn test_dimension() {
    {
        let rle = Rle { pattern: vec![] };

        assert_eq!(rle.dimension(), (0, 0));
    }
    {
        let rle = Rle {
            pattern: vec![RleEntry::NewRow(10)],
        };

        assert_eq!(rle.dimension(), (10, 0));
    }
    {
        let rle = Rle {
            pattern: vec![RleEntry::Live(1), RleEntry::Dead(2)],
        };

        assert_eq!(rle.dimension(), (1, 3));
    }
    {
        let rle = Rle {
            pattern: vec![RleEntry::Live(1), RleEntry::Dead(2), RleEntry::NewRow(1)],
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
            ],
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
            ],
        };

        assert_eq!(rle.dimension(), (3, 3));
    }
}

/* --------------------------------------------------------------------------------------------- */

#[test]
fn read_glider() {
    {
        let data = "3h";
        let rle_read = Rle::read(BufReader::new(data.as_bytes()));

        assert!(rle_read.is_err());
    }
    {
        let rle = Rle { pattern: vec![] };
        let data = "";
        let rle_read = Rle::read(BufReader::new(data.as_bytes()));

        assert_eq!(rle.pattern, rle_read.unwrap().0.pattern);
    }
    {
        let rle = Rle {
            pattern: vec![RleEntry::Live(3)],
        };

        let data = "x = 3, y = 3, rule = B3/S23\n3o!\n";
        let rle_read = Rle::read(BufReader::new(data.as_bytes()));

        assert_eq!(rle.pattern, rle_read.unwrap().0.pattern);
    }
    {
        let rle = Rle {
            pattern: vec![RleEntry::NewRow(10)],
        };

        let data = "#COMMENT\n10$!\n";
        let rle_read = Rle::read(BufReader::new(data.as_bytes()));

        assert_eq!(rle.pattern, rle_read.unwrap().0.pattern);
    }
    {
        let rle = Rle {
            pattern: vec![RleEntry::Dead(42)],
        };

        let data = "\n42b\n";
        let rle_read = Rle::read(BufReader::new(data.as_bytes()));

        assert_eq!(rle.pattern, rle_read.unwrap().0.pattern);
    }
    {
        let rle = Rle {
            pattern: vec![
                RleEntry::Live(3),
                RleEntry::NewRow(1),
                RleEntry::Dead(2),
                RleEntry::Live(1),
                RleEntry::NewRow(1),
                RleEntry::Dead(1),
                RleEntry::Live(1),
            ],
        };

        let data = "x = 3, y = 3, rule = B3/S23\n3o$2bo$bo!\n";
        let rle_read = Rle::read(BufReader::new(data.as_bytes()));

        assert_eq!(rle.pattern, rle_read.unwrap().0.pattern);
    }
}

/* --------------------------------------------------------------------------------------------- */
