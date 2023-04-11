use gtk4::prelude::*;
use gio::prelude::*;
use glib::clone;

fn main() {
    let application = gtk4::Application::new(
        Some("com.example.gtk4_chat_app"),
        Default::default(),
    );

    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(app: &gtk4::Application) {
    let window = gtk4::ApplicationWindow::new(app);
    window.set_title(Some("Chat App"));
    window.set_default_size(600, 400);

    let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 5);
    window.set_child(Some(&main_box));

    let scroll = gtk4::ScrolledWindow::new();
    main_box.append(&scroll);

    let chat_box = gtk4::Box::new(gtk4::Orientation::Vertical, 5);
    scroll.set_child(Some(&chat_box));

    let entry_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 5);
    main_box.append(&entry_box);

    let entry = gtk4::Entry::new();
    entry_box.append(&entry);

    let send_button = gtk4::Button::with_label("Send");
    entry_box.append(&send_button);

    send_button.connect_clicked(clone!(@strong entry, @strong chat_box => move |_| {
        let text = entry.text().to_string();
        if !text.trim().is_empty() {
            let label = gtk4::Label::new(None);
            label.set_markup(&text);
            chat_box.append(&label);
            entry.set_text("");
        }
    }));

    window.present();
}

