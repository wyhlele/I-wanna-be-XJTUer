use bevy::{ecs::query, prelude::*, transform};

// const CAMERA_DISTANCE: f32 = 5.0;

pub struct CameraPlugin;

impl Plugin for CameraPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Startup,spawn_cacmera);
    }
}

fn spawn_cacmera(mut commands: Commands){
    commands.spawn(Camera2d::default());
}
