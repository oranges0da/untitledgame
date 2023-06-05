use bevy::prelude::*;

fn main() {
    App::new().add_startup_system(setup).run();
}

fn setup(commands: Commands) {
    println!("Hello World!");
}
