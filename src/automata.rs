use crate::rules::*;
use image::RgbImage;

pub struct Automata1DIter<'a, T: Rules1D> {
    automata: &'a Automata1D<T>,
    idx: i64,
}
impl<'a, T: Rules1D> Iterator for Automata1DIter<'a, T> {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        self.idx += 1;
        assert!(self.idx >= self.automata.view_start);
        if self.idx >= self.automata.view_start + self.automata.view_width as i64 {
            None
        } else if self.idx < self.automata.view_cell_start {
            Some(*self.automata.cells.first().unwrap())
        } else if self.idx < self.automata.view_cell_start + self.automata.cells.len() as i64 {
            Some(
                *self
                    .automata
                    .cells
                    .get((self.idx - self.automata.view_cell_start) as usize)
                    .unwrap(),
            )
        } else {
            Some(*self.automata.cells.last().unwrap())
        }
    }
}
pub struct Automata1D<T: Rules1D> {
    rule: T,
    step: usize,
    cells: Vec<u8>,
    view_start: i64,
    view_width: usize,
    view_cell_start: i64,
}
impl<T: Rules1D> Automata1D<T> {
    pub fn new(rule: T, view_start: i64, view_width: usize) -> Automata1D<T> {
        let cells = rule.initialize();
        Automata1D {
            rule,
            step: 0,
            cells,
            view_start,
            view_width,
            view_cell_start: -3,
        }
    }
    pub fn iter(&self) -> Automata1DIter<T> {
        Automata1DIter {
            automata: &self,
            idx: self.view_start - 1,
        }
    }
    pub fn step(&mut self, n_step: usize) {
        self.cells.reserve(2 * n_step);
        for _j in 0..n_step {
            let cur_len = self.cells.len();
            for i in 0..cur_len - 2 {
                *self.cells.get_mut(i).unwrap() = self.rule.apply(&self.cells[i..i + 3]);
            }
            *self.cells.get_mut(cur_len - 2).unwrap() = self.cells[cur_len - 3];
            *self.cells.get_mut(cur_len - 1).unwrap() = self.cells[cur_len - 3];
            self.cells.insert(0, self.cells[0]);
            self.cells.insert(0, self.cells[0]);
            self.step += 1;
            self.view_cell_start -= 1;
        }
    }
    pub fn as_text(&self) -> String {
        format!(
            "|{}|",
            self.iter()
                .map(|c| self.rule.cell_to_text(&c))
                .collect::<Vec<_>>()
                .join("")
        )
    }
    pub fn as_image_buffer(&mut self, n_step: usize) -> RgbImage {
        let mut buf = Vec::new();
        for i in 0..(n_step + 1) {
            buf.extend(
                self.iter()
                    .flat_map(|c| self.rule.cell_to_rgb_vec(&c))
                    .collect::<Vec<_>>(),
            );
            if i != n_step {
                self.step(1)
            }
        }
        RgbImage::from_raw(self.view_width as u32, (n_step + 1) as u32, buf).unwrap()
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
        assert_eq!(rule.apply(&vec![0, 0, 1]), 1);
        assert_eq!(rule.apply(&vec![0, 1, 0]), 0);
    }
    #[test]
    fn automata_1d_works() {
        // Rule 254 create black cell whenever any cell was black
        let rule = Rule1D::from_int(254);
        let mut automata = Automata1D::new(rule, -5, 11);
        assert_eq!(automata.step, 0);
        assert_eq!(automata.as_text(), "|     *     |");
        automata.step(1);
        assert_eq!(automata.as_text(), "|    ***    |");
        automata.step(1);
        assert_eq!(automata.as_text(), "|   *****   |");
        automata.step(1);
        assert_eq!(automata.as_text(), "|  *******  |");
        automata.step(1);
        assert_eq!(automata.as_text(), "| ********* |");
        automata.step(1);
        assert_eq!(automata.as_text(), "|***********|");
        for _i in 0..10 {
            automata.step(1);
            assert_eq!(automata.as_text(), "|***********|");
        }
    }
    #[test]
    fn automata_1d_edges() {
        // Rule 1: All black-> white, other->white
        let rule = Rule1D::from_int(127);
        let mut automata = Automata1D::new(rule, -5, 11);
        assert_eq!(automata.step, 0);
        assert_eq!(automata.as_text(), "|     *     |");
        for _i in 0..10 {
            automata.step(1);
            assert_eq!(automata.as_text(), "|***********|");
            automata.step(1);
            assert_eq!(automata.as_text(), "|           |");
        }
    }
    #[test]
    fn automata_1d_iter_works() {
        // Rule 254 create black cell whenever any cell was black
        let rule = Rule1D::from_int(254);
        let mut automata = Automata1D::new(rule, -5, 11);
        assert_eq!(
            automata.iter().collect::<Vec<_>>(),
            vec![0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0]
        );
        assert_eq!(
            automata.iter().collect::<Vec<_>>(),
            vec![0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0]
        );
        automata.step(1);
        assert_eq!(
            automata.iter().collect::<Vec<_>>(),
            vec![0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0]
        );
    }
}
