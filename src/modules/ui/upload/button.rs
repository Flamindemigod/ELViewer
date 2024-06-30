use std::{fs, io::BufRead};

use gio::{Cancellable, ListStore};
use gtk::{
    glib::clone, prelude::*, Box, Button, FileChooserAction, FileDialog, FileFilter, ProgressBar,
    ResponseType, Widget, Window,
};

pub fn upload_button<W: IsA<Window>>(window: W) {
    let filter = FileFilter::new();
    filter.set_name(Some("TESO Encounter Log"));
    filter.add_suffix("log");
    filter.add_mime_type("text/plain");
    let filter_list = ListStore::new::<FileFilter>();
    filter_list.append(&filter);
    let file_chooser = FileDialog::builder()
        .modal(true)
        .title("Choose Encounter Log to View")
        .filters(&filter_list)
        .default_filter(&filter)
        .build();

    let button = Button::builder()
        .label("Upload Log")
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .build();

    button.connect_clicked(clone!(@strong file_chooser, @strong window=> move |_| file_chooser.open(
                Some(Window::new()).as_ref(),
                Some(Cancellable::new()).as_ref(),
                clone!(@strong window => move |res|{
                    if let Ok(file) = res{
                        let pb = ProgressBar::builder().orientation(gtk::Orientation::Horizontal).halign(gtk::Align::Fill).valign(gtk::Align::Center).margin_end(12).margin_start(12).build();
                        window.set_child(Some(&pb));
                        // let _ = tokio::spawn(parse_file(file.path().unwrap()));
                        // println!("File: {file:#?}");
                        // let out = handle.await;
                    }
                })

    )));
    window.set_child(Some(&button));
}
