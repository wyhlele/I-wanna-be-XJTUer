mod kid;
mod camera;
mod movement;
mod schedule;
mod state;
mod ground;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use ground::GroundPlugin;
use kid::KidPlugin;
use camera::CameraPlugin;
use movement::MovementPlugin;
use schedule::SchedulePlugin;
use state::StatePlugin;


fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(1.0,1.0,1.0)))
        .add_plugins(
            DefaultPlugins.set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "I Wanna Be XJTUer".into(),
                    resolution: (800., 600.).into(),
                    resizable: true,
                    decorations: true,
                    ..default()
                }),
                ..default()
            })
        )
        .add_plugins(KidPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(SchedulePlugin)
        .add_plugins(StatePlugin)
        .add_plugins(GroundPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .run();
}


