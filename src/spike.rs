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

fn spawn_spike(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
){
    commands.spawn((
        Sprite{
            image: image_assets.spike.clone(),
            ..Default::default()
        },
        Trap,
    )).insert(
        Transform::from_xyz(64.0,-32.0,0.0)
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
    );

    commands.spawn((
        Sprite{
            image: image_assets.spike.clone(),
            ..Default::default()
        },
        Trap,
    )).insert(
        Transform{
            translation: Vec3::new(-64.0,-32.0,0.0),
            rotation: Quat::from_rotation_z(-90.0 * TO_RAD),
            ..default()
        }
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
    );
}


