use gtk::prelude::*;
use gtk::{FileChooserDialog, FileChooserAction, ResponseType};

pub fn open_file_dialog(parent: &gtk::Window) -> Option<String> {
    let dialog = FileChooserDialog::new(Some("Select a Document"), Some(parent), FileChooserAction::Open);
    dialog.add_button("Cancel", ResponseType::Cancel);
    dialog.add_button("Open", ResponseType::Accept);
    dialog.show_all();

    let response = dialog.run();
    let filename = if response == ResponseType::Accept {
        dialog.get_filename().map(|f| f.to_string_lossy().into_owned())
    } else {
        None
    };
    dialog.close();
    filename
}