
use bevy_rapier2d::prelude::*;
use bevy::prelude::*;

use bevy::sprite::{TextureAtlas, TextureAtlasLayout,Sprite};

use crate::asset_loader::ImageAssets;
use crate::kid::Kid;
use crate::state::{GameState, NeedReload};

pub struct HiddenPlugin;

#[derive(Component, Debug)]
struct HiddenWall;

impl Plugin for HiddenPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(OnExit(GameState::Reload),spawn_hidden)
        .add_systems(Update,update_hidden);
    }
}

fn spawn_hidden(
    mut commands: Commands,  
){
    commands.spawn(
        RigidBody::Fixed
    ).insert(
        GravityScale(0.0)
    ).insert(
        Transform::from_xyz(512.0,64.0,0.0)
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
    ).insert(HiddenWall)
    .insert(NeedReload);
}

fn update_hidden(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    kid_query: Query<&Kid>,
    hidden_query: Query<&HiddenWall>,
    image_assets: Res<ImageAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
){
    let image = image_assets.bg.clone();
    let layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 6, 9, None, None);
    let atlas_layout = texture_atlases.add(layout);
    let atlas = TextureAtlas{
        layout : atlas_layout,
        index : 39,
    };
    let sprite = Sprite{
        image: image.clone(),
        texture_atlas: Some(atlas.clone()),
        ..Default::default()
    };


    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity_a, entity_b, _) => {
                let is_entity1_b = kid_query.get(*entity_b).is_ok();
                let is_entity2_a = hidden_query.get(*entity_a).is_ok();
                if is_entity1_b && is_entity2_a{
                    commands.entity(*entity_a).insert(sprite.clone());
                }

                let is_entity1_a = kid_query.get(*entity_a).is_ok();
                let is_entity2_b = hidden_query.get(*entity_b).is_ok();
                if is_entity1_a && is_entity2_b{
                    commands.entity(*entity_b).insert(sprite.clone());
                }
            }
            _ => {}
        }
    }
}