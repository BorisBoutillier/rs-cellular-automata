use image::RgbImage;
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
        let idx = ((cells[0] as usize) << 2) + ((cells[1] as usize) << 1) + (cells[2] as usize);
        *self.outputs.get(idx).unwrap()
    }
}

const BLACK: [u8; 3] = [0u8, 0u8, 0u8];
const WHITE: [u8; 3] = [255u8, 255u8, 255u8];
pub struct Automata1DIter<'a> {
    automata: &'a Automata1D,
    idx: i64,
}
impl<'a> Iterator for Automata1DIter<'a> {
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
    pub fn iter(&self) -> Automata1DIter {
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
                .map(|c| if c == 0 { " " } else { "*" })
                .collect::<Vec<_>>()
                .join("")
        )
    }
    pub fn as_image_buffer(&mut self, n_step: usize) -> RgbImage {
        let mut buf = Vec::new();
        for i in 0..(n_step + 1) {
            buf.extend(
                self.iter()
                    .flat_map(|c| {
                        if c == 0 {
                            WHITE.to_vec()
                        } else {
                            BLACK.to_vec()
                        }
                    })
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
