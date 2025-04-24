use bevy_rapier2d::prelude::*;
use bevy::prelude::*;

use bevy::sprite::Sprite;
use crate::base::kid::Kid;
use crate::state::{GameState, NeedReload};


#[derive(Resource, Debug, Default)]
pub struct LeafNum{
    pub num: u8,
}

#[derive(Component, Debug)]
pub struct Leaf{
    pub score: u8,
}
pub struct LeafPlugin;

impl Plugin for LeafPlugin{
    fn build(&self,app: &mut App){
        app.init_resource::<LeafNum>()
        .add_systems(OnExit(GameState::Reload),init_num)
        .add_systems(Update,do_touch);
    }
}

fn init_num(
    mut leaf_num: ResMut<LeafNum>,
){
    leaf_num.num = 0;
}

pub fn spawn_single_leaf(
    commands: &mut Commands,
    sprtie: &Handle<Image>,
    x: f32,y: f32,
    bx: f32, by:f32,
    score: u8,
)->Entity{
    commands.spawn((
        Sprite{
            image: sprtie.clone(),
            ..Default::default()
        },
        Leaf{
            score: score
        }
    )).insert(
        RigidBody::Fixed
    ).insert(
        GravityScale(0.0)
    ).insert(
        Transform::from_xyz(bx+x*32.,by+y*32.,-0.2)
    ).insert(
        Collider::cuboid(10.0, 10.0)
    ).insert(
        LockedAxes::ROTATION_LOCKED
    ).insert(
        CollisionGroups::new(
            Group::GROUP_4,
            Group::GROUP_1|Group::GROUP_4,
        )
    ).insert(SolverGroups::new(
        Group::GROUP_4,
        Group::NONE,
        )
    ).insert(NeedReload).id()
}


fn do_touch(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    kid_query: Query<&Kid>,
    leaf_query: Query<&Leaf>,
    mut leaf_num: ResMut<LeafNum>,
){
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity_a, entity_b, _) => {
                let is_entity1_b = kid_query.get(*entity_b).is_ok();
                let is_entity2_a = leaf_query.get(*entity_a).is_ok();
                let is_entity1_a = kid_query.get(*entity_a).is_ok();
                let is_entity2_b = leaf_query.get(*entity_b).is_ok();
                if is_entity1_b && is_entity2_a{
                    leaf_num.num += leaf_query.get(*entity_a).unwrap().score;
                    commands.entity(*entity_a).despawn_recursive();
                }else if is_entity1_a && is_entity2_b{
                    leaf_num.num += leaf_query.get(*entity_b).unwrap().score;
                    commands.entity(*entity_b).despawn_recursive();
                }
            }
            _ => {}
        }
    }
}



