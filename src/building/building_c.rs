use std::collections::HashSet;

use bevy::prelude::*;

use bevy::sprite::Sprite;
use crate::asset_loader::{BackGroundAssets, BuildingAssets, ImageAssets, MusicAssets, SceneAssets};
use crate::base::ground::spawn_single_box;
use crate::base::wrap::spawn_once_warp;
use crate::state::{GameState, NeedReload};

use super::center::{BuildingState, ChangeBuilding};

const BASEX: f32 = 0.;
const BASEY: f32 = 608.0*4.;

#[derive(Resource, Debug, Default)]
pub struct MyPos{
    pub state: bool,
    pub stack: Vec<usize>,
    pub history: Vec<i32>,
    pub score: i32,
    pub used: HashSet<usize>,
    pub x: usize,
    pub y: usize,
}

#[derive(Component, Debug)]
pub struct Block{
    pub id: usize,
    pub x: usize,
    pub y: usize,
}

#[derive(Component, Debug)]
pub struct Num;

pub struct BuildingCPlugin;

impl Plugin for BuildingCPlugin{
    fn build(&self,app: &mut App){
        app.init_resource::<MyPos>()
        .add_systems(PostStartup,spawn_once)
        .add_systems(OnExit(GameState::Reload),spawn_reload)
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
            image: scene_assets.discription_c.clone(),
            ..Default::default()
        }
    ).insert(
        Transform::from_xyz(BASEX-112., BASEY+80., -0.2)
    );
    commands.spawn(
        Sprite{
            image: scene_assets.unfilled.clone(),
            ..Default::default()
        }
    ).insert(
        Transform{
            translation: Vec3::new(BASEX+144., BASEY+48., -0.2),
            scale: Vec3::new(0.83, 0.83, 1.0),
            ..Default::default()
        }
    );

    commands.spawn(
        Sprite{
            image: scene_assets.arrow.clone(),
            ..Default::default()
        }
    ).insert(
        Transform::from_xyz(BASEX+240., BASEY+80., -0.2)
    );
    commands.spawn(
        Sprite{
            image: scene_assets.arrow.clone(),
            ..Default::default()
        }
    ).insert(
        Transform::from_xyz(BASEX+48., BASEY-16., -0.2)
    );

    spawn_single_box(&mut commands,0.,-8.5,BASEX,BASEY,12.5,1.);
    spawn_single_box(&mut commands,0.,8.5,BASEX,BASEY,12.5,1.);
    spawn_single_box(&mut commands,-11.5,0.,BASEX,BASEY,1.,7.5);
    spawn_single_box(&mut commands,11.5,0.,BASEX,BASEY,1.,7.5);


}


fn spawn_reload(
    mut commands: Commands,
    building_assets: Res<BuildingAssets>,
    scene_assets: Res<SceneAssets>,
    mut mypos: ResMut<MyPos>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>
){
    mypos.state = true;
    mypos.stack = Vec::new();
    mypos.used = HashSet::new();
    mypos.history = Vec::new();
    mypos.score = 3;
    mypos.x = 4;
    mypos.y = 0;
    mypos.used.insert(20);


    let mp_layout = TextureAtlasLayout::from_grid(UVec2::new(48, 48), 10, 5, None, None);
    let mp_atlas_layout = texture_atlases.add(mp_layout);
    let mut x = 0;
    let mut py = BASEY + 112.;
    while x<5 {
        let mut y = 0;
        let mut px = BASEX + 80.;
        while y<5 {
            let mut index = x*10+y*2;
            if x==4 && y==0{
                index += 1;
            }
            commands.spawn(
                Sprite{
                    image: building_assets.countmap.clone(),
                    texture_atlas: Some(
                        TextureAtlas{
                            layout : mp_atlas_layout.clone(),
                            index : index,
                        }
                    ),
                    ..Default::default()
                }
            ).insert(
                Transform{
                  translation: Vec3::new(px, py, -0.1),
                  scale: Vec3::new(0.66, 0.66, 1.0),
                  ..Default::default()
                }
            ).insert(
                Block{
                    id: x*5+y,
                    x: x,
                    y: y,
                }
            ).insert(NeedReload);
            y+=1;
            px+=32.;
        }
        x+=1;
        py-=32.;
    }

    let num_layout = TextureAtlasLayout::from_grid(UVec2::new(29, 40), 11, 1, None, None);
    let num_atlas_layout = texture_atlases.add(num_layout);
    commands.spawn(
        Sprite{
            image: scene_assets.numbers.clone(),
            texture_atlas: Some(
                TextureAtlas{
                    layout : num_atlas_layout.clone(),
                    index : 3,
                }
            ),
            ..Default::default()
        }
    ).insert(
        Transform::from_xyz(BASEX-64., BASEY-32., -0.2)
    ).insert(Num);

}


fn do_move(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    music_assets: Res<MusicAssets>,
    scene_assets: Res<SceneAssets>,
    state: Res<BuildingState>,
    mut mypos: ResMut<MyPos>,
    keyboard_input:Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Sprite, &mut Block),(With<Block>,Without<Num>)>,
    num_query: Query<Entity,(With<Num>,Without<Block>)>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>
){
    if state.choose != 3 || mypos.state==false{
        return;
    }
    if keyboard_input.just_released(KeyCode::Backspace){
        if mypos.stack.len() == 0{
            return;
        }
        let nowpos = 5*mypos.x + mypos.y;
        mypos.used.remove(&nowpos);
        let mut lastpos = 0;
        if let Some(top) = mypos.stack.pop() {
            lastpos = top;
        }
        mypos.x = lastpos/5;
        mypos.y = lastpos%5;
        let mut lastscore = 0;
        if let Some(top) = mypos.history.pop() {
            lastscore = top;
        }
        mypos.score = lastscore;
        
    }else{
        let mut newpos = 5*mypos.x + mypos.y;
        if keyboard_input.just_released(KeyCode::KeyW){
            if mypos.x == 0{
                return;
            }
            newpos -= 5;
        }else if keyboard_input.just_released(KeyCode::KeyA){
            if mypos.y == 0{
                return;
            }
            newpos -= 1;
        }else if keyboard_input.just_released(KeyCode::KeyS){
            if mypos.x == 4{
                return;
            }
            newpos += 5;
        }else if keyboard_input.just_released(KeyCode::KeyD){
            if mypos.y == 4{
                return;
            }
            newpos += 1;
        }else{
            return;
        }
        if mypos.used.contains(&newpos){
            return;
        }
        let nowscore = mypos.score;
        let nowpos = 5*mypos.x + mypos.y;
        mypos.history.push(nowscore);
        mypos.stack.push(nowpos);
        mypos.used.insert(newpos);
        mypos.x = newpos/5;
        mypos.y = newpos%5;
        match newpos {
            0 => { mypos.score+=4; },
            1 => { mypos.score+=14; },
            2 => { mypos.score*=2; },
            3 => { mypos.score-=63; },
            4 => { mypos.score*=4; },
            5 => { mypos.score-=3; },
            6 => { mypos.score*=4; },
            7 => { mypos.score-=9; },
            8 => { mypos.score+=180; },
            9 => { mypos.score-=16; },
            10 => { mypos.score*=3; },
            11 => { mypos.score/=4; },
            12 => { mypos.score/=2; },
            13 => { mypos.score+=6; },
            14 => { mypos.score-=30; },
            15 => { mypos.score-=1; },
            16 => { mypos.score+=6; },
            17 => { mypos.score/=2; },
            18 => { mypos.score*=2; },
            19 => { mypos.score*=2; },
            20 => { mypos.score+=3; },
            21 => { mypos.score*=2; },
            22 => { mypos.score+=5; },
            23 => { mypos.score+=4; },
            24 => { mypos.score/=2; },
            _ => {},
        }
    }


    for (mut sprite,block) in query.iter_mut(){
        if mypos.used.contains(&block.id){
            if let Some(atlas) = &mut sprite.texture_atlas{
                atlas.index = block.id*2+1;
            }
        }else{
            if let Some(atlas) = &mut sprite.texture_atlas{
                atlas.index = block.id*2;
            }
        }
    }

    for item in num_query.iter(){
        commands.entity(item).despawn_recursive();
    }

    let flag = mypos.score < 0;
    let mut score = mypos.score.abs();
    let mut px = -64.;
    let num_layout = TextureAtlasLayout::from_grid(UVec2::new(29, 40), 11, 1, None, None);
    let num_atlas_layout = texture_atlases.add(num_layout);
    if score == 0{
        commands.spawn(
            Sprite{
                image: scene_assets.numbers.clone(),
                texture_atlas: Some(
                    TextureAtlas{
                        layout : num_atlas_layout.clone(),
                        index : 0,
                    }
                ),
                ..Default::default()
            }
        ).insert(
            Transform::from_xyz(BASEX+px, BASEY-32., -0.2)
        ).insert(Num);
    }else{
        while score>0{
            commands.spawn(
                Sprite{
                    image: scene_assets.numbers.clone(),
                    texture_atlas: Some(
                        TextureAtlas{
                            layout : num_atlas_layout.clone(),
                            index : (score%10) as usize,
                        }
                    ),
                    ..Default::default()
                }
            ).insert(
                Transform::from_xyz(BASEX+px, BASEY-32., -0.2)
            ).insert(Num);
            px -= 32.;
            score /= 10;
        }
        if flag{
            commands.spawn(
                Sprite{
                    image: scene_assets.numbers.clone(),
                    texture_atlas: Some(
                        TextureAtlas{
                            layout : num_atlas_layout.clone(),
                            index : 10,
                        }
                    ),
                    ..Default::default()
                }
            ).insert(
                Transform::from_xyz(BASEX+px, BASEY-32., -0.2)
            ).insert(Num);
        }
    }

    if mypos.score==130 && mypos.x==1 && mypos.y==4{
        mypos.state = false;
        commands.spawn(AudioPlayer::new(music_assets.coin.clone())).insert(NeedReload);
        let wr_layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 4, 1, None, None);
        let wr_atlas_layout = texture_atlases.add(wr_layout);
        let wr_atlas = TextureAtlas{
            layout : wr_atlas_layout,
            index : 0,
        };
        let wr_image = image_assets.warp.clone();
        let wr = spawn_once_warp(&mut commands,&wr_image,&wr_atlas,BASEX,BASEY-96.,BASEX+1600.,BASEY-1216.+64.);
        commands.entity(wr).insert(
            ChangeBuilding{
                delta: 4,
                to: 0,
            }
        );
    }
}

