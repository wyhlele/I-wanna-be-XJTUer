use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use bevy::sprite::Sprite;

use crate::base::trap::Trap;

const TO_RAD :f32 = 3.1415926 / 180.0;

pub fn spawn_single_spike(
    commands: &mut Commands,
    sprtie: &Handle<Image>,
    x: f32,y: f32,
    bx: f32, by: f32,
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
            translation: Vec3::new(x*32.+bx,y*32.+by,-0.3),
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
            Vec2::new(-15.5, -16.0),Vec2::new(15.5, -16.0),Vec2::new(0.0, 16.0)
        )
    ).insert(
        ColliderMassProperties::Mass(10000.0)
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

pub fn spawn_single_spike_fixed(
    commands: &mut Commands,
    sprtie: &Handle<Image>,
    x: f32,y: f32,
    bx: f32, by: f32,
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
            translation: Vec3::new(x*32.+bx,y*32.+by,-0.3),
            rotation: Quat::from_rotation_z(angle * TO_RAD),
            ..default()
        }
    ).insert(
        RigidBody::Fixed
    ).insert(
        Collider::triangle(
            Vec2::new(-15.5, -16.0),Vec2::new(15.5, -16.0),Vec2::new(0.0, 16.0)
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
