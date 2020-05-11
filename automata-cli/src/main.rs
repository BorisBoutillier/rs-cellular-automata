extern crate automata_lib;

use automata_lib::*;
use rand::{thread_rng, Rng};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    /// Define the rule number of colors in the automata, 2,3 or 4 is supported
    #[structopt(
        short = "c",
        long = "colors",
        default_value = "3",
        possible_values(&["2", "3", "4"])
    )]
    colors: u8,
    /// Define the rule number that the cellular automata will follow, represend as an integer
    /// If not provided, a random rule will be choosen.
    #[structopt(short = "r", long = "rule")]
    rule: Option<u64>,
    /// Define the number of step to iterate on the cellular automata
    #[structopt(short = "s", long = "steps", default_value = "40")]
    steps: u32,
    /// Define the width of the viewed area of the cellular automata
    #[structopt(short = "w", long = "width", default_value = "80")]
    view_width: u32,
    /// Define the starting point of the viewed area, in relation to the starting black cell.
    /// When not provided, view will be centered around the starting cell
    #[structopt(short = "x", allow_hyphen_values(true))]
    view_start: Option<i32>,
    /// When defines, only print to stdout the last 'last' steps.
    /// When undefined, print all steps.
    #[structopt(long = "last")]
    last: Option<u32>,
    /// When defined, output is saved as a PNG image in the named file
    /// When undefined, output is displayed as text in stdout
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    output: Option<PathBuf>,
}
fn main() {
    let mut rng = thread_rng();
    let opt = Opt::from_args();

    let rule_nb = match opt.rule {
        Some(v) => v,
        _ => rng.gen_range(0, Rule1D::get_max_nb(opt.colors)),
    };
    let rule = Rule1D::new(opt.colors, rule_nb);
    let view_start = match opt.view_start {
        Some(v) => v,
        _ => -(opt.view_width as i32) / 2,
    };
    let print_step = match opt.last {
        Some(v) if v <= opt.steps => opt.steps - v,
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
    println!("Colors: {}, Rule: {}", opt.colors, rule_nb)
}
