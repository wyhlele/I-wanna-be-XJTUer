use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy::sprite::{TextureAtlas, TextureAtlasLayout,Sprite};
use crate::asset_loader::{BackGroundAssets, ImageAssets, MusicAssets, SceneAssets};
use crate::base::ground::spawn_single_box;
use crate::base::hidden::spawn_single_hidden;
use crate::base::kid::Kid;
use crate::base::moveto::Move;
use crate::base::savepointer::spawn_single_savepointer;
use crate::base::spike::{spawn_single_spike,spawn_single_spike_fixed};
use crate::base::toucher::spawn_single_toucher;
use crate::base::trap::Trap;
use crate::base::wrap::spawn_single_warp;
use crate::state::{GameState, NeedReload};

use super::leaf::{spawn_single_leaf, Leaf, LeafNum};

const BASEX: f32 = -800.0;
const BASEY: f32 = 608.0;

pub struct Fest3Plugin;

impl Plugin for Fest3Plugin{
    fn build(&self, app: &mut App){
        app.add_systems(PostStartup,spawn_once)
        .add_systems(OnExit(GameState::Reload),spawn_reload);
    }
}

#[derive(Component, Debug)]
struct Trig1;

#[derive(Component, Debug)]
struct Trig2;

#[derive(Component, Debug)]
struct Trig3;

#[derive(Component, Debug)]
struct Trig4;

#[derive(Component, Debug)]
struct Trig5;

#[derive(Component, Debug)]
struct Trig6;

#[derive(Component, Debug)]
struct Trig7;

#[derive(Component, Debug)]
struct Trig8;

#[derive(Component, Debug)]
struct Trap1;

#[derive(Component, Debug)]
struct Trap2;

#[derive(Component, Debug)]
struct Trap3;

#[derive(Component, Debug)]
struct Trap4;

#[derive(Component, Debug)]
struct Trap5;

#[derive(Component, Debug)]
struct Trap6;

#[derive(Component, Debug)]
struct Trap7;

fn spawn_once(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    scene_assets: Res<SceneAssets>,
    bg_assets: Res<BackGroundAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>
){
    commands.spawn(
        Sprite{
            image: bg_assets.street.clone(),
            ..Default::default()
        }
    ).insert(
        Transform::from_xyz(BASEX, BASEY, -0.5)
    );
    commands.spawn(
        Sprite{
            image: scene_assets.festival3.clone(),
            ..Default::default()
        }
    ).insert(
        Transform::from_xyz(BASEX, BASEY, -0.35)
    );
    spawn_single_box(&mut commands,-12.,0.,BASEX,BASEY,0.5,9.5);
    spawn_single_box(&mut commands,12.,0.,BASEX,BASEY,0.5,9.5);
    spawn_single_box(&mut commands,0.,-9.,BASEX,BASEY,12.5,0.5);
    spawn_single_box(&mut commands,0.,9.,BASEX,BASEY,12.5,0.5);
    spawn_single_box(&mut commands,2.,-6.,BASEX,BASEY,10.5,0.5);
    spawn_single_box(&mut commands,-6.,-4.5,BASEX,BASEY,2.5,1.0);
    spawn_single_box(&mut commands,4.,-4.,BASEX,BASEY,6.5,0.5);
    spawn_single_box(&mut commands,8.5,-3.,BASEX,BASEY,2.,0.5);
    spawn_single_box(&mut commands,9.,-2.,BASEX,BASEY,1.5,0.5);
    spawn_single_box(&mut commands,-11.,-2.,BASEX,BASEY,0.5,0.5);
    spawn_single_box(&mut commands,-3.5,1.5,BASEX,BASEY,8.,1.);
    spawn_single_box(&mut commands,8.,1.5,BASEX,BASEY,1.5,1.);
    spawn_single_box(&mut commands,10.5,1.,BASEX,BASEY,1.,0.5);
    spawn_single_box(&mut commands,9.,3.,BASEX,BASEY,0.5,0.5);
    spawn_single_box(&mut commands,9.5,7.5,BASEX,BASEY,2.,1.);

    spawn_single_spike_fixed(&mut commands, &image_assets.spike, 9., 6., BASEX,BASEY,180.0);
    
    let sv_layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 2, 1, None, None);
    let sv_atlas_layout = texture_atlases.add(sv_layout);
    let sv_atlas = TextureAtlas{
        layout : sv_atlas_layout,
        index : 0,
    };
    let sv_image = image_assets.save.clone();
    spawn_single_savepointer(&mut commands,&sv_image,&sv_atlas,-10.,3.,BASEX,BASEY,2);


    let wr_layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 4, 1, None, None);
    let wr_atlas_layout = texture_atlases.add(wr_layout);
    let wr_atlas = TextureAtlas{
        layout : wr_atlas_layout,
        index : 0,
    };
    let wr_image = image_assets.warp.clone();

    let warp = spawn_single_warp(&mut commands,&wr_image,&wr_atlas,BASEX+352.,BASEY-224.,0.,0.);
    commands.entity(warp).insert(Leaf{score:-3});

    commands.spawn(
        Sprite{
            image: image_assets.tree.clone(),
            ..Default::default()
        },
    ).insert(
        Transform::from_xyz(BASEX-224., BASEY+144., -0.4)
    );

    commands.spawn(
        Sprite{
            image: image_assets.tree.clone(),
            ..Default::default()
        },
    ).insert(
        Transform::from_xyz(BASEX-96., BASEY+144., -0.4)
    );
}

fn spawn_reload(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    scene_assets: Res<SceneAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>
){
    let bg_image = scene_assets.yellow.clone();
    let bg_layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 4, 4, None, None);
    let bg_atlas_layout = texture_atlases.add(bg_layout);
    let in_bg_atlas = TextureAtlas{
        layout : bg_atlas_layout.clone(),
        index : 7,
    };
    spawn_single_toucher(&mut commands,&bg_image,&in_bg_atlas,10.,2.,BASEX,BASEY);
    spawn_single_toucher(&mut commands,&bg_image,&in_bg_atlas,11.,2.,BASEX,BASEY);
    

    let u_bg_atlas = TextureAtlas{
        layout : bg_atlas_layout.clone(),
        index : 14,
    };
    spawn_single_toucher(&mut commands,&bg_image,&u_bg_atlas,10.,3.,BASEX,BASEY);
    spawn_single_toucher(&mut commands,&bg_image,&u_bg_atlas,11.,3.,BASEX,BASEY);


    spawn_single_hidden(&mut commands,&bg_image,&u_bg_atlas,11.,-4.,BASEX,BASEY);

    let ori_image = scene_assets.bg.clone();
    let ori_layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 6, 9, None, None);
    let ori_atlas_layout = texture_atlases.add(ori_layout);
    let ori_atlas = TextureAtlas{
        layout : ori_atlas_layout.clone(),
        index : 53,
    };
    spawn_single_hidden(&mut commands,&ori_image,&ori_atlas,8.,-8.,BASEX,BASEY);
    spawn_single_hidden(&mut commands,&ori_image,&ori_atlas,8.,-7.,BASEX,BASEY);


    let lf_image = image_assets.leaf.clone();
    spawn_single_leaf(&mut commands,&lf_image,11.,4.,BASEX,BASEY,1);
    spawn_single_leaf(&mut commands,&lf_image,11.,-5.,BASEX,BASEY,0);
    spawn_single_leaf(&mut commands,&lf_image,-11.,-1.,BASEX,BASEY,1);
    
    commands.spawn(
        Sprite{
            image: image_assets.tree.clone(),
            ..Default::default()
        },
    ).insert(
        Transform::from_xyz(BASEX+32., BASEY+144., -0.4)
    ).insert(NeedReload);
}


