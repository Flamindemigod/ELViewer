use glib::ExitCode;
use gtk::Application;
use gtk::prelude::*;

use super::{channel::ClientChannel, ui::build_ui};

pub async fn spawn_frontend(app_id: &str, mut client: ClientChannel) -> ExitCode {
    let app = Application::builder().application_id(app_id).build();
    app.connect_activate(build_ui);
    // Run the application
    app.run()
}
