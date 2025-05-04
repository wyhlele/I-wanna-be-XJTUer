use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy::audio::{PlaybackMode, Volume};
use crate::{asset_loader::MusicAssets, base::kid::Kid, kid_saver::KidSaver};

#[derive(Debug, Default, States, Hash, Clone, Copy, Eq, PartialEq)]
pub enum GameState {
    #[default]
    InGame,
    Reload,
    GameOver,
}

#[derive(Component, Debug, Default)]
pub struct NeedReload;

#[derive(Component, Debug, Default)]
pub struct BGM;

#[derive(Component, Debug, Default)]
pub struct BGMReload{
    pub id: i8
}
pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
        .insert_resource(State::new(GameState::Reload))
        .add_systems(Update, reload_game)
        .add_systems(Update, reload_bgm)
        .add_systems(OnEnter(GameState::InGame), play_bgm);
    }
}

fn reload_game(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    keyboard_input:Res<ButtonInput<KeyCode>>,
    query: Query<Entity,With<NeedReload>>,
){
    if keyboard_input.pressed(KeyCode::KeyR){
        next_state.set(GameState::Reload);
    }
    if keyboard_input.just_released(KeyCode::KeyR){
        for entity in &query{
            commands.entity(entity).despawn_recursive();
        }
        next_state.set(GameState::InGame);
    }
}

fn cnt_bgm(
    id: i8,
    music_assets: &Res<MusicAssets>,
)->Handle<AudioSource>{
    if id <=0{
        music_assets.gate.clone()
    }else if id<=3{
        music_assets.festival.clone()
    }else if id<=8{
        music_assets.museum.clone()
    }else if id<=14{
        music_assets.building.clone()
    }else{
        music_assets.dead.clone()
    }
}

fn play_bgm(
    mut commands: Commands,
    kid_saver: Res<KidSaver>,
    music_assets: Res<MusicAssets>,
){
    commands.spawn(
        AudioPlayer::new(cnt_bgm(kid_saver.save_id, &music_assets))
    )
    .insert(PlaybackSettings {
        mode: PlaybackMode::Loop,
        volume: Volume::new(0.3),
        speed: 1.0,
        paused: false,
        spatial: false,
        spatial_scale: None,
    })
    .insert(NeedReload)
    .insert(BGM);
}

fn reload_bgm(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    kid_query: Query<&Kid>,
    reload_query: Query<&BGMReload>,
    bgm_query: Query<Entity,With<BGM>>,
    music_assets: Res<MusicAssets>,
){
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity_a, entity_b, _) => {
                let is_entity1_b = kid_query.get(*entity_b).is_ok();
                let is_entity2_a = reload_query.get(*entity_a).is_ok();
                let is_entity1_a = kid_query.get(*entity_a).is_ok();
                let is_entity2_b = reload_query.get(*entity_b).is_ok();
                if is_entity1_b && is_entity2_a{
                    let id = reload_query.get(*entity_a).unwrap().id;
                    for bgm in bgm_query.iter(){
                        commands.entity(bgm).despawn_recursive();
                    }
                    commands.spawn(
                        AudioPlayer::new(cnt_bgm(id, &music_assets))
                    ).insert(PlaybackSettings {
                        mode: PlaybackMode::Loop,
                        volume: Volume::new(0.3),
                        speed: 1.0,
                        paused: false,
                        spatial: false,
                        spatial_scale: None,
                    })
                    .insert(NeedReload)
                    .insert(BGM);
                }else if is_entity1_a && is_entity2_b{
                    let id = reload_query.get(*entity_b).unwrap().id;
                    for bgm in bgm_query.iter(){
                        commands.entity(bgm).despawn_recursive();
                    }
                    let mut music = music_assets.gate.clone();
                    if 1<=id && id<=3{
                        music = music_assets.festival.clone();
                    }
                    commands.spawn(AudioPlayer::new(music))
                    .insert(PlaybackSettings {
                        mode: PlaybackMode::Loop,
                        volume: Volume::new(0.3),
                        speed: 1.0,
                        paused: false,
                        spatial: false,
                        spatial_scale: None,
                    })
                    .insert(NeedReload)
                    .insert(BGM);
                }
            }
            _ => {}
        }
    }
}