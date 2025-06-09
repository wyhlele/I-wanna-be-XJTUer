use std::collections::HashSet;

use bevy::prelude::*;
use bevy::sprite::Sprite;

use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::asset_loader::{AchievementAssets, BackGroundAssets, BuildingAssets, ImageAssets, MusicAssets, SceneAssets};
use crate::base::ground::spawn_single_box;
use crate::base::wrap::spawn_once_warp;
use crate::kid_saver::KidSaver;
use crate::menu::achievement::Achievement;
use crate::state::{GameState, NeedReload};

use super::center::{BuildingState, ChangeBuilding};

const BASEX: f32 = -800.*2.;
const BASEY: f32 = 608.0*2.;

#[derive(Resource, Debug, Default)]
pub struct HuaState{
    pub state: bool,
}

#[derive(Component, Debug)]
pub struct Block{
    pub id: i8,
    pub x: i8,
    pub y: i8,
}


pub struct BuildingAPlugin;

impl Plugin for BuildingAPlugin{
    fn build(&self,app: &mut App){
        app.init_resource::<HuaState>()
        .add_systems(PostStartup,spawn_once)
        .add_systems(OnExit(GameState::ReForBuilding),spawn_reload)
        .add_systems(Update,do_move);
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
            image: scene_assets.discription_a.clone(),
            ..Default::default()
        }
    ).insert(
        Transform::from_xyz(BASEX-144., BASEY+48., -0.2)
    );
    commands.spawn(
        Sprite{
            image: scene_assets.right_eq.clone(),
            ..Default::default()
        }
    ).insert(
        Transform::from_xyz(BASEX+224., BASEY+48., -0.2)
    );

    commands.spawn(
        Sprite{
            image: scene_assets.unfilled.clone(),
            ..Default::default()
        }
    ).insert(
        Transform{
            translation: Vec3::new(BASEX+96., BASEY+48., -0.2),
            scale: Vec3::new(0.75,0.75,1.),
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
    building_assets: Res<BuildingAssets>,
    mut hua: ResMut<HuaState>,
    query: Query<Entity,With<Block>>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>
){
    hua.state = true;

    for item in &query{
        commands.entity(item).despawn_recursive();
    }

    let hua_layout = TextureAtlasLayout::from_grid(UVec2::new(64, 64), 3, 3, None, None);
    let hua_atlas_layout = texture_atlases.add(hua_layout);

    commands.spawn(
        Sprite{
            image: building_assets.hua.clone(),
            texture_atlas: Some(
                TextureAtlas{
                    layout : hua_atlas_layout.clone(),
                    index : 0,
                }
            ),
            ..Default::default()
        }
    ).insert(
        Transform{
          translation: Vec3::new(BASEX+96.+0., BASEY+48.+0., -0.1),
          scale: Vec3::new(0.75, 0.75, 1.0),
          ..Default::default()
        }
    ).insert(
        Block{
            id: 0,
            x: 1,
            y: 1,
        }
    ).insert(NeedReload);

    commands.spawn(
        Sprite{
            image: building_assets.hua.clone(),
            texture_atlas: Some(
                TextureAtlas{
                    layout : hua_atlas_layout.clone(),
                    index : 1,
                }
            ),
            ..Default::default()
        }
    ).insert(
        Transform{
          translation: Vec3::new(BASEX+96.+48., BASEY+48.-48., -0.1),
          scale: Vec3::new(0.75, 0.75, 1.0),
          ..Default::default()
        }
    ).insert(
        Block{
            id: 1,
            x: 2,
            y: 2,
        }
    ).insert(NeedReload);

    commands.spawn(
        Sprite{
            image: building_assets.hua.clone(),
            texture_atlas: Some(
                TextureAtlas{
                    layout : hua_atlas_layout.clone(),
                    index : 2,
                }
            ),
            ..Default::default()
        }
    ).insert(
        Transform{
          translation: Vec3::new(BASEX+96.-48., BASEY+48.+0., -0.1),
          scale: Vec3::new(0.75, 0.75, 1.0),
          ..Default::default()
        }
    ).insert(
        Block{
            id: 2,
            x: 1,
            y: 0,
        }
    ).insert(NeedReload);

    commands.spawn(
        Sprite{
            image: building_assets.hua.clone(),
            texture_atlas: Some(
                TextureAtlas{
                    layout : hua_atlas_layout.clone(),
                    index : 3,
                }
            ),
            ..Default::default()
        }
    ).insert(
        Transform{
          translation: Vec3::new(BASEX+96.+0., BASEY+48.-48., -0.1),
          scale: Vec3::new(0.75, 0.75, 1.0),
          ..Default::default()
        }
    ).insert(
        Block{
            id: 3,
            x: 2,
            y: 1,
        }
    ).insert(NeedReload);

    commands.spawn(
        Sprite{
            image: building_assets.hua.clone(),
            texture_atlas: Some(
                TextureAtlas{
                    layout : hua_atlas_layout.clone(),
                    index : 4,
                }
            ),
            ..Default::default()
        }
    ).insert(
        Transform{
          translation: Vec3::new(BASEX+96.+48., BASEY+48.+48., -0.1),
          scale: Vec3::new(0.75, 0.75, 1.0),
          ..Default::default()
        }
    ).insert(
        Block{
            id: 4,
            x: 0,
            y: 2,
        }
    ).insert(NeedReload);

    commands.spawn(
        Sprite{
            image: building_assets.hua.clone(),
            texture_atlas: Some(
                TextureAtlas{
                    layout : hua_atlas_layout.clone(),
                    index : 5,
                }
            ),
            ..Default::default()
        }
    ).insert(
        Transform{
          translation: Vec3::new(BASEX+96.-48., BASEY+48.-48., -0.1),
          scale: Vec3::new(0.75, 0.75, 1.0),
          ..Default::default()
        }
    ).insert(
        Block{
            id: 5,
            x: 2,
            y: 0,
        }
    ).insert(NeedReload);

    commands.spawn(
        Sprite{
            image: building_assets.hua.clone(),
            texture_atlas: Some(
                TextureAtlas{
                    layout : hua_atlas_layout.clone(),
                    index : 7,
                }
            ),
            ..Default::default()
        }
    ).insert(
        Transform{
          translation: Vec3::new(BASEX+96.-48., BASEY+48.+48., -0.1),
          scale: Vec3::new(0.75, 0.75, 1.0),
          ..Default::default()
        }
    ).insert(
        Block{
            id: 7,
            x: 0,
            y: 0,
        }
    ).insert(NeedReload);

    commands.spawn(
        Sprite{
            image: building_assets.hua.clone(),
            texture_atlas: Some(
                TextureAtlas{
                    layout : hua_atlas_layout.clone(),
                    index : 8,
                }
            ),
            ..Default::default()
        }
    ).insert(
        Transform{
          translation: Vec3::new(BASEX+96.+48., BASEY+48.+0., -0.1),
          scale: Vec3::new(0.75, 0.75, 1.0),
          ..Default::default()
        }
    ).insert(
        Block{
            id: 8,
            x: 1,
            y: 2,
        }
    ).insert(NeedReload);
    
}


fn do_move(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    music_assets: Res<MusicAssets>,
    achievement_assets: Res<AchievementAssets>,
    mut kid_saver: ResMut<KidSaver>,
    state: Res<BuildingState>,
    keyboard_input:Res<ButtonInput<KeyCode>>,
    mut hua: ResMut<HuaState>,
    mut query: Query<(&mut Transform, &mut Block),With<Block>>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>
){
    if state.choose != 1 || hua.state == false{
        return;
    }

    let mut set = HashSet::new();
    for (_, block) in query.iter(){
        let value = 3*block.x+block.y;
        set.insert(value);
    }

    let mut now = 0;
    while now < 9{
        if !set.contains(&now){
            break;
        }
        now += 1;
    }
    
    let dir;
    if keyboard_input.just_released(KeyCode::KeyW){
        if now/3==2 {
            return;
        }
        now += 3;
        dir = 0;
    }else if keyboard_input.just_released(KeyCode::KeyA){
        if now%3==2 {
            return;
        }
        now += 1;
        dir = 1;
    }else if keyboard_input.just_released(KeyCode::KeyS){
        if now/3==0 {
            return;
        }
        now -= 3;
        dir = 2;
    }else if keyboard_input.just_released(KeyCode::KeyD){
        if now%3==0 {
            return;
        }
        now -= 1;
        dir = 3;
    }else{
        return;
    }

    for (mut trans, mut block) in query.iter_mut(){
        let pos = 3*block.x+block.y;
        if pos == now{
            match dir{
                0 => {block.x-=1;trans.translation.y+=48.;},
                1 => {block.y-=1;trans.translation.x-=48.;},
                2 => {block.x+=1;trans.translation.y-=48.;},
                3 => {block.y+=1;trans.translation.x+=48.;},
                _ => {}
            }
        }
    }

    for (_, block) in query.iter(){
        if block.id%3==1 && block.y!=1{
            return;
        }
    }
    let mut a0=0;
    let mut a1=0;
    let mut a2=0;
    let mut b0=0;
    let mut b1=0;
    let mut b2=0;

    for (_, block) in query.iter(){
        let value;
        let pos = 3*block.x+block.y;
        match block.id{
            0 => {value = 6;},
            2 => {value = 2;},
            3 => {value = 3;},
            5 => {value = 7;},
            8 => {value = 5;},
            _ => {continue;}
        }
        match pos{
            0 => {a0 = value;},
            2 => {b0 = value;},
            3 => {a1 = value;},
            5 => {b1 = value;},
            6 => {a2 = value;},
            8 => {b2 = value;},
            _ => {}
        }
    }
    let mut solve= 0;
    for (_, block) in query.iter(){
        if block.id==1{
            if block.x==0 && (a0&b0)!=6{
                return;
            }
            if block.x==1 && (a1&b1)!=3{
                return;
            }
            if block.x==2 && (a2&b2)!=2{
                return;
            }
            if block.x==0{
                solve=1;
            }else{
                solve=2;
            }
        }
        if block.id==4{
            if block.x==0 && (a0|b0)!=6{
                return;
            }
            if block.x==1 && (a1|b1)!=3{
                return;
            }
            if block.x==2 && (a2|b2)!=2{
                return;
            }
        }
        if block.id==7{
            if block.x==0 && ((7&(!b0))!=6 || a0!=0){
                return;
            }
            if block.x==1 && ((7&(!b1))!=3 || a1!=0){
                return;
            }
            if block.x==2 && ((7&(!b2))!=2 || a2!=0){
                return;
            }
        }
    }

    if kid_saver.solve!=3 {
        kid_saver.solve |= solve;
        let file_path = Path::new("save");
        let mut file = match File::create(file_path) {
            Ok(file) => file,
            Err(_) => {
                warn!("ERROR: cannot create file save");
                return;
            }
        };
        let numbers = [kid_saver.save_id as i32, kid_saver.achi as i32, kid_saver.solve as i32];
        for &number in &numbers {
            if let Err(_) = writeln!(file, "{}", number) {
                warn!("ERROR: cannot create file save");
                return;
            }
        }
        if kid_saver.solve == 3{
            if (kid_saver.achi>>4)&1==0{
                commands.spawn(Achievement{time: 72, id: 4})
                .insert(Sprite{
                    image: achievement_assets.achievement4.clone(),
                    ..Default::default()
                }).insert(Transform::from_xyz(0., 0., -5.0));
            }
        }
    }

    hua.state = false;
    commands.spawn(AudioPlayer::new(music_assets.coin.clone())).insert(NeedReload);
    let wr_layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 4, 1, None, None);
    let wr_atlas_layout = texture_atlases.add(wr_layout);
    let wr_atlas = TextureAtlas{
        layout : wr_atlas_layout,
        index : 0,
    };
    let wr_image = image_assets.warp.clone();
    let wr = spawn_once_warp(&mut commands,&wr_image,&wr_atlas,BASEX,BASEY-96.,BASEX+3200.,BASEY+64.);
    commands.entity(wr).insert(
        ChangeBuilding{
            delta: 1,
            to: 0,
        }
    );
    

}

