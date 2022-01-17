use fltk::{prelude::*, *};
mod mainview;
mod mydialog;

fn main() {
    let app = app::App::default();

    let mut mainwin = mainview::UserInterface::make_window();
    let mut frame = mainwin.frame.clone();

    mainwin.btn.set_callback(move |_| {
        frame.set_label("Hello World!");
        let dialog = mydialog::Dialog::make_dialog();
        while dialog.dialog.shown() {
            app::wait();
        }
    });

    app.run().unwrap();
}
