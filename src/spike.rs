use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use bevy::sprite::Sprite;

use crate::asset_loader::ImageAssets;
use crate::trap::Trap;

const TO_RAD :f32 = 3.1415926 / 180.0;
pub struct SpikePlugin;

impl Plugin for SpikePlugin{
    fn build(&self, app: &mut App){
        app.add_systems(PostStartup, spawn_spike);
    }
}

fn spawn_single_spike(
    commands: &mut Commands,
    sprtie: &Handle<Image>,
    x: f32,y: f32,
    angle: f32,
) -> Entity{
    commands.spawn((
        Sprite{
            image: sprtie.clone(),
            ..Default::default()
        },
        Trap,
    )).insert(
        Transform{
            translation: Vec3::new(x,y,0.0),
            rotation: Quat::from_rotation_z(angle * TO_RAD),
            ..default()
        }
    ).insert(
        Velocity::zero()
    ).insert(
        RigidBody::Dynamic
    ).insert(
        GravityScale(0.0)
    ).insert(
        Collider::triangle(
            Vec2::new(-16.0, -16.0),Vec2::new(16.0, -16.0),Vec2::new(0.0, 16.0)
        )
    ).insert(
        CollisionGroups::new(
            Group::GROUP_3,
            Group::GROUP_1|Group::GROUP_4,
        )
    ).insert(SolverGroups::new(
        Group::GROUP_3,
        Group::GROUP_1,
        )
    ).id()
}

fn spawn_spike(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
){
    let image = image_assets.spike.clone();
    spawn_single_spike(&mut commands, &image, 64.0, -32.0, 0.0);
    spawn_single_spike(&mut commands, &image, -64.0, -32.0, -90.0);
    spawn_single_spike(&mut commands, &image, 800.0, -32.0, 0.0);
}


