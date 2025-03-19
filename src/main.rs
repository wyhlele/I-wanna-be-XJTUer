mod kid;
mod camera;
mod movement;

use bevy::prelude::*;
use kid::KidPlugin;
use camera::CameraPlugin;
use movement::MovementPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(1.0,1.0,1.0)))
        .add_plugins(KidPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // 禁用线性过滤
        .run();
}
