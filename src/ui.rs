use gtk::prelude::*;
use gtk::{TextView, ScrolledWindow};

pub struct DocumentReaderUI {
    pub text_view: TextView,
}

impl DocumentReaderUI {
    pub fn new() -> Self {
        let text_view = TextView::new();
        let scrolled_window = ScrolledWindow::new(None, None);
        scrolled_window.add(&text_view);
        Self { text_view }
    }

    pub fn set_text(&self, text: &str) {
        self.text_view.get_buffer().unwrap().set_text(text);
    }
}