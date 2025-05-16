use std::collections::HashSet;

use bevy::prelude::*;

use bevy::sprite::Sprite;
use crate::asset_loader::{BackGroundAssets, BuildingAssets, ImageAssets, MusicAssets, SceneAssets};
use crate::base::ground::spawn_single_box;
use crate::base::wrap::spawn_once_warp;
use crate::state::{GameState, NeedReload};

use super::center::{BuildingState, ChangeBuilding};

const BASEX: f32 = 800.0;
const BASEY: f32 = 608.0*3.;

#[derive(Resource, Debug, Default)]
pub struct BlockChosen{
    pub choose: i8,
}

#[derive(Component, Debug)]
pub struct Block{
    pub id: i8,
    pub x: i8,
    pub y: i8,
}


pub struct BuildingDPlugin;

impl Plugin for BuildingDPlugin{
    fn build(&self,app: &mut App){
        app.init_resource::<BlockChosen>()
        .add_systems(PostStartup,spawn_once)
        .add_systems(OnExit(GameState::Reload),spawn_reload)
        .add_systems(Update,(do_choose,do_move));
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
            image: scene_assets.discription_d.clone(),
            ..Default::default()
        }
    ).insert(
        Transform::from_xyz(BASEX-144., BASEY+48., -0.2)
    );
    commands.spawn(
        Sprite{
            image: scene_assets.unfilled.clone(),
            ..Default::default()
        }
    ).insert(
        Transform::from_xyz(BASEX+144., BASEY+48., -0.2)
    );

    spawn_single_box(&mut commands,0.,-8.5,BASEX,BASEY,12.5,1.);
    spawn_single_box(&mut commands,0.,8.5,BASEX,BASEY,12.5,1.);
    spawn_single_box(&mut commands,-11.5,0.,BASEX,BASEY,1.,7.5);
    spawn_single_box(&mut commands,11.5,0.,BASEX,BASEY,1.,7.5);

}


fn spawn_reload(
    mut commands: Commands,
    building_assets: Res<BuildingAssets>,
    mut choose: ResMut<BlockChosen>,
){
    choose.choose = -1;

    commands.spawn(
        Sprite{
            image: building_assets.block0.clone(),
            ..Default::default()
        }
    ).insert(
        Transform::from_xyz(BASEX-288., BASEY+224., -0.1)
    ).insert(
        Block{
            id: 0,
            x: -9-2,
            y: 7+1,
        }
    ).insert(NeedReload);

    commands.spawn(
        Sprite{
            image: building_assets.block1.clone(),
            ..Default::default()
        }
    ).insert(
        Transform::from_xyz(BASEX-224.+16., BASEY+192.+16., -0.1)
    ).insert(
        Block{
            id: 1,
            x: -7-2,
            y: 6+1,
        }
    ).insert(NeedReload);

    commands.spawn(
        Sprite{
            image: building_assets.block2.clone(),
            ..Default::default()
        }
    ).insert(
        Transform::from_xyz(BASEX-192.-16., BASEY+256.-16., -0.1)
    ).insert(
        Block{
            id: 2,
            x: -6-2,
            y: 8+1,
        }
    ).insert(NeedReload);

    commands.spawn(
        Sprite{
            image: building_assets.block3.clone(),
            ..Default::default()
        }
    ).insert(
        Transform::from_xyz(BASEX-96., BASEY+224.-16., -0.1)
    ).insert(
        Block{
            id: 3,
            x: -3-2,
            y: 7+1,
        }
    ).insert(NeedReload);

    commands.spawn(
        Sprite{
            image: building_assets.block4.clone(),
            ..Default::default()
        }
    ).insert(
        Transform::from_xyz(BASEX-32.+16., BASEY+224., -0.1)
    ).insert(
        Block{
            id: 4,
            x: -1-2,
            y: 7+1,
        }
    ).insert(NeedReload);
    
    commands.spawn(
        Sprite{
            image: building_assets.block5.clone(),
            ..Default::default()
        }
    ).insert(
        Transform::from_xyz(BASEX+32.-16., BASEY+224., -0.1)
    ).insert(
        Block{
            id: 5,
            x: 1-2,
            y: 7+1,
        }
    ).insert(NeedReload);

    commands.spawn(
        Sprite{
            image: building_assets.block6.clone(),
            ..Default::default()
        }
    ).insert(
        Transform::from_xyz(BASEX+96., BASEY+224., -0.1)
    ).insert(
        Block{
            id: 6,
            x: 3-2,
            y: 7+1,
        }
    ).insert(NeedReload);

    commands.spawn(
        Sprite{
            image: building_assets.block7.clone(),
            ..Default::default()
        }
    ).insert(
        Transform::from_xyz(BASEX+160., BASEY+224., -0.1)
    ).insert(
        Block{
            id: 7,
            x: 5-2,
            y: 7+1,
        }
    ).insert(NeedReload);

    commands.spawn(
        Sprite{
            image: building_assets.block8.clone(),
            ..Default::default()
        }
    ).insert(
        Transform::from_xyz(BASEX+224., BASEY+224.-16., -0.1)
    ).insert(
        Block{
            id: 8,
            x: 7-2,
            y: 7+1,
        }
    ).insert(NeedReload);

    commands.spawn(
        Sprite{
            image: building_assets.block9.clone(),
            ..Default::default()
        }
    ).insert(
        Transform::from_xyz(BASEX+320., BASEY+224.-16., -0.1)
    ).insert(
        Block{
            id: 9,
            x: 10-2,
            y: 7+1,
        }
    ).insert(NeedReload);
}


fn do_choose(
    keyboard_input:Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Block),With<Block>>,
    mut choose: ResMut<BlockChosen>,
    state: Res<BuildingState>,
){
    if state.choose != 4 || choose.choose == -2{
        return;
    }
    for (mut trans,block) in query.iter_mut(){
        if block.id == choose.choose{
            trans.translation.z = -0.1;
        }
    }

    if keyboard_input.just_released(KeyCode::Digit0) || keyboard_input.just_released(KeyCode::Numpad0){
        choose.choose = 0;
    }else if keyboard_input.just_released(KeyCode::Digit1) || keyboard_input.just_released(KeyCode::Numpad1){
        choose.choose = 1;
    }else if keyboard_input.just_released(KeyCode::Digit2) || keyboard_input.just_released(KeyCode::Numpad2){
        choose.choose = 2;
    }else if keyboard_input.just_released(KeyCode::Digit3) || keyboard_input.just_released(KeyCode::Numpad3){
        choose.choose = 3;
    }else if keyboard_input.just_released(KeyCode::Digit4) || keyboard_input.just_released(KeyCode::Numpad4){
        choose.choose = 4;
    }else if keyboard_input.just_released(KeyCode::Digit5) || keyboard_input.just_released(KeyCode::Numpad5){
        choose.choose = 5;
    }else if keyboard_input.just_released(KeyCode::Digit6) || keyboard_input.just_released(KeyCode::Numpad6){
        choose.choose = 6;
    }else if keyboard_input.just_released(KeyCode::Digit7) || keyboard_input.just_released(KeyCode::Numpad7){
        choose.choose = 7;
    }else if keyboard_input.just_released(KeyCode::Digit8) || keyboard_input.just_released(KeyCode::Numpad8){
        choose.choose = 8;
    }else if keyboard_input.just_released(KeyCode::Digit9) || keyboard_input.just_released(KeyCode::Numpad9){
        choose.choose = 9;
    }

    for (mut trans,block) in query.iter_mut(){
        if block.id == choose.choose{
            trans.translation.z = -0.05;
        }
    }
}


fn do_move(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    music_assets: Res<MusicAssets>,
    state: Res<BuildingState>,
    keyboard_input:Res<ButtonInput<KeyCode>>,
    mut choose: ResMut<BlockChosen>,
    mut query: Query<(&mut Transform, &mut Block),With<Block>>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>
){
    if state.choose != 4 || choose.choose == -2{
        return;
    }
    let mut dir = -1;
    if keyboard_input.just_released(KeyCode::KeyW){
        dir = 0;
    }else if keyboard_input.just_released(KeyCode::KeyA){
        dir = 1;
    }else if keyboard_input.just_released(KeyCode::KeyS){
        dir = 2;
    }else if keyboard_input.just_released(KeyCode::KeyD){
        dir = 3;
    }

    for (mut trans,mut block) in query.iter_mut(){
        if block.id == choose.choose{
            if dir == 0{
                trans.translation.y += 32.;
                block.y += 1;
            }else if dir == 1{
                trans.translation.x -= 32.;
                block.x -= 1;
            }else if dir == 2{
                trans.translation.y -= 32.;
                block.y -= 1;
            }else if dir == 3{
                trans.translation.x += 32.;
                block.x += 1;
            }    
        }
    }

    let mut set = HashSet::new();
    for (_, block) in query.iter(){
        if block.id == 0{
            if 1<=block.x && block.x<=4 && 1<=block.y && block.y<=4{
                set.insert(block.x+6*block.y-1);
                set.insert(block.x+6*block.y+5);
                set.insert(block.x+6*block.y-7);
                set.insert(block.x+6*block.y-6);
                set.insert(block.x+6*block.y-5);
            }
        }else if block.id == 1{
            if 0<=block.x && block.x<=4 && 0<=block.y && block.y<=4{
                set.insert(block.x+6*block.y);
                set.insert(block.x+6*block.y+6);
                set.insert(block.x+6*block.y+1);
            }
        }else if block.id == 2{
            if 2<=block.x && block.x<=4 && 1<=block.y && block.y<=5{
                set.insert(block.x+6*block.y);
                set.insert(block.x+6*block.y-1);
                set.insert(block.x+6*block.y-2);
                set.insert(block.x+6*block.y+1);
                set.insert(block.x+6*block.y-5);
            }
        }else if block.id == 3{
            if 1<=block.x && block.x<=4 && 1<=block.y && block.y<=5{
                set.insert(block.x+6*block.y);
                set.insert(block.x+6*block.y-1);
                set.insert(block.x+6*block.y-6);
                set.insert(block.x+6*block.y-5);
            }
        }else if block.id == 4{
            if 0<=block.x && block.x<=4 && 1<=block.y && block.y<=4{
                set.insert(block.x+6*block.y);
                set.insert(block.x+6*block.y+6);
                set.insert(block.x+6*block.y-6);
                set.insert(block.x+6*block.y+1);
            }
        }else if block.id == 5{
            if 1<=block.x && block.x<=5 && 1<=block.y && block.y<=4{
                set.insert(block.x+6*block.y);
                set.insert(block.x+6*block.y+6);
                set.insert(block.x+6*block.y+5);
                set.insert(block.x+6*block.y-6);
            }
        }else if block.id == 6{
            if 0<=block.x && block.x<=5 && 0<=block.y && block.y<=5{
                set.insert(block.x+6*block.y);
            }
        }else if block.id == 7{
            if 0<=block.x && block.x<=5 && 1<=block.y && block.y<=4{
                set.insert(block.x+6*block.y);
                set.insert(block.x+6*block.y+6);
                set.insert(block.x+6*block.y-6);
            }
        }else if block.id == 8{
            if 0<=block.x && block.x<=5 && 1<=block.y && block.y<=5{
                set.insert(block.x+6*block.y);
                set.insert(block.x+6*block.y-6);
            }
        }else if block.id == 9{
            if 1<=block.x && block.x<=4 && 1<=block.y && block.y<=5{
                set.insert(block.x+6*block.y-1);
                set.insert(block.x+6*block.y+1);
                set.insert(block.x+6*block.y-7);
                set.insert(block.x+6*block.y-6);
                set.insert(block.x+6*block.y-5);
            }
        }
    }

    if set.len() == 36{
        choose.choose = -2;
        commands.spawn(AudioPlayer::new(music_assets.coin.clone())).insert(NeedReload);
        let wr_layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 4, 1, None, None);
        let wr_atlas_layout = texture_atlases.add(wr_layout);
        let wr_atlas = TextureAtlas{
            layout : wr_atlas_layout,
            index : 0,
        };
        let wr_image = image_assets.warp.clone();
        let wr = spawn_once_warp(&mut commands,&wr_image,&wr_atlas,BASEX,BASEY-96.,BASEX+800.,BASEY-608.+64.);
        commands.entity(wr).insert(
            ChangeBuilding{
                delta: 8,
                to: 0,
            }
        );
    }
}

