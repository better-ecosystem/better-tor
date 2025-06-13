mod ui;
mod tor;
mod network;

use gtk4::prelude::*;
use gtk4::Application;

const APP_ID: &str = "com.better-ecosystem.tor";

#[tokio::main]
async fn main() {
    libadwaita::init().unwrap();
    let app = Application::builder()
        .application_id(APP_ID)
        .build();
    app.connect_activate(ui::build_ui as fn(&gtk4::Application));
    app.run();
}
