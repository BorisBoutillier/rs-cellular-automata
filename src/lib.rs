#[derive(Debug)]
pub struct Rule1D {
    outputs: Vec<u8>,
}
impl Rule1D {
    pub fn from_int(nb:u32) -> Rule1D {
        assert!(nb<256);
        let mut outputs = Vec::new();
        for i in 0..8 {
            outputs.push(
                if (nb>>i)%2 == 0 {0} else {1} 
            );
        }
        Rule1D{ outputs }
    }
    pub fn apply(&self,cells: &[u8]) -> u8{
        assert_eq!(cells.len(),3);
        let idx = ((cells[2] as usize) <<2) + ((cells[1] as usize)<<1) + (cells[0] as usize);
        *self.outputs.get(idx).unwrap()
    }
}


pub struct Automata1D {
    step  : usize,
    width : usize,
    rule  : Rule1D,
    cells : Vec<u8>,
}
impl Automata1D {
    pub fn new(width:usize,rule:Rule1D) -> Automata1D {
        assert!(width>0);
        assert!(width<65536);
        // Starts with all 0 apart a 1 in the middle cell
        let mut cells = vec![0;width+2];
        if let Some(cell) = cells.get_mut(width/2+1) {
            *cell= 1;
        }
        assert_eq!(cells.len(),width+2);
        Automata1D {
            step: 0,
            width,
            rule,
            cells,
        }
    }
    pub fn as_text(&self) -> String {
        let text = self.cells[1..self.width+1]
            .iter()
            .map(|&x| {
                if x==0 { " " } else { "*" }
            })
            .collect::<Vec<_>>()
            .join("");
        assert_eq!(text.len(),self.width);
        format!("|{}|",text)
    }
    pub fn step(&mut self) {
        let mut pattern = Vec::new();
        let mut new_cells = Vec::new();
        for &cell in self.cells.iter() {
            pattern.push(cell);
            if pattern.len()<3 {
                continue;
            } else if pattern.len()==4 {
                pattern.remove(0);
            }
            new_cells.push( self.rule.apply(&pattern) );
        }
        new_cells.insert(0,*new_cells.first().unwrap());
        new_cells.push( *new_cells.last().unwrap());
        assert_eq!(self.cells.len(),new_cells.len());
        self.cells = new_cells;
        self.step += 1;
   }
        

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rule_1d_works() {
        let rule = Rule1D::from_int(0);
        assert_eq!(rule.apply(&vec![1,1,0]),0);
        let rule = Rule1D::from_int(1);
        println!("Rule 1 {:?}",rule);
        assert_eq!(rule.apply(&vec![0,0,0]),1);
        assert_eq!(rule.apply(&vec![1,0,0]),0);
        assert_eq!(rule.apply(&vec![0,1,1]),0);
        let rule = Rule1D::from_int(2);
        assert_eq!(rule.apply(&vec![0,0,0]),0);
        assert_eq!(rule.apply(&vec![1,0,0]),1);
        assert_eq!(rule.apply(&vec![0,1,0]),0);
    }
    #[test]
    fn automata_1d_works() {
        let rule = Rule1D::from_int(2);
        let mut automata = Automata1D::new(11,rule);
        assert_eq!(automata.step, 0);
        assert_eq!(automata.as_text(),"     *     ");
        automata.step();
        assert_eq!(automata.as_text(),"      *    ");
    }
}
