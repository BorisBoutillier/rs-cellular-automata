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
use std::thread;
use std::time::Duration;

enum Message {
    UpdatePlayButton(bool),                       // is_playing
    ResetDrawing(i32, i32),                       // width, height
    DrawStripe(i32, i32, i32, Vec<(u8, u8, u8)>), // row, width, height, rbg_vec
    SetRuleNb(u64),                               // value
    SetNColors(u8),                               // value
    SetWidth(i32),                                // value
    SetHeight(i32),                               // value
    SetStepNb(u32),                               // value
}

struct AutomataModel {
    automata: Option<Automata1D>,
    n_colors: u8,
    rule_nb: u64,
    width: i32,
    height: i32,
    continuous: bool,
    playing: bool,
    cur_row: i32,
    clean: bool,
    tx: Option<glib::Sender<Message>>,
}
impl AutomataModel {
    fn new() -> AutomataModel {
        AutomataModel {
            automata: None,
            n_colors: 0,
            rule_nb: 0,
            width: 0,
            height: 0,
            continuous: false,
            playing: false,
            cur_row: 0,
            clean: true,
            tx: None,
        }
    }
    fn initialize(&mut self) {
        self.set_n_colors(3);
        self.set_rule_nb(40327);
        self.set_width(1600);
        self.set_height(800);
        self.set_continous(true);
        self.reset();
    }
    fn reset(&mut self) {
        self.reset_automata();
        if self.width != 0 && self.height != 0 {
            self.tx
                .as_ref()
                .unwrap()
                .send(Message::ResetDrawing(self.width, self.height))
                .unwrap();
        }
        self.clean = true;
        self.cur_row = 0;
        self.stop_playing();
    }
    fn play(&mut self, n_steps: i32) {
        if self.playing {
            let rgb_vec = self.automata.as_mut().unwrap().as_rgb_vec(n_steps as u32);
            self.tx
                .as_ref()
                .unwrap()
                .send(Message::DrawStripe(
                    self.cur_row,
                    self.width,
                    n_steps,
                    rgb_vec,
                ))
                .unwrap();
            self.tx
                .as_ref()
                .unwrap()
                .send(Message::SetStepNb(
                    self.automata.as_ref().unwrap().get_cur_step(),
                ))
                .unwrap();
            self.cur_row += n_steps;
            if !self.continuous && self.cur_row >= self.height {
                self.stop_playing();
                self.clean = false;
            }
        }
    }
    fn set_tx(&mut self, tx: glib::Sender<Message>) {
        self.tx = Some(tx);
        self.initialize();
    }
    fn switch_playing(&mut self) {
        if self.playing {
            self.stop_playing();
        } else if self.width > 0 && self.height > 0 {
            self.start_playing();
        }
    }
    fn start_playing(&mut self) {
        if !self.clean {
            self.reset();
            self.clean = true;
        }
        self.playing = true;
        self.tx
            .as_ref()
            .unwrap()
            .send(Message::UpdatePlayButton(true))
            .unwrap();
    }
    fn stop_playing(&mut self) {
        self.playing = false;
        self.tx
            .as_ref()
            .unwrap()
            .send(Message::UpdatePlayButton(false))
            .unwrap();
    }
    fn reset_automata(&mut self) {
        let rule = Rule1D::new(self.n_colors, self.rule_nb);
        self.automata = Some(Automata1D::new(rule, -self.width / 2, self.width as u32));
    }
    fn set_n_colors(&mut self, n_colors: u8) {
        let filtered = if n_colors < 2 {
            2
        } else if n_colors > 4 {
            4
        } else {
            n_colors
        };
        if filtered != self.n_colors {
            self.n_colors = filtered;
            self.clean = false;
            self.tx
                .as_ref()
                .unwrap()
                .send(Message::SetNColors(self.n_colors))
                .unwrap();
        }
        // Rule_nb can become illegal, try to setting it again
        // This will legalize it
        self.set_rule_nb(self.rule_nb);
    }
    fn set_rule_nb(&mut self, rule_nb: u64) {
        let max = Rule1D::get_max_nb(self.n_colors);
        let filtered = if rule_nb >= max { max } else { rule_nb };
        if filtered != self.rule_nb {
            self.rule_nb = filtered;
            self.clean = false;
            self.tx
                .as_ref()
                .unwrap()
                .send(Message::SetRuleNb(self.rule_nb))
                .unwrap();
        }
    }
    fn set_width(&mut self, width: i32) {
        if width != self.width {
            self.width = width;
            self.clean = false;
            self.tx
                .as_ref()
                .unwrap()
                .send(Message::SetWidth(self.width))
                .unwrap();
        }
    }
    fn set_height(&mut self, height: i32) {
        if height != self.height {
            self.height = height;
            self.clean = false;
            self.tx
                .as_ref()
                .unwrap()
                .send(Message::SetHeight(self.height))
                .unwrap();
        }
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

    let n_colors_combo: gtk::ComboBox = builder.get_object("n_colors_combo").unwrap();
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
    let play_img: gtk::Image = builder.get_object("icon_play").unwrap();
    let pause_img: gtk::Image = builder.get_object("icon_pause").unwrap();
    let step_nb_label: gtk::Label = builder.get_object("step_nb_label").unwrap();

    let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_HIGH);
    {
        let mut m = model.lock().unwrap();
        m.set_tx(tx.clone());
    }

    n_colors_combo.connect_changed(clone!(@weak model => move |combo| {
        let val = combo.get_active().unwrap()+2;
        let mut m = model.lock().unwrap();
        m.set_n_colors(val as u8);
    }));
    rule_nb_entry.connect_changed(clone!(@weak model => move |entry| {
        let text = filter_integer(&entry);
        let val = text.parse::<u64>().unwrap_or(0);
        let mut m = model.lock().unwrap( );
        m.set_rule_nb(val);
    }));
    height_entry.connect_changed(clone!(@weak model => move |entry| {
        let text = filter_integer(&entry);
        let val = text.parse::<i32>().unwrap_or(0);
        let mut m = model.lock().unwrap();
        m.set_height(val);
    }));
    width_entry.connect_changed(clone!(@weak model => move |entry| {
        let text = filter_integer(&entry);
        let val = text.parse::<i32>().unwrap_or(0);
        let mut m = model.lock().unwrap();
        m.set_width(val);
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
    play_btn.connect_clicked(clone!(@weak model => move |_| {
        let mut m = model.lock().unwrap();
        m.switch_playing();
    }));
    reset_btn.connect_clicked(clone!(@weak model => move |_| {
        let mut m = model.lock().unwrap();
        m.reset();
    }));

    thread::spawn(move || loop {
        {
            let mut m = model.lock().unwrap();
            m.play(2);
        }
        thread::sleep(Duration::from_millis(5));
    });
    rx.attach(None, move |msg| {
        match msg {
            Message::UpdatePlayButton(play) => {
                play_btn.set_image(Some(if play { &pause_img } else { &play_img }));
            }
            Message::ResetDrawing(width, height) => {
                let pixbuf = Pixbuf::new(Colorspace::Rgb, false, 8, width, height)
                    .expect("Cannot create the Pixbuf!");
                pixbuf.fill(0xFFFFFFFF);
                display_img.set_from_pixbuf(Some(&pixbuf));
            }
            Message::SetRuleNb(value) => {
                rule_nb_entry.set_text(&value.to_string());
            }
            Message::SetNColors(value) => {
                n_colors_combo.set_active(Some((value - 2) as u32));
            }
            Message::SetWidth(value) => {
                width_entry.set_text(&value.to_string());
            }
            Message::SetHeight(value) => {
                height_entry.set_text(&value.to_string());
            }
            Message::SetStepNb(value) => {
                step_nb_label.set_text(&value.to_string());
            }
            Message::DrawStripe(row, width, height, rgb_vec) => {
                let pixbuf = display_img.get_pixbuf().unwrap();
                let mut real_row = row;
                if row + height > pixbuf.get_height() {
                    pixbuf.copy_area(
                        0,
                        height,
                        width,
                        pixbuf.get_height() - height,
                        &pixbuf,
                        0,
                        0,
                    );
                    real_row = pixbuf.get_height() - height;
                }
                for (i, &(r, g, b)) in rgb_vec.iter().enumerate() {
                    let x = (i as i32) % width;
                    let y = (i as i32) / width + real_row;
                    pixbuf.put_pixel(x, y, r, g, b, 0);
                }
                display_img.set_from_pixbuf(Some(&pixbuf));
                display_img.queue_draw();
            }
        };
        glib::Continue(true)
    });
    //continuous_chk.set_active(false);
    //width_entry.set_text("11");
    //height_entry.set_text("5");
    //rule_nb_entry.set_text("30");
    window.show_all();
}
