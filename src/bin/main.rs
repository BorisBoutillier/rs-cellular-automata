use rs_cellular_automata::*;

fn main() {
    let rule = Rule1D::from_int(30);
    let mut automata = Automata1D::new(80,rule);
    for i in 0..40 {
        println!("{}",automata.as_text());
        automata.step();
    }
}
