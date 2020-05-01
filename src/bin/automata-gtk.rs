extern crate gio;
extern crate gtk;

use gdk_pixbuf::{Colorspace, Pixbuf};
use gio::prelude::*;
use glib::clone;
use gtk::prelude::*;
use rand::{thread_rng, Rng};
use rs_cellular_automata::*;
use std::env;

fn main() {
    let application = gtk::Application::new(Some("xyz.bobox.automata-gtk"), Default::default())
        .expect("Failed to initialize GTK");
    application.connect_activate(|app| build_ui(app));
    application.run(&env::args().collect::<Vec<_>>());
}

fn filter_integer(entry: &gtk::Entry) {
    let text = entry
        .get_text()
        .unwrap()
        .as_str()
        .chars()
        .filter_map(|c| c.to_digit(10).and_then(|i| Some(i.to_string())))
        .collect::<Vec<_>>()
        .join("");
    entry.set_text(&text);
}
fn build_ui(app: &gtk::Application) {
    let glade_src = include_str!("automata-gtk.glade");
    let builder = gtk::Builder::new_from_string(glade_src);
    let window: gtk::Window = builder.get_object("application_window").unwrap();
    window.set_application(Some(app));

    let rule_nb_entry: gtk::Entry = builder.get_object("rule_nb_entry").unwrap();
    let width_entry: gtk::Entry = builder.get_object("width_entry").unwrap();
    let height_entry: gtk::Entry = builder.get_object("height_entry").unwrap();
    let rule_rand_btn: gtk::Button = builder.get_object("rule_rand_btn").unwrap();
    let display_img: gtk::Image = builder.get_object("display_img").unwrap();
    let play_btn: gtk::Button = builder.get_object("play_btn").unwrap();

    let pixbuf =
        Pixbuf::new(Colorspace::Rgb, false, 8, 1600, 800).expect("Cannot create the Pixbuf!");
    pixbuf.fill(0xFFFFFFFF);
    display_img.set_from_pixbuf(Some(&pixbuf));

    rule_nb_entry.connect_changed(&filter_integer);
    height_entry.connect_changed(&filter_integer);
    width_entry.connect_changed(&filter_integer);
    rule_rand_btn.connect_clicked(clone!(@weak rule_nb_entry => move |_| {
        let mut rng = thread_rng();
        let rule_id = rng.gen_range(0, 81 * 729);
        rule_nb_entry.set_text(&rule_id.to_string());
        pixbuf.fill(rule_id);
    }));
    //clone!(@weak counter_label => |X|
    play_btn.connect_clicked(clone!(@weak rule_nb_entry => move |_| {
        let width = width_entry
            .get_text()
            .unwrap()
            .as_str()
            .parse::<u32>()
            .unwrap();
        let height = height_entry
            .get_text()
            .unwrap()
            .as_str()
            .parse::<u32>()
            .unwrap();
        let rule_nb = rule_nb_entry
            .get_text()
            .unwrap()
            .as_str()
            .parse::<u32>()
            .unwrap_or(1600);
        let rule = Rule1D3Color::from_int(rule_nb);
        let mut automata = Automata1D::new(rule, -(width as i32) / 2, width as u32);
        let pixbuf = automata.as_pixbuf(height);
        display_img.set_from_pixbuf(Some(&pixbuf));
    }));
    window.show_all();
}
