/* --------------------------------------------------------------------------------------------- */

#[derive(Clone, Copy, Debug)]
pub struct Rule {
    rule: [[bool; 9]; 2],
}

/* --------------------------------------------------------------------------------------------- */

impl Rule {
    pub fn new(birth: Vec<u8>, survival: Vec<u8>) -> Self {
        let mut b = [false; 9];
        let mut s = [false; 9];

        for i in birth {
            b[i as usize] = true;
        }

        for i in survival {
            s[i as usize] = true;
        }

        Rule { rule: [b, s] }
    }

    pub fn lives(&self, previous: bool, nb_live_neighbors: u8) -> bool {
        self.rule[previous as usize][nb_live_neighbors as usize]
    }
}

/* --------------------------------------------------------------------------------------------- */
