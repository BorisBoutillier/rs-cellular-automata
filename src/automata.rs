use crate::rules::*;
use image::RgbImage;

pub struct Automata1DIter<'a> {
    automata: &'a Automata1D,
    idx: i32,
}
impl<'a> Iterator for Automata1DIter<'a> {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        self.idx += 1;
        if self.idx >= self.automata.view_start + self.automata.view_width as i32 {
            None
        } else if self.idx < self.automata.view_cell_start {
            Some(*self.automata.cells.first().unwrap())
        } else if self.idx < self.automata.view_cell_start + self.automata.cells.len() as i32 {
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
pub struct Automata1D {
    rule: Rule1D,
    step: u32,
    cells: Vec<u8>,
    view_start: i32,
    view_width: u32,
    view_cell_start: i32,
}
impl Automata1D {
    pub fn new(rule: Rule1D, view_start: i32, view_width: u32) -> Automata1D {
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
    pub fn iter(&self) -> Automata1DIter {
        Automata1DIter {
            automata: &self,
            idx: self.view_start - 1,
        }
    }
    pub fn step(&mut self, n_step: u32) {
        self.cells.reserve(2 * n_step as usize);
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
    pub fn cells_len(&self) -> usize {
        self.cells.len()
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
    pub fn as_image_buffer(&mut self, n_step: u32) -> RgbImage {
        let mut buf = Vec::new();
        for i in 0..(n_step + 1) {
            buf.extend(
                self.iter()
                    .flat_map(|c| {
                        let (r, g, b) = self.rule.cell_to_rgb(&c);
                        vec![r, g, b]
                    })
                    .collect::<Vec<_>>(),
            );
            if i != n_step {
                self.step(1)
            }
        }
        RgbImage::from_raw(self.view_width, n_step + 1, buf).unwrap()
    }
    pub fn as_rgb_vec(&mut self, n_step: u32) -> Vec<(u8, u8, u8)> {
        let mut buf = Vec::new();
        for i in 0..n_step {
            buf.extend(
                self.iter()
                    .map(|c| self.rule.cell_to_rgb(&c))
                    .collect::<Vec<_>>(),
            );
            if i != n_step - 1 {
                self.step(1)
            }
        }
        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn automata_1d_works() {
        // Rule 254 create black cell whenever any cell was black
        let rule = Rule1D::new(2, 254);
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
    fn automata2c_30_works() {
        // Rule 254 create black cell whenever any cell was black
        let rule = Rule1D::new(2, 30);
        let mut automata = Automata1D::new(rule, -5, 11);
        assert_eq!(automata.step, 0);
        assert_eq!(automata.as_text(), "|     *     |");
        automata.step(1);
        assert_eq!(automata.as_text(), "|    ***    |");
        automata.step(1);
        assert_eq!(automata.as_text(), "|   **  *   |");
        automata.step(1);
        assert_eq!(automata.as_text(), "|  ** ****  |");
        automata.step(1);
        assert_eq!(automata.as_text(), "| **  *   * |");
    }
    #[test]
    fn automata_1d_edges() {
        // Rule 1: All black-> white, other->white
        let rule = Rule1D::new(2, 127);
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
        let rule = Rule1D::new(2, 254);
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
