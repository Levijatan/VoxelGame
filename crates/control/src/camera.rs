

use bevy::{
    core::Time,
    ecs::{ResMut, Res, Mut, Bundle},
    input::{keyboard::KeyboardInput, mouse::MouseMotion},
    ecs::Query, math::{Vec3, vec3},
    prelude::KeyCode, prelude::{
        Transform,
        GlobalTransform,
    },
    render::camera::{
        Camera,
        PerspectiveProjection,
        VisibleEntities,

    },
};
use std::ops::{Deref, DerefMut};

pub struct MainTag {}

#[derive(Default)]
pub struct Yaw(f32);

impl Deref for Yaw {
    type Target = f32;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Yaw {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Default)]
pub struct Pitch(f32);

impl Deref for Pitch {
    type Target = f32;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Pitch {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct Controller {
    amount_left: f32,
    amount_right: f32,
    amount_forward: f32,
    amount_backward: f32,
    amount_up: f32,
    amount_down: f32,
    rotate_horizontal: f32,
    rotate_vertical: f32,
    speed: f32,
    sensitivity: f32,
}

impl Default for Controller {
    fn default() -> Self {
        Self{
            amount_backward: 0.0,
            amount_down: 0.0,
            amount_forward: 0.0,
            amount_left: 0.0,
            amount_right: 0.0,
            amount_up: 0.0,
            rotate_vertical: 0.0,
            rotate_horizontal: 0.0,
            speed: 2.0,
            sensitivity: 1.0,
        }
    }
}

impl Controller {
    pub fn new(speed: f32, sensitivity: f32) -> Self {
        Self {
            speed,
            sensitivity,
            ..Self::default()
        }
    }

    pub fn process_keyboard(
        &mut self,
        val: &KeyboardInput,
    ) {
        let amount = if val.state.is_pressed() {
            1.0
        } else {
            0.0
        };

        match val.key_code {
            Some(KeyCode::W) => {
                self.amount_forward = amount;
            }
            Some(KeyCode::S) => {
                self.amount_backward = amount;
            }
            Some(KeyCode::A) => {
                self.amount_left = amount;
            }
            Some(KeyCode::D) => {
                self.amount_right = amount;
            }
            Some(KeyCode::Space) => {
                self.amount_up = amount;
            }
            Some(KeyCode::LShift) => {
                self.amount_down = amount;
            }
            _ => {}
        }
    }

    pub fn process_mouse(
        &mut self,
        val: &MouseMotion,
    ) {
        let delta = val.delta;
        self.rotate_horizontal = delta.x;
        self.rotate_vertical = delta.y;
    }
}

pub fn update_camera_system(
    mut controller: ResMut<Controller>,
    time: Res<Time>,
    mut query: Query<(&MainTag, Mut<Transform>, Mut<Yaw>, Mut<Pitch>)>
) {
    use std::f32::consts::FRAC_PI_2;
    for (_tag, mut transform, mut yaw, mut pitch) in query.iter_mut() {
        
        let (yaw_sin, yaw_cos) = yaw.sin_cos();
        let forward = vec3(yaw_cos, 0.0, yaw_sin).normalize();
        let right = vec3(-yaw_sin, 0.0, yaw_cos).normalize();
        transform.translation += forward * (controller.amount_forward - controller.amount_backward) * controller.speed * time.delta_seconds;
        transform.translation += right * (controller.amount_right - controller.amount_left) * controller.speed * time.delta_seconds;
        transform.translation.y += (controller.amount_up - controller.amount_down) * controller.speed * time.delta_seconds;

        yaw.0 += controller.rotate_horizontal * controller.sensitivity * time.delta_seconds;
        pitch.0 += -controller.rotate_vertical * controller.sensitivity * time.delta_seconds;
        
        if pitch.0 < -FRAC_PI_2 {
            pitch.0 = -FRAC_PI_2;
        } else if pitch.0 > FRAC_PI_2 {
            pitch.0 = FRAC_PI_2;
        }

        let pos = transform.translation;

        transform.look_at(pos + vec3(yaw.cos(), pitch.sin(), yaw.sin()).normalize(), Vec3::unit_y());

        controller.rotate_horizontal = 0.0;
        controller.rotate_vertical = 0.0;

        
    }
}

#[derive(Bundle)]
pub struct CameraBundle {
    pub camera: Camera,
    pub perspective_projection: PerspectiveProjection,
    pub visible_entities: VisibleEntities,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub yaw: Yaw,
    pub pitch: Pitch,
}

impl Default for CameraBundle {
    fn default() -> Self {
        let yaw_f: f32 = -90.0;
        let pitch_f: f32 = -20.0;
        let yaw = Yaw(yaw_f.to_radians());
        let pitch = Pitch(pitch_f.to_radians());
        Self {
            camera: Camera {
                name: Some(bevy::render::render_graph::base::camera::CAMERA3D.to_string()),
                ..Camera::default()
            },
            perspective_projection: PerspectiveProjection::default(),
            visible_entities: VisibleEntities::default(),
            transform: Transform::from_translation(vec3(0.0, 0.0, 0.0)).looking_at(vec3(0.0, 0.0, 0.0) + vec3(yaw.cos(), pitch.sin(), yaw.sin()).normalize(), Vec3::unit_y()),
            global_transform: GlobalTransform::default(),
            yaw,
            pitch,
        }
    }
}