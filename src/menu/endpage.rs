use bevy::prelude::*;
use bevy::sprite::Sprite;

const BASEX: f32 = -2.*800.;
const BASEY: f32 = 0.0;

use crate::asset_loader::{BackGroundAssets, ImageAssets, SceneAssets};
use crate::base::ground::spawn_single_box;
use crate::base::savepointer::spawn_single_savepointer;
use crate::base::wrap::spawn_single_warp;
use crate::state::BGMReload;

pub struct EndPagePlugin;
impl Plugin for EndPagePlugin{
    fn build(&self, app: &mut App){
        app.add_systems(PostStartup,spawn_menu);
    }
}

fn spawn_menu(
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
    bg_assets: Res<BackGroundAssets>,
    image_assets: Res<ImageAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
){
    commands.spawn(
        Sprite{
            image: bg_assets.gate.clone(),
            ..Default::default()
        }
    ).insert(
        Transform::from_xyz(BASEX, BASEY, -0.5)
    );
    
    spawn_single_box(&mut commands,0.,-9.,BASEX,BASEY,12.5,0.5);
    spawn_single_box(&mut commands,-13.,-0.,BASEX,BASEY,0.5,9.5);
    spawn_single_box(&mut commands,13.,-0.,BASEX,BASEY,0.5,9.5);

    let gd_image = scene_assets.yellow.clone();
    let gd_layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 4, 4, None, None);
    let gd_atlas_layout = texture_atlases.add(gd_layout);
    let gd_atlas = TextureAtlas{
        layout : gd_atlas_layout,
        index : 14,
    };
    let sprite = Sprite{
        image: gd_image.clone(),
        texture_atlas: Some(gd_atlas.clone()),
        ..Default::default()
    };
    let mut x: f32 = -384.0;
    while x <= 384.0{
        commands.spawn(sprite.clone()).insert(
            Transform::from_xyz(BASEX+x,BASEY-288.0,0.0)
        );
        x += 32.0;
    };
    commands.spawn(
        Sprite{
            image: scene_assets.thanks.clone(),
            ..Default::default()
        }
    ).insert(
        Transform::from_xyz(BASEX,BASEY,0.0)
    );

    let wr_layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 4, 1, None, None);
    let wr_atlas_layout = texture_atlases.add(wr_layout);
    let wr_atlas = TextureAtlas{
        layout : wr_atlas_layout,
        index : 0,
    };
    let wr_image = image_assets.warp.clone();

    let warp = spawn_single_warp(&mut commands,&wr_image,&wr_atlas,BASEX+384.,BASEY-256.,0.,0.);
    commands.entity(warp).insert(BGMReload{id:0});

    let sv_layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 2, 1, None, None);
    let sv_atlas_layout = texture_atlases.add(sv_layout);
    let sv_atlas = TextureAtlas{
        layout : sv_atlas_layout,
        index : 0,
    };
    let sv_image = image_assets.save.clone();
    spawn_single_savepointer(&mut commands,&sv_image,&sv_atlas,0.,-8.,BASEX,BASEY,14);
}