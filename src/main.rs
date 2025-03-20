mod kid;
mod camera;
mod movement;

use bevy::prelude::*;
use kid::KidPlugin;
use camera::CameraPlugin;
use movement::MovementPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(1.0,1.0,1.0)))
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(KidPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(MovementPlugin)
        .run();
}


