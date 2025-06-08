use bevy::prelude::*;

use bevy::sprite::Sprite;
use crate::asset_loader::{BackGroundAssets, ImageAssets};
use crate::base::ground::spawn_single_box;
use crate::base::wrap::spawn_single_warp;

const BASEX: f32 = 800.0*3.;
const BASEY: f32 = 608.0;

pub struct BuildingTopPlugin;

impl Plugin for BuildingTopPlugin{
    fn build(&self,app: &mut App){
        app.add_systems(PostStartup,spawn_once);
    }
}


fn spawn_once(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    bg_assets: Res<BackGroundAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>
){
    commands.spawn(
        Sprite{
            image: bg_assets.top.clone(),
            ..Default::default()
        }
    ).insert(
        Transform::from_xyz(BASEX, BASEY, -0.5)
    );
    

    spawn_single_box(&mut commands,0.,-10.,BASEX,BASEY,12.5,0.5);
    spawn_single_box(&mut commands,-4.5,-3.,BASEX,BASEY,3.5,6.5);

    let wr_layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 4, 1, None, None);
    let wr_atlas_layout = texture_atlases.add(wr_layout);
    let wr_atlas = TextureAtlas{
        layout : wr_atlas_layout,
        index : 0,
    };
    let wr_image = image_assets.warp.clone();
    spawn_single_warp(&mut commands,&wr_image,&wr_atlas,BASEX-192.,BASEY+128.,1600.,192.);
}
