use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use bevy::sprite::Sprite;

use crate::asset_loader::ImageAssets;
use crate::kid::Kid;
use crate::kid_saver::KidSaver;
use crate::schedule::InGameSet;

#[derive(Component, Debug, Default)]
pub struct SavePointer{
    id: u8,
    position: Vec2,
}

pub struct SavePointerPlugin;

impl Plugin for SavePointerPlugin{
    fn build(&self,app: &mut App){
        app.add_systems(Startup,create_savepointer)
            .add_systems(Update,do_save.in_set(InGameSet::SaveSpawnPoint));
    }
}

fn create_savepointer(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
){
    let layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 2, 1, None, None);
    let atlas_layout = texture_atlases.add(layout);
    let atlas = TextureAtlas{
        layout : atlas_layout,
        index : 0,
    };
    commands.spawn((
        Sprite{
            image: image_assets.save.clone(),
            texture_atlas:Some(atlas.clone()),
            ..Default::default()
        },
        SavePointer{
            id: 1,
            position: Vec2::new(192.0, -32.0)
        },
    )).insert(
        Transform::from_xyz(192.0,-32.0,-0.1)
    ).insert(
        RigidBody::Dynamic
    ).insert(
        Collider::cuboid(14.0,16.0)
    ).insert(
        GravityScale(0.0)
    ).insert(
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

fn do_save(
    mut collision_events: EventReader<CollisionEvent>,
    kid_query: Query<&Kid>,
    savepointer_query: Query<&SavePointer>,
    mut sprite_query: Query<&mut Sprite>,
    mut kid_saver: ResMut<KidSaver>,
){
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity_a, entity_b, _) => {
                let is_entity1_b = kid_query.get(*entity_b).is_ok();
                let is_entity2_a = savepointer_query.get(*entity_a).is_ok();
                if is_entity1_b && is_entity2_a{
                    let pointer = savepointer_query.get(*entity_a).unwrap();
                    if pointer.id == 0 || pointer.id>kid_saver.save_id{
                        kid_saver.save_id = pointer.id;
                        kid_saver.position = pointer.position;
                    }
                    let mut sprite = sprite_query.get_mut(*entity_a);
                    if let Ok(ssprite) = &mut sprite{
                        if let Some(atlas) = &mut ssprite.texture_atlas{
                            atlas.index = 1;
                        }
                    }
                }

                let is_entity1_a = kid_query.get(*entity_a).is_ok();
                let is_entity2_b = savepointer_query.get(*entity_b).is_ok();
                if is_entity1_a && is_entity2_b{
                    let pointer = savepointer_query.get(*entity_b).unwrap();
                    if pointer.id == 0 || pointer.id>kid_saver.save_id{
                        kid_saver.save_id = pointer.id;
                        kid_saver.position = pointer.position;
                    }
                    let mut sprite = sprite_query.get_mut(*entity_b);
                    if let Ok(ssprite) = &mut sprite{
                        if let Some(atlas) = &mut ssprite.texture_atlas{
                            atlas.index = 1;
                        }
                    }
                }
            }
            CollisionEvent::Stopped(entity_a, entity_b, _) => {
                let is_entity1_b = kid_query.get(*entity_b).is_ok();
                let is_entity2_a = savepointer_query.get(*entity_a).is_ok();
                if is_entity1_b && is_entity2_a{
                    let mut sprite = sprite_query.get_mut(*entity_a);
                    if let Ok(ssprite) = &mut sprite{
                        if let Some(atlas) = &mut ssprite.texture_atlas{
                            atlas.index = 0;
                        }
                    }
                }

                let is_entity1_a = kid_query.get(*entity_a).is_ok();
                let is_entity2_b = savepointer_query.get(*entity_b).is_ok();
                if is_entity1_a && is_entity2_b{
                    let mut sprite = sprite_query.get_mut(*entity_b);
                    if let Ok(ssprite) = &mut sprite{
                        if let Some(atlas) = &mut ssprite.texture_atlas{
                            atlas.index = 0;
                        }
                    }
                }
            }
        }
    }
}

