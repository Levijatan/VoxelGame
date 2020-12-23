use bevy::{prelude::AppBuilder, app::{
        EventReader,
        Events,
    }, ecs::{Res, ResMut}, input::{
        keyboard::{
            KeyboardInput,

        },
        mouse::MouseMotion,
    }, prelude::Plugin};
use bevy::prelude::IntoSystem;

#[derive(Default)]
struct State {
    keys: EventReader<KeyboardInput>,
    motion: EventReader<MouseMotion>,
}

fn system(
    mut state: ResMut<State>,
    mut camera_controller: ResMut<super::camera::Controller>,
    ev_keys: Res<Events<KeyboardInput>>,
    ev_motion: Res<Events<MouseMotion>>,
) {
    
    for ev in state.keys.iter(&ev_keys) {
        camera_controller.process_keyboard(ev);
    }

    for ev in state.motion.iter(&ev_motion) {
        camera_controller.process_mouse(ev);
    }
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<State>()
            .init_resource::<super::camera::Controller>()
            .add_system_to_stage(bevy::app::stage::PRE_UPDATE, system.system())
            .add_system_to_stage(bevy::app::stage::PRE_UPDATE, super::camera::update_camera_system.system());
    }
}