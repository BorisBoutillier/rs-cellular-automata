pub trait Rules1D {
    fn initialize(&self) -> Vec<u8>;
    fn apply(&self, cells: &[u8]) -> u8;
    fn cell_to_rgb_vec(&self, cell: &u8) -> Vec<u8>;
    fn cell_to_text(&self, cell: &u8) -> String;

    fn cell_to_rgb(&self, cell: &u8) -> (u8, u8, u8) {
        let v = self.cell_to_rgb_vec(cell);
        (v[0], v[1], v[2])
    }
}

const BLACK: [u8; 3] = [0u8, 0u8, 0u8];
const BLUE: [u8; 3] = [0u8, 0u8, 192u8];
const WHITE: [u8; 3] = [255u8, 255u8, 255u8];

#[derive(Debug)]
pub struct Rule1D2Color {
    outputs: Vec<u8>,
}
impl Rule1D2Color {
    pub fn from_int(nb: u32) -> Rule1D2Color {
        assert!(nb < 256);
        let mut outputs = Vec::new();
        for i in 0..8 {
            outputs.push(if (nb >> i) % 2 == 0 { 0 } else { 1 });
        }
        Rule1D2Color { outputs }
    }
}
impl Rules1D for Rule1D2Color {
    fn initialize(&self) -> Vec<u8> {
        return vec![0, 0, 0, 1, 0, 0, 0];
    }
    #[inline]
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

#[derive(Debug)]
pub struct Rule1D3Color {
    outputs: Vec<u8>,
}
impl Rule1D3Color {
    pub fn from_int(nb: u32) -> Rule1D3Color {
        assert!(nb < (81 * 729));
        let mut outputs = Vec::new();
        outputs.push((nb % 3) as u8); // 0 0 0
        outputs.push(((nb / 3) % 3) as u8); // 0 0 1
        outputs.push(((nb / 9) % 3) as u8); // 0 1 1
        outputs.push(((nb / 27) % 3) as u8); // 1 1 1
        outputs.push(((nb / 81) % 3) as u8); // 0 0 2
        outputs.push(((nb / 243) % 3) as u8); // 0 1 2
        outputs.push(((nb / 729) % 3) as u8); // 1 1 2
        outputs.push(0);
        outputs.push(((nb / (3 * 729)) % 3) as u8); // 0 2 2
        outputs.push(((nb / (9 * 729)) % 3) as u8); // 1 2 2
        outputs.push(0);
        outputs.push(0);
        outputs.push(((nb / (27 * 729)) % 3) as u8); // 2 2 2
        Rule1D3Color { outputs }
    }
}
impl Rules1D for Rule1D3Color {
    fn initialize(&self) -> Vec<u8> {
        return vec![0, 0, 0, 1, 0, 0, 0];
    }
    #[inline]
    fn apply(&self, cells: &[u8]) -> u8 {
        assert_eq!(cells.len(), 3);
        let idx = (cells[0] * cells[0] + cells[1] * cells[1] + cells[2] * cells[2]) as usize;
        *self.outputs.get(idx).expect(&format!(
            "Unexpected cells {} {} {}",
            cells[2], cells[1], cells[0],
        ))
    }
    fn cell_to_text(&self, cell: &u8) -> String {
        match *cell {
            0 => String::from(" "),
            1 => String::from("+"),
            _ => String::from("*"),
        }
    }
    fn cell_to_rgb_vec(&self, cell: &u8) -> Vec<u8> {
        match *cell {
            0 => WHITE.to_vec(),
            1 => BLUE.to_vec(),
            _ => BLACK.to_vec(),
        }
    }
}
