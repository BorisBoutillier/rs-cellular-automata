extern crate gio;
extern crate gtk;

use gdk_pixbuf::{Colorspace, Pixbuf};
use gio::prelude::*;
use glib::clone;
use gtk::prelude::*;
use rand::{thread_rng, Rng};
use rs_cellular_automata::*;
use std::env;
use std::sync::{Arc, Mutex};

struct AutomataModel {
    automata: Automata1D<Rule1D3Color>,
    rule_nb: u32,
    width: u32,
    height: u32,
    continuous: bool,
    pixbuf: gdk_pixbuf::Pixbuf,
    clean: bool,
}
impl AutomataModel {
    fn new() -> AutomataModel {
        let rule_nb = 3u32;
        let width = 1600u32;
        let height = 800u32;
        let rule = Rule1D3Color::from_int(rule_nb);
        let automata = Automata1D::new(rule, -(width as i32) / 2, width);
        let pixbuf = Pixbuf::new(Colorspace::Rgb, false, 8, width as i32, height as i32)
            .expect("Cannot create the Pixbuf!");
        pixbuf.fill(0xFFFFFFFF);
        AutomataModel {
            automata,
            rule_nb,
            width,
            height,
            continuous: true,
            pixbuf,
            clean: true,
        }
    }
    fn reset(&mut self) {
        self.reset_automata();
        self.reset_pixbuf();
        self.clean = true;
    }
    fn play(&mut self) {
        if !self.clean {
            self.reset_automata();
            self.clean = true;
        }
        self.pixbuf = self.automata.as_pixbuf(self.height);
        if !self.continuous {
            self.clean = false;
        }
    }
    fn reset_pixbuf(&mut self) {
        self.pixbuf = Pixbuf::new(
            Colorspace::Rgb,
            false,
            8,
            self.width as i32,
            self.height as i32,
        )
        .expect("Cannot create the Pixbuf!");
        self.pixbuf.fill(0xFFFFFFFF);
    }
    fn reset_automata(&mut self) {
        let rule = Rule1D3Color::from_int(self.rule_nb);
        self.automata = Automata1D::new(rule, -(self.width as i32) / 2, self.width);
    }
    fn set_rule_nb(&mut self, rule_nb: u32) -> u32 {
        if rule_nb != self.rule_nb {
            self.rule_nb = rule_nb;
            self.clean = false;
        }
        rule_nb
    }
    fn set_width(&mut self, width: u32) -> u32 {
        if width != self.width {
            self.width = width;
            self.clean = false;
        }
        width
    }
    fn set_height(&mut self, height: u32) -> u32 {
        if height != self.height {
            self.height = height;
            self.clean = false;
        }
        height
    }
    fn set_continous(&mut self, continous: bool) {
        if continous != self.continuous {
            self.continuous = continous;
            self.clean = false;
        }
    }
}
fn main() {
    let application = gtk::Application::new(Some("xyz.bobox.automata-gtk"), Default::default())
        .expect("Failed to initialize GTK");
    let model = Arc::new(Mutex::new(AutomataModel::new()));
    application.connect_activate(clone!(@weak model => move |app| build_ui(app, model)));
    application.run(&env::args().collect::<Vec<_>>());
}

fn filter_integer(entry: &gtk::Entry) -> String {
    let text = entry
        .get_text()
        .unwrap()
        .as_str()
        .chars()
        .filter_map(|c| c.to_digit(10).and_then(|i| Some(i.to_string())))
        .collect::<Vec<_>>()
        .join("");
    entry.set_text(&text);
    text
}
fn build_ui(app: &gtk::Application, model: Arc<Mutex<AutomataModel>>) {
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
    let reset_btn: gtk::Button = builder.get_object("reset_btn").unwrap();
    let save_btn: gtk::Button = builder.get_object("save_btn").unwrap();
    let continuous_chk: gtk::CheckButton = builder.get_object("continuous_chk").unwrap();
    let save_dlg: gtk::FileChooserDialog = builder.get_object("save_file_dlg").unwrap();

    display_img.set_from_pixbuf(Some(&model.lock().unwrap().pixbuf));

    rule_nb_entry.connect_changed(clone!(@weak model => move |entry| {
        let text = filter_integer(&entry);
        let mut m = model.lock().unwrap();
        let nb = (*m).set_rule_nb(text.parse::<u32>().unwrap());
        entry.set_text(&nb.to_string());
    }));
    height_entry.connect_changed(clone!(@weak model => move |entry| {
        let text = filter_integer(&entry);
        let mut m = model.lock().unwrap();
        let nb = m.set_height(text.parse::<u32>().unwrap());
        entry.set_text(&nb.to_string());
    }));
    width_entry.connect_changed(clone!(@weak model => move |entry| {
        let text = filter_integer(&entry);
        let mut m = model.lock().unwrap();
        let nb = m.set_width(text.parse::<u32>().unwrap());
        entry.set_text(&nb.to_string());
    }));
    continuous_chk.connect_clicked(clone!(@weak model => move |entry| {
        let mut m = model.lock().unwrap();
        m.set_continous(entry.get_active());
    }));
    rule_rand_btn.connect_clicked(clone!(@weak rule_nb_entry => move |_| {
        let mut rng = thread_rng();
        let rule_id = rng.gen_range(0, 81 * 729);
        rule_nb_entry.set_text(&rule_id.to_string());
    }));
    save_btn.connect_clicked(clone!(@weak model,@weak display_img => move |_| {
        save_dlg.show();
        let rule_nb = (*model.lock().unwrap()).rule_nb;
        save_dlg.set_current_name(format!("3C_{}.png",rule_nb));
        if save_dlg.run() == gtk::ResponseType::Ok {
            if let Some(filename) = save_dlg.get_filename() {
                display_img.get_pixbuf().unwrap().savev(filename,"png",&[]).unwrap();
            }
        }
        save_dlg.hide();
    }));
    //clone!(@weak counter_label => |X|
    play_btn.connect_clicked(clone!(@weak model,@weak display_img => move |_| {
        let mut m = model.lock().unwrap();
        m.play();
        display_img.set_from_pixbuf(Some(&m.pixbuf));
    }));
    reset_btn.connect_clicked(clone!(@weak model,@weak display_img => move |_| {
        let mut m = model.lock().unwrap();
        m.reset();
        display_img.set_from_pixbuf(Some(&m.pixbuf));
    }));
    window.show_all();
}
