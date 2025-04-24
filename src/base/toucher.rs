
use bevy_rapier2d::prelude::*;
use bevy::prelude::*;

use bevy::sprite::{TextureAtlas,Sprite};

use crate::base::kid::Kid;
use crate::state::NeedReload;

pub struct ToucherPlugin;

#[derive(Component, Debug)]
struct ToucherWall;

impl Plugin for ToucherPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Update,update_toucher);
    }
}

pub fn spawn_single_toucher(
    commands: &mut Commands,  
    sprtie: &Handle<Image>,
    atlas: &TextureAtlas,
    x: f32, y: f32,
    bx: f32, by: f32,
)-> Entity{
    commands.spawn((
        Sprite{
            image: sprtie.clone(),
            texture_atlas:Some(atlas.clone()),
            ..Default::default()
        },
        ToucherWall
    )).insert(
        RigidBody::Fixed
    ).insert(
        GravityScale(0.0)
    ).insert(
        Transform::from_xyz(bx+x*32.,by+y*32.,0.0)
    ).insert(
        Collider::cuboid(16.0, 16.0)
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

fn update_toucher(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    kid_query: Query<&Kid>,
    hidden_query: Query<&ToucherWall>,
){
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity_a, entity_b, _) => {
                let is_entity1_b = kid_query.get(*entity_b).is_ok();
                let is_entity2_a = hidden_query.get(*entity_a).is_ok();
                let is_entity1_a = kid_query.get(*entity_a).is_ok();
                let is_entity2_b = hidden_query.get(*entity_b).is_ok();
                if is_entity1_b && is_entity2_a{
                    commands.entity(*entity_a).despawn_recursive();
                }else if is_entity1_a && is_entity2_b{
                    commands.entity(*entity_b).despawn_recursive();
                }
            }
            _ => {}
        }
    }
}