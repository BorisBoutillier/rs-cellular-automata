#[derive(Debug)]
pub struct Rule1D {
    outputs: Vec<u8>,
}
impl Rule1D {
    pub fn from_int(nb: u32) -> Rule1D {
        assert!(nb < 256);
        let mut outputs = Vec::new();
        for i in 0..8 {
            outputs.push(if (nb >> i) % 2 == 0 { 0 } else { 1 });
        }
        Rule1D { outputs }
    }
    pub fn apply(&self, cells: &[u8]) -> u8 {
        assert_eq!(cells.len(), 3);
        let idx = ((cells[2] as usize) << 2) + ((cells[1] as usize) << 1) + (cells[0] as usize);
        *self.outputs.get(idx).unwrap()
    }
}

pub struct Automata1D {
    rule: Rule1D,
    step: usize,
    cells: Vec<u8>,
    view_start: i64,
    view_width: usize,
    view_cell_start: i64,
}
impl Automata1D {
    pub fn new(rule: Rule1D, view_start: i64, view_width: usize) -> Automata1D {
        let cells = vec![0, 0, 0, 1, 0, 0, 0];
        Automata1D {
            rule,
            step: 0,
            cells,
            view_start,
            view_width,
            view_cell_start: -3,
        }
    }
    pub fn as_text(&self) -> String {
        let mut text = Vec::new();
        for idx in self.view_start..(self.view_start + self.view_width as i64) {
            let cell = self
                .cells
                .get((idx - self.view_cell_start) as usize)
                .unwrap_or(self.cells.first().unwrap());
            if *cell == 0u8 {
                text.push(" ");
            } else {
                text.push("*");
            }
        }
        let text = text.join("");
        assert_eq!(text.len(), self.view_width);
        format!("|{}|", text)
    }
    pub fn step(&mut self) {
        let mut pattern = Vec::new();
        let mut new_cells = Vec::new();
        for &cell in self.cells.iter() {
            pattern.push(cell);
            if pattern.len() < 3 {
                continue;
            } else if pattern.len() == 4 {
                pattern.remove(0);
            }
            new_cells.push(self.rule.apply(&pattern));
        }
        assert_eq!(new_cells.len(), self.cells.len() - 2);
        let first = *new_cells.first().unwrap();
        let last = *new_cells.last().unwrap();
        new_cells.insert(0, first);
        new_cells.insert(0, first);
        new_cells.push(last);
        new_cells.push(last);
        assert_eq!(new_cells.len(), self.cells.len() + 2);
        self.cells = new_cells;
        self.step += 1;
        self.view_cell_start -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rule_1d_works() {
        let rule = Rule1D::from_int(0);
        assert_eq!(rule.apply(&vec![1, 1, 0]), 0);
        let rule = Rule1D::from_int(1);
        println!("Rule 1 {:?}", rule);
        assert_eq!(rule.apply(&vec![0, 0, 0]), 1);
        assert_eq!(rule.apply(&vec![1, 0, 0]), 0);
        assert_eq!(rule.apply(&vec![0, 1, 1]), 0);
        let rule = Rule1D::from_int(2);
        assert_eq!(rule.apply(&vec![0, 0, 0]), 0);
        assert_eq!(rule.apply(&vec![1, 0, 0]), 1);
        assert_eq!(rule.apply(&vec![0, 1, 0]), 0);
    }
    #[test]
    fn automata_1d_works() {
        // Rule 254 create black cell whenever any cell was black
        let rule = Rule1D::from_int(254);
        let mut automata = Automata1D::new(rule, -5, 11);
        assert_eq!(automata.step, 0);
        assert_eq!(automata.as_text(), "|     *     |");
        automata.step();
        assert_eq!(automata.as_text(), "|    ***    |");
        automata.step();
        assert_eq!(automata.as_text(), "|   *****   |");
        automata.step();
        assert_eq!(automata.as_text(), "|  *******  |");
        automata.step();
        assert_eq!(automata.as_text(), "| ********* |");
        automata.step();
        assert_eq!(automata.as_text(), "|***********|");
        for _i in 0..10 {
            automata.step();
            assert_eq!(automata.as_text(), "|***********|");
        }
    }
}
