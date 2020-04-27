extern crate image;
use rs_cellular_automata::*;
use std::path::PathBuf;
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
    /// When defined, output is saved as a PNG image in the named file
    /// When undefined, output is displayed as text in stdout
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    output: Option<PathBuf>,
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
    if print_step > 0 {
        automata.step(print_step);
    }
    if let Some(image_file) = opt.output {
        let image_buffer = automata.as_image_buffer(opt.steps - print_step);
        image_buffer
            .save_with_format(image_file, image::ImageFormat::Png)
            .unwrap();
    } else {
        for _i in print_step..opt.steps {
            println!("{}", automata.as_text());
            automata.step(1);
        }
    }
}
