use crate::entities::environment::{create_environment, Environment};

pub mod entities;
fn main() {
    let bob = create_environment(Some("Environment 1"), 0.0, 20.0, 0.0);
    println!("Hello, world! {} was created successfully!", bob.get_name());
}
