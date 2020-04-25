use rs_cellular_automata::*;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    /// Define the rule number that the cellular automata will follow, represend as an integer
    #[structopt(short = "r", long = "rule")]
    rule: u32,
    /// Define the number of step to iterate on the cellular automata
    #[structopt(short = "s", long = "steps")]
    steps: usize,
    /// Define the width of the viewed area of the cellular automata
    #[structopt(short = "w", long = "width", default_value = "130")]
    view_width: usize,
    /// Define the starting point of the viewed area, in relation to the starting black cell.
    /// When not provided, view will be centered around the starting cell
    #[structopt(short = "x", allow_hyphen_values(true))]
    view_start: Option<i64>,
    /// When defines, only print to stdout the last 'last' steps.
    /// When undefined, print all steps.
    #[structopt(long = "last")]
    last: Option<usize>,
}
fn main() {
    let opt = Opt::from_args();

    let rule = Rule1D::from_int(opt.rule);
    let view_start = match opt.view_start {
        Some(v) => v,
        _ => -(opt.view_width as i64) / 2,
    };
    let print_step = match opt.last {
        Some(v) => opt.steps - v,
        _ => 0,
    };
    let mut automata = Automata1D::new(rule, view_start, opt.view_width);
    for i in 0..opt.steps {
        if i >= print_step {
            println!("{}", automata.as_text());
        }
        automata.step();
    }
}
