#![warn(clippy::all, clippy::nursery)]
#![allow(clippy::single_match)]

use bevy::{input::system::exit_on_esc_system, prelude::*};

mod mesh_generator;

fn setup(
    commands: &mut Commands,
) {
    let camera = control::camera::CameraBundle::default();

    commands
        .spawn(camera);
}

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(control::input::InputPlugin)
        .add_startup_system(setup.system())
        .add_system(exit_on_esc_system.system())
        .run();
}

