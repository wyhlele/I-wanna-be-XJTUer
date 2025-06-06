
use bevy::prelude::*;

use bevy::sprite::Sprite;
use rand::Rng;
use crate::asset_loader::{BackGroundAssets, BuildingAssets, ImageAssets, MusicAssets, SceneAssets};
use crate::base::ground::spawn_single_box;
use crate::base::wrap::spawn_once_warp;
use crate::state::{GameState, NeedReload};

use super::center::{BuildingState, ChangeBuilding};

const BASEX: f32 = -800.;
const BASEY: f32 = 608.0*3.;

#[derive(Resource, Debug, Default)]
pub struct WordleState{
    pub state: bool,
    pub answer: Vec<usize>,
    pub mem: Vec<usize>,
}

#[derive(Component, Debug)]
pub struct Block;

pub struct BuildingBPlugin;

impl Plugin for BuildingBPlugin{
    fn build(&self,app: &mut App){
        app.init_resource::<WordleState>()
        .add_systems(PostStartup,spawn_once)
        .add_systems(OnExit(GameState::ReForBuilding),spawn_reload)
        .add_systems(Update,do_input);
    }
}


fn spawn_once(
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
    bg_assets: Res<BackGroundAssets>,
){
    commands.spawn(
        Sprite{
            image: bg_assets.classroom.clone(),
            ..Default::default()
        }
    ).insert(
        Transform::from_xyz(BASEX, BASEY, -0.5)
    );
    commands.spawn(
        Sprite{
            image: scene_assets.main_building.clone(),
            ..Default::default()
        }
    ).insert(
        Transform::from_xyz(BASEX, BASEY, -0.3)
    );
    commands.spawn(
        Sprite{
            image: scene_assets.discription_b.clone(),
            ..Default::default()
        }
    ).insert(
        Transform::from_xyz(BASEX-128., BASEY+48., -0.2)
    );

    commands.spawn(
        Sprite{
            image: scene_assets.unfilled.clone(),
            ..Default::default()
        }
    ).insert(
        Transform{
            translation: Vec3::new(BASEX+144., BASEY+48., -0.2),
            scale: Vec3::new(1.,1.,1.),
            ..Default::default()
        }
    );

    spawn_single_box(&mut commands,0.,-8.5,BASEX,BASEY,12.5,1.);
    spawn_single_box(&mut commands,0.,8.5,BASEX,BASEY,12.5,1.);
    spawn_single_box(&mut commands,-11.5,0.,BASEX,BASEY,1.,7.5);
    spawn_single_box(&mut commands,11.5,0.,BASEX,BASEY,1.,7.5);

}


fn spawn_reload(
    mut commands: Commands,
    mut wordle: ResMut<WordleState>,
    query: Query<Entity,With<Block>>,
){
    wordle.state = true;
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..3);
    match index {
        0 => {wordle.answer = vec![92, 36, 76, 80];} // XJTU
        1 => {wordle.answer = vec![44, 56, 84, 16];} // LOVE
        _ => {wordle.answer = vec![72, 76, 16, 48];} // STEM
    }
    wordle.mem = vec![];
    for item in &query{
        commands.entity(item).despawn_recursive();
    }
}


fn do_input(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    music_assets: Res<MusicAssets>,
    building_assets: Res<BuildingAssets>,
    mut wordle: ResMut<WordleState>,
    state: Res<BuildingState>,
    keyboard_input:Res<ButtonInput<KeyCode>>,
    query: Query<Entity,With<Block>>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>
){
    if state.choose != 2 || wordle.state == false{
        return;
    }

    let code: usize;
    if keyboard_input.just_released(KeyCode::KeyA){
        code = 0;
    }else if keyboard_input.just_released(KeyCode::KeyB) {
        code = 4;
    }else if keyboard_input.just_released(KeyCode::KeyC) {
        code = 8;
    }else if keyboard_input.just_released(KeyCode::KeyD) {
        code = 12;
    }else if keyboard_input.just_released(KeyCode::KeyE) {
        code = 16;
    }else if keyboard_input.just_released(KeyCode::KeyF) {
        code = 20;
    }else if keyboard_input.just_released(KeyCode::KeyG) {
        code = 24;
    }else if keyboard_input.just_released(KeyCode::KeyH) {
        code = 28;
    }else if keyboard_input.just_released(KeyCode::KeyI) {
        code = 32;
    }else if keyboard_input.just_released(KeyCode::KeyJ) {
        code = 36;
    }else if keyboard_input.just_released(KeyCode::KeyK) {
        code = 40;
    }else if keyboard_input.just_released(KeyCode::KeyL) {
        code = 44;
    }else if keyboard_input.just_released(KeyCode::KeyM) {
        code = 48;
    }else if keyboard_input.just_released(KeyCode::KeyN) {
        code = 52;
    }else if keyboard_input.just_released(KeyCode::KeyO) {
        code = 56;
    }else if keyboard_input.just_released(KeyCode::KeyP) {
        code = 60;
    }else if keyboard_input.just_released(KeyCode::KeyQ) {
        code = 64;
    }else if keyboard_input.just_released(KeyCode::KeyR) {
        code = 68;
    }else if keyboard_input.just_released(KeyCode::KeyS) {
        code = 72;
    }else if keyboard_input.just_released(KeyCode::KeyT) {
        code = 76;
    }else if keyboard_input.just_released(KeyCode::KeyU) {
        code = 80;
    }else if keyboard_input.just_released(KeyCode::KeyV) {
        code = 84;
    }else if keyboard_input.just_released(KeyCode::KeyW) {
        code = 88;
    }else if keyboard_input.just_released(KeyCode::KeyX) {
        code = 92;
    }else if keyboard_input.just_released(KeyCode::KeyY) {
        code = 96;
    }else if keyboard_input.just_released(KeyCode::KeyZ) {
        code = 100;
    }else if keyboard_input.just_released(KeyCode::Enter) {
        code = 104;
    }else{
        return;
    }
    if code == 104{
        if wordle.mem.len()==0{
            return;
        }else if wordle.mem.len()%4!=0{
            return;
        }else{
            if let Some(lst) = wordle.mem.last(){
                if lst%4!=0{
                    return;
                }
            }
        }
        let len = wordle.mem.len();
        let split_point = if len >= 4 { len - 4 } else { 0 };
        let mut last_four = wordle.mem.split_off(split_point);
        let mut answer = wordle.answer.clone();

        let mut x = 0;
        while x<4{
            if last_four[x] == answer[x]{
                last_four[x] += 3;
                answer[x] = 104;
            }
            x+=1;
        }

        x = 0;
        while x<4{
            let mut y = 0;
            while y<4{
                if last_four[x] == answer[y]{
                    last_four[x] += 1;
                    answer[y] = 104;
                }
                y+=1;
            }
            x+=1;
        }

        x = 0;
        let mut flag = true;
        while x<4{
            if last_four[x]%4==0{
                last_four[x] += 2;
            }
            if last_four[x]%4!=3{
                flag = false;
            }
            wordle.mem.push(last_four[x]);
            x+=1;
        }

        if flag{
            wordle.state = false;
            commands.spawn(AudioPlayer::new(music_assets.coin.clone())).insert(NeedReload);
            let wr_layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 4, 1, None, None);
            let wr_atlas_layout = texture_atlases.add(wr_layout);
            let wr_atlas = TextureAtlas{
                layout : wr_atlas_layout,
                index : 0,
            };
            let wr_image = image_assets.warp.clone();
            let wr = spawn_once_warp(&mut commands,&wr_image,&wr_atlas,BASEX,BASEY-96.,BASEX+2400.,BASEY-608.+64.);
            commands.entity(wr).insert(
                ChangeBuilding{
                    delta: 2,
                    to: 0,
                }
            );
        }else{
            while wordle.mem.len()>12{
                wordle.mem.remove(0);
            }
        }

    }else{
        if wordle.mem.len()>0 && wordle.mem.len()%4==0 {
            if let Some(lst) = wordle.mem.last(){
                if lst%4==0{
                    return;
                }
            }
        }
        wordle.mem.push(code);
    }

    for item in &query{
        commands.entity(item).despawn_recursive();
    }

    let len = wordle.mem.len();
    let word_layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 16, 7, None, None);
    let word_atlas_layout = texture_atlases.add(word_layout);

    let mut i = 0;
    let mut x = 72.;
    let mut y = 120.;
    while i < len{
        commands.spawn(
            Sprite{
                image: building_assets.alphabeta.clone(),
                texture_atlas: Some(
                    TextureAtlas{
                        layout : word_atlas_layout.clone(),
                        index : wordle.mem[i],
                    }
                ),
                ..Default::default()
            }
        ).insert(
            Transform{
              translation: Vec3::new(BASEX+x, BASEY+y, -0.1),
              scale: Vec3::new(1.5, 1.5, 1.0),
              ..Default::default()
            }
        ).insert(
            Block
        );
        x += 48.;
        i += 1;
        if i%4==0{
            x = 72.;
            y -= 48.;
        }
    }

}