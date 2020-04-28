pub trait Rules1D {
    fn initialize(&self) -> Vec<u8>;
    fn apply(&self, cells: &[u8]) -> u8;
    fn cell_to_rgb_vec(&self, cell: &u8) -> Vec<u8>;
    fn cell_to_text(&self, cell: &u8) -> String;
}

const BLACK: [u8; 3] = [0u8, 0u8, 0u8];
const WHITE: [u8; 3] = [255u8, 255u8, 255u8];

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
}
impl Rules1D for Rule1D {
    fn initialize(&self) -> Vec<u8> {
        return vec![0, 0, 0, 1, 0, 0, 0];
    }
    fn apply(&self, cells: &[u8]) -> u8 {
        assert_eq!(cells.len(), 3);
        let idx = ((cells[0] as usize) << 2) + ((cells[1] as usize) << 1) + (cells[2] as usize);
        *self.outputs.get(idx).unwrap()
    }
    fn cell_to_text(&self, cell: &u8) -> String {
        if *cell == 0 {
            String::from(" ")
        } else {
            String::from("*")
        }
    }
    fn cell_to_rgb_vec(&self, cell: &u8) -> Vec<u8> {
        if *cell == 0 {
            WHITE.to_vec()
        } else {
            BLACK.to_vec()
        }
    }
}
