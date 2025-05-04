
use bevy_rapier2d::prelude::*;
use bevy::prelude::*;

use bevy::sprite::{TextureAtlas,Sprite};

use crate::asset_loader::MusicAssets;
use crate::base::kid::Kid;
use crate::state::NeedReload;

pub struct HiddenPlugin;

#[derive(Component, Debug)]
struct HiddenWall{
    image: Handle<Image>,
    atlas: TextureAtlas,
    used: bool,
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
        used: false,
    })
    .insert(NeedReload).id()
}

fn update_hidden(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    kid_query: Query<&Kid>,
    mut hidden_query: Query<&mut HiddenWall>,
    music_assets: Res<MusicAssets>,
){
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity_a, entity_b, _) => {
                let is_entity1_b = kid_query.get(*entity_b).is_ok();
                let is_entity2_a = hidden_query.get(*entity_a).is_ok();
                let is_entity1_a = kid_query.get(*entity_a).is_ok();
                let is_entity2_b = hidden_query.get(*entity_b).is_ok();
                if is_entity1_b && is_entity2_a{
                    let Ok(mut hidden) = hidden_query.get_mut(*entity_a)
                    else{
                        continue;
                    };
                    if hidden.used == false{
                        commands.entity(*entity_a).insert(Sprite{
                            image: hidden.image.clone(),
                            texture_atlas: Some(hidden.atlas.clone()),
                            ..Default::default()
                        });
                        commands.spawn(AudioPlayer::new(music_assets.trap.clone())).insert(NeedReload);
                        hidden.used = true;
                    }
                }else if is_entity1_a && is_entity2_b{
                    let Ok(mut hidden) = hidden_query.get_mut(*entity_b)
                    else{
                        continue;
                    };
                    if hidden.used == false{
                        commands.entity(*entity_b).insert(Sprite{
                            image: hidden.image.clone(),
                            texture_atlas: Some(hidden.atlas.clone()),
                            ..Default::default()
                        });
                        commands.spawn(AudioPlayer::new(music_assets.trap.clone())).insert(NeedReload);
                        hidden.used = true;
                    }
                }
            }
            _ => {}
        }
    }
}