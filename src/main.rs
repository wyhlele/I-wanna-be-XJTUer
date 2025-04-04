mod kid;
mod camera;
mod schedule;
mod state;
mod ground;
mod asset_loader;
mod kid_saver;
mod trap;
mod spike;
mod savepointer;
mod bullet;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use ground::GroundPlugin;
use kid::KidPlugin;
use camera::CameraPlugin;
use asset_loader::AssetLoaderPlugin;
use schedule::SchedulePlugin;
use state::StatePlugin;
use kid_saver::KidSaverPlugin;
use spike::SpikePlugin;
use savepointer::SavePointerPlugin;
use bullet::BulletPlugin;


fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(1.0,1.0,1.0)))
        .add_plugins(
            DefaultPlugins.set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "I Wanna Be XJTUer".into(),
                    resolution: (800., 608.).into(),
                    resizable: true,
                    decorations: true,
                    ..default()
                }),
                ..default()
            })
        )
        .add_plugins(KidPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(SchedulePlugin)
        .add_plugins(StatePlugin)
        .add_plugins(GroundPlugin)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(KidSaverPlugin)
        .add_plugins(SpikePlugin)
        .add_plugins(SavePointerPlugin)
        .add_plugins(BulletPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .run();
}


