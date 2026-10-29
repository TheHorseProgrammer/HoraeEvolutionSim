use crate::entities::environment::create_environment;
use crate::ui::ui::run_app;

pub mod entities;
mod ui;

fn main() {
    let bob = create_environment(Some("Environment 1"), 0.0, 20.0, 0.0);
    println!("Hello, world! {} was created successfully!", bob.get_name());
    run_app();
}
