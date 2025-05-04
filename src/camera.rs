use bevy::prelude::*;

use crate::asset_loader::ImageAssets;
use crate::base::kid::Kid;
use crate::kid_saver::KidSaver;
use crate::schedule::InGameSet;
use crate::state::GameState;

#[derive(Component, Default)]
pub struct GameOver;

pub struct CameraPlugin;

impl Plugin for CameraPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Startup,spawn_camera)
        .add_systems(Update, camera_follow.in_set(InGameSet::CameraFollowed))
        .add_systems(OnEnter(GameState::GameOver), gameover_text_spawn)
        .add_systems(OnExit(GameState::GameOver), gameover_text_despawn);
    }
}

fn spawn_camera(mut commands: Commands){
    commands.spawn(Camera2d::default());
}

fn camera_follow(
    mut camera_query: Query<&mut Transform,(With<Camera2d>, Without<Kid>)>,
    kid_query: Query<&Transform,With<Kid>>,
){
    let Ok(mut camera) = camera_query.get_single_mut()
    else{
        return;
    };
    let Ok(kid) = kid_query.get_single()
    else{
        return;
    };
    camera.translation.x = ((kid.translation.x + 400.0)/800.0).floor() * 800.0;
    camera.translation.y = ((kid.translation.y + 304.0)/608.0).floor() * 608.0;
}

fn gameover_text_spawn(
    mut commands: Commands,
    camera_query: Query<&mut Transform,With<Camera2d>>,
    image_assets: Res<ImageAssets>,
    kid_saver: Res<KidSaver>,
){
    let Ok(camera) = camera_query.get_single()
    else{
        return;
    };
    if 3<kid_saver.save_id && kid_saver.save_id<=8{
        return;
    }
    commands.spawn((
        Sprite{
            image: image_assets.gameover.clone(),
            ..default()
        },
        GameOver,
    )).insert(
        Transform::from_xyz(camera.translation.x, camera.translation.y, 0.1)
    );
}

fn gameover_text_despawn(
    mut commands: Commands,
    query: Query<Entity,With<GameOver>>
){
    for item in &query{
        commands.entity(item).despawn_recursive();
    }
}