mod modules;
use gtk::prelude::*;
use gtk::{glib, Application};
use std::{fs::File, io::BufRead};

use modules::ui::build_ui;

const APP_ID: &str = "org.flamindemigod.ElViewer";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    // Run the application
    app.run()

    // let file = File::open("Encounter.log").unwrap();
    // println!("{:#?}", lexer.data.lines().take(5).collect::<Vec<_>>());
    // for line in mapped_file.lines().take(20) {
    //     println!("{}", line?);
    // }
    // println!("{file_size}");
}
