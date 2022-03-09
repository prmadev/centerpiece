mod app;
mod components;
mod stores;
mod structs;

use gtk4::{prelude::*, Application};

fn main() {
    let application = Application::builder()
        .application_id("com.github.friedow.tucan-search")
        .build();
    application.connect_activate(app::build);
    application.run();
}
