
use bevy_rapier2d::prelude::*;
use bevy::prelude::*;

use bevy::sprite::{TextureAtlas,Sprite};

use crate::base::kid::Kid;
use crate::state::NeedReload;

pub struct HiddenPlugin;

#[derive(Component, Debug)]
struct HiddenWall{
    image: Handle<Image>,
    atlas: TextureAtlas,
}

impl Plugin for HiddenPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Update,update_hidden);
    }
}

pub fn spawn_single_hidden(
    commands: &mut Commands,
    image: &Handle<Image>,
    atlas: &TextureAtlas,
    x: f32, y: f32,
    bx: f32, by: f32,
)->Entity{
    commands.spawn(
        RigidBody::Fixed
    ).insert(
        GravityScale(0.0)
    ).insert(
        Transform::from_xyz(bx+32.*x,by+32.*y,0.0)
    ).insert(
        Collider::cuboid(16.0, 16.0)
    ).insert(
        LockedAxes::ROTATION_LOCKED
    ).insert(
        CollisionGroups::new(
            Group::GROUP_2,
            Group::GROUP_1|Group::GROUP_4,
        )
    ).insert(
        SolverGroups::new(
        Group::GROUP_2,
        Group::GROUP_1,
        )
    ).insert(HiddenWall{
        image: image.clone(),
        atlas: atlas.clone(),
    })
    .insert(NeedReload).id()
}

fn update_hidden(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    kid_query: Query<&Kid>,
    hidden_query: Query<&HiddenWall>,
){
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity_a, entity_b, _) => {
                let is_entity1_b = kid_query.get(*entity_b).is_ok();
                let is_entity2_a = hidden_query.get(*entity_a).is_ok();
                let is_entity1_a = kid_query.get(*entity_a).is_ok();
                let is_entity2_b = hidden_query.get(*entity_b).is_ok();
                if is_entity1_b && is_entity2_a{
                    let hidden = hidden_query.get(*entity_a).unwrap();
                    commands.entity(*entity_a).insert(Sprite{
                        image: hidden.image.clone(),
                        texture_atlas: Some(hidden.atlas.clone()),
                        ..Default::default()
                    });
                }else if is_entity1_a && is_entity2_b{
                    let hidden = hidden_query.get(*entity_b).unwrap();
                    commands.entity(*entity_b).insert(Sprite{
                        image: hidden.image.clone(),
                        texture_atlas: Some(hidden.atlas.clone()),
                        ..Default::default()
                    });
                }
            }
            _ => {}
        }
    }
}