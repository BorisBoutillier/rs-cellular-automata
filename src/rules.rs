const BLACK: (u8, u8, u8) = (0u8, 0u8, 0u8);
const BLUE: (u8, u8, u8) = (0u8, 0u8, 192u8);
const CYAN: (u8, u8, u8) = (0u8, 192u8, 192u8);
const WHITE: (u8, u8, u8) = (255u8, 255u8, 255u8);

// 2 Colors : All combinations, 8 entries, rule_nb < 2**8
// 3 Colors : Ordered combinations, 10 entries (000,001,011,111,002,012,112,022,122,222) , rule_nb < 3**10
// 4 Colors : Totalistics sum of cell values , 10 entries (0..9), rule_nb < 4**10
#[derive(Debug)]
pub struct Rule1D {
    n_colors: u8,
    outputs: Vec<u8>,
}
impl Rule1D {
    pub fn new(n_colors: u8, rule_nb: u64) -> Rule1D {
        assert!(vec![2, 3, 4].contains(&n_colors));
        let rule_nb_max = Rule1D::get_max_nb(n_colors);
        if rule_nb >= rule_nb_max {
            panic!("The provide rule_nb {} is incompatible with {} colors. Maximum for this number of colors is {}"
                ,rule_nb,n_colors,rule_nb_max);
        }
        let mut outputs = Vec::new();
        match n_colors {
            2 => {
                for i in 0..8 {
                    outputs.push(((rule_nb >> i) % 2) as u8);
                }
            }
            3 => {
                outputs.push((rule_nb % 3) as u8); // 0 0 0
                outputs.push(((rule_nb / 3) % 3) as u8); // 0 0 1
                outputs.push(((rule_nb / 9) % 3) as u8); // 0 1 1
                outputs.push(((rule_nb / 27) % 3) as u8); // 1 1 1
                outputs.push(((rule_nb / 81) % 3) as u8); // 0 0 2
                outputs.push(((rule_nb / 243) % 3) as u8); // 0 1 2
                outputs.push(((rule_nb / 729) % 3) as u8); // 1 1 2
                outputs.push(0);
                outputs.push(((rule_nb / (3 * 729)) % 3) as u8); // 0 2 2
                outputs.push(((rule_nb / (9 * 729)) % 3) as u8); // 1 2 2
                outputs.push(0);
                outputs.push(0);
                outputs.push(((rule_nb / (27 * 729)) % 3) as u8); // 2 2 2
            }
            _ => {
                for i in 0..10 {
                    outputs.push(((rule_nb >> (2 * i)) % 4) as u8);
                }
            }
        }
        Rule1D { n_colors, outputs }
    }
    pub fn get_max_nb(n_colors: u8) -> u64 {
        assert!(vec![2, 3, 4].contains(&n_colors));
        match n_colors {
            2 => 256,
            3 => 59049,
            _ => 1048576,
        }
    }
    pub fn initialize(&self) -> Vec<u8> {
        return vec![0, 0, 0, self.n_colors - 1, 0, 0, 0];
    }
    #[inline]
    pub fn apply(&self, cells: &[u8]) -> u8 {
        assert_eq!(cells.len(), 3);
        let idx = match self.n_colors {
            2 => ((cells[0] as usize) << 2) + ((cells[1] as usize) << 1) + (cells[2] as usize),
            3 => (cells[0] * cells[0] + cells[1] * cells[1] + cells[2] * cells[2]) as usize,
            _ => cells.iter().sum::<u8>() as usize,
        };
        *self.outputs.get(idx).unwrap()
    }
    pub fn cell_to_text(&self, cell: &u8) -> String {
        String::from(match *cell {
            0 => " ",
            x if x == self.n_colors - 1 => "*",
            x if x == self.n_colors - 2 => "+",
            _ => "-",
        })
    }
    pub fn cell_to_rgb(&self, cell: &u8) -> (u8, u8, u8) {
        match *cell {
            0 => WHITE,
            x if x == self.n_colors - 1 => BLACK,
            x if x == self.n_colors - 2 => BLUE,
            _ => CYAN,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rule_works() {
        let rule = Rule1D::new(2, 0);
        assert_eq!(rule.apply(&vec![1, 1, 0]), 0);
        let rule = Rule1D::new(2, 1);
        assert_eq!(rule.apply(&vec![0, 0, 0]), 1);
        assert_eq!(rule.apply(&vec![1, 0, 0]), 0);
        assert_eq!(rule.apply(&vec![0, 1, 1]), 0);
        let rule = Rule1D::new(2, 2);
        assert_eq!(rule.apply(&vec![0, 0, 0]), 0);
        assert_eq!(rule.apply(&vec![0, 0, 1]), 1);
        assert_eq!(rule.apply(&vec![0, 1, 0]), 0);
    }
    #[test]
    fn rule2c_30_works() {
        let rule = Rule1D::new(2, 30);
        assert_eq!(rule.apply(&vec![1, 1, 1]), 0);
        assert_eq!(rule.apply(&vec![0, 1, 1]), 1);
        assert_eq!(rule.apply(&vec![1, 1, 0]), 0);
    }
}
