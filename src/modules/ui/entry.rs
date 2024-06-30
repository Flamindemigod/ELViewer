use crate::modules::ui::upload::upload_button;
use gtk::{prelude::*, Application, ApplicationWindow};

pub fn build_ui(app: &Application) {
    let window = ApplicationWindow::new(app);
    upload_button(window.clone());
    window.present();
}
