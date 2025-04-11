use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::kid::Kid;
use crate::moveto::Move;

pub struct TestPlugin;

#[derive(Component, Debug)]
pub struct MySensor;

impl Plugin for TestPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(PostStartup, spawn)
            .add_systems(Update,check);
    }
}

fn spawn(
    mut commands: Commands,
){
    commands.spawn(Collider::cuboid(16.0, 100.0))
    .insert(Transform::from_xyz(300.0, 0.0, 0.0))
    .insert(MySensor)
    .insert(
        CollisionGroups::new(
            Group::GROUP_4,
            Group::GROUP_1|Group::GROUP_4,
        )
    ).insert(SolverGroups::new(
        Group::GROUP_4,
        Group::NONE,
        )
    );
}

fn check(
    mut collision_events: EventReader<CollisionEvent>,
    kid_query: Query<&Kid>,
    mut move_query: Query<&mut Move>,
    sensor_query: Query<&MySensor>,
){
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity_a, entity_b, _) => {
                let is_entity1_b = kid_query.get(*entity_b).is_ok();
                let is_entity2_a = sensor_query.get(*entity_a).is_ok();
                if is_entity1_b && is_entity2_a{
                    let Ok(mut mover) = move_query.get_single_mut()
                    else {
                        continue;
                    };
                    if mover.status == 0{
                        mover.goal_pos = Vec2::new(320.0,-300.0);
                        mover.status = 1;
                    }
                }

                let is_entity1_a = kid_query.get(*entity_a).is_ok();
                let is_entity2_b = sensor_query.get(*entity_b).is_ok();
                if is_entity1_a && is_entity2_b{
                    let Ok(mut mover) = move_query.get_single_mut()
                    else {
                        continue;
                    };
                    if mover.status == 0{
                        mover.goal_pos = Vec2::new(320.0,-300.0);
                        mover.status = 1;
                    }
                }
            }
            _ => {}
        }
    }
}