use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, glib};
use crate::entities::environment::create_environment;
use crate::ui::ui::build_ui;

const APP_ID: &str = "org.gtk_rs.HelloWorld2";

pub mod entities;
mod ui;

fn main() -> glib::ExitCode{
    let bob = create_environment(Some("Environment 1"), 0.0, 20.0, 0.0);
    println!("Hello, world! {} was created successfully!", bob.get_name());

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}
