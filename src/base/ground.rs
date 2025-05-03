use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn spawn_single_box(
    commands: &mut Commands,
    x: f32, y: f32,
    bx: f32, by: f32,
    rx: f32, ry: f32,
){
    // bx + 32*x, by + 32*y spawn a 32*rx,32*ry box
    commands.spawn(
        RigidBody::Fixed
    ).insert(
        GravityScale(0.0)
    ).insert(
        Transform::from_xyz(bx + 32.*x,by + 32.*y,0.0)
    ).insert(
        Collider::cuboid(32.*rx, 32.*ry)
    ).insert(
        LockedAxes::ROTATION_LOCKED
    ).insert(
        CollisionGroups::new(
            Group::GROUP_2,
            Group::GROUP_1|Group::GROUP_4,
        )
    ).insert(SolverGroups::new(
        Group::GROUP_2,
        Group::GROUP_1,
    )).insert(Friction::coefficient(0.0))
    .insert(Restitution::coefficient(0.0));
}