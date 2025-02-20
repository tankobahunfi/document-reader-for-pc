use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Box, FileChooserDialog, FileChooserAction, TextView, ScrolledWindow};
use std::fs::File;
use std::io::{self, Read};

fn main() {
    let app = Application::new(Some("com.example.documentreader"), Default::default());
    app.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Document Reader");
        window.set_default_size(800, 600);

        let vbox = Box::new(gtk::Orientation::Vertical, 5);
        let open_button = Button::with_label("Open Document");
        let text_view = TextView::new();
        let scrolled_window = ScrolledWindow::new(None, None);
        scrolled_window.add(&text_view);
        vbox.pack_start(&open_button, false, false, 0);
        vbox.pack_start(&scrolled_window, true, true, 0);
        window.add(&vbox);

        open_button.connect_clicked(move |_| {
            let dialog = FileChooserDialog::new(Some("Select a Document"), Some(&window), FileChooserAction::Open);
            dialog.add_button("Cancel", gtk::ResponseType::Cancel);
            dialog.add_button("Open", gtk::ResponseType::Accept);
            dialog.show_all();

            dialog.connect_response(move |dialog, response| {
                if response == gtk::ResponseType::Accept {
                    let filename = dialog.get_filename().unwrap();
                    let content = read_file(&filename);
                    text_view.get_buffer().unwrap().set_text(&content);
                }
                dialog.close();
            });
        });

        window.show_all();
    });

    app.run();
}

fn read_file(path: &std::path::Path) -> String {
    let mut file = File::open(path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content
}