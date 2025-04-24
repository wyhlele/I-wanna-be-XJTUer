use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use bevy::sprite::Sprite;

use crate::asset_loader::ImageAssets;
use crate::base::kid::{Kid,spawn_single_kid};
use crate::schedule::InGameSet;

#[derive(Component, Debug, Default)]
pub struct Warp{
    position: Vec2,
}

pub struct WarpPlugin;

impl Plugin for WarpPlugin{
    fn build(&self,app: &mut App){
        app.add_systems(PostStartup, spawn_warp)
            .add_systems(Update,do_trans.in_set(InGameSet::SaveSpawnPoint))
            .add_systems(Update, update_warp.in_set(InGameSet::EntityUpdates));
    }
}

#[derive(Resource)]
pub struct AnimationTimer(Timer);

pub fn spawn_single_warp(
    commands: &mut Commands,
    sprtie: &Handle<Image>,
    atlas: &TextureAtlas,
    x: f32, y: f32,
    tox: f32, toy: f32,
)-> Entity{
    commands.spawn((
        Sprite{
            image: sprtie.clone(),
            texture_atlas:Some(atlas.clone()),
            ..Default::default()
        },
        Warp{
            position: Vec2::new(tox, toy)
        },
    )).insert(
        Transform::from_xyz(x,y,-0.2)
    ).insert(
        RigidBody::Dynamic
    ).insert(
        Collider::cuboid(12.0,13.0)
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
    ).id()
}

fn spawn_warp(
    mut commands: Commands,
){
    commands.insert_resource(AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)));
}

fn do_trans(
    mut collision_events: EventReader<CollisionEvent>,
    kid_query: Query<&Kid>,
    warp_query: Query<&Warp>,
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
){
    let layout = TextureAtlasLayout::from_grid(UVec2::new(25, 24), 4, 6, None, None);
    let atlas_layout = texture_atlases.add(layout);
    let atlas = TextureAtlas{
        layout : atlas_layout,
        index : 0,
    };
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity_a, entity_b, _) => {
                let is_entity1_b = kid_query.get(*entity_b).is_ok();
                let is_entity2_a = warp_query.get(*entity_a).is_ok();
                let is_entity1_a = kid_query.get(*entity_a).is_ok();
                let is_entity2_b = warp_query.get(*entity_b).is_ok();

                if is_entity1_b && is_entity2_a{
                    let pointer = warp_query.get(*entity_a).unwrap();
                    commands.entity(*entity_b).despawn_recursive();
                    spawn_single_kid(&mut commands,&image_assets.kid,&atlas,pointer.position.x,pointer.position.y);
                    
                }else if is_entity1_a && is_entity2_b{
                    let pointer = warp_query.get(*entity_b).unwrap();
                    commands.entity(*entity_a).despawn_recursive();
                    spawn_single_kid(&mut commands,&image_assets.kid,&atlas,pointer.position.x,pointer.position.y);

                }
                
            }
            _ => {}
        }
    }
}


fn update_warp(
    mut query: Query<&mut Sprite,With<Warp>>,
    time: Res<Time>,
    mut timer: ResMut<AnimationTimer>,
){
    timer.0.tick(time.delta());
    if timer.0.finished() {
        for mut sprite in query.iter_mut(){
            if let Some(atlas) = &mut sprite.texture_atlas{
                atlas.index = (atlas.index + 1) % 4;
            }
        }
    }
}
