use bevy::prelude::*;

use crate::state::GameState;

#[derive(Resource, Debug, Default)]
pub struct ImageAssets{
    pub apple: Handle<Image>,
    pub bike: Handle<Image>,
    pub bullet: Handle<Image>,
    pub fest1_1: Handle<Image>,
    pub fest1_2: Handle<Image>,
    pub f2_up: Handle<Image>,
    pub gameover: Handle<Image>,
    pub kid : Handle<Image>,
    pub leaf : Handle<Image>,
    pub save: Handle<Image>,
    pub spike: Handle<Image>,
    pub warp: Handle<Image>,
}

#[derive(Resource, Debug, Default)]
pub struct SceneAssets{
    pub bg: Handle<Image>,
    pub festival1: Handle<Image>,
    pub festival2: Handle<Image>,
    pub f2_timer: Handle<Image>,
    pub yellow: Handle<Image>,
}

#[derive(Resource, Debug, Default)]
pub struct BackGroundAssets{
    pub gate: Handle<Image>,
    pub street: Handle<Image>,
}

#[derive(Resource, Debug, Default)]
pub struct MusicAssets{
    bgm_festival: Handle<AudioSource>,
    dead: Handle<AudioSource>,
    jump1: Handle<AudioSource>,
    jump2: Handle<AudioSource>,
    trig: Handle<AudioSource>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin{
    fn build(&self,app: &mut App){
        app.init_resource::<ImageAssets>()
            .init_resource::<SceneAssets>()
            .init_resource::<BackGroundAssets>()
            .add_systems(Startup,load_assets);
    }
}

fn load_assets(
    mut image_assets: ResMut<ImageAssets>, 
    mut scene_assets: ResMut<SceneAssets>, 
    mut background_assets: ResMut<BackGroundAssets>, 
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<GameState>>,
){
    
    
    *image_assets = ImageAssets{
        apple: asset_server.load("images/apple.png"),
        bike: asset_server.load("images/bike.png"),
        bullet: asset_server.load("images/bullet.png"),
        fest1_1: asset_server.load("images/fest1-1.png"),
        fest1_2: asset_server.load("images/fest1-2.png"),
        f2_up: asset_server.load("images/f2_up.png"),
        gameover: asset_server.load("images/gameover.png"),
        kid: asset_server.load("images/kid_all2.png"),
        leaf: asset_server.load("images/leaf.png"),
        save: asset_server.load("images/save.png"),
        spike: asset_server.load("images/spike.png"),
        warp: asset_server.load("images/warp.png"),
    };

    *scene_assets = SceneAssets{
        bg: asset_server.load("scene/bg.png"),
        festival1: asset_server.load("scene/festival1.png"),
        festival2: asset_server.load("scene/festival2.png"),
        f2_timer: asset_server.load("scene/17_59_18_00.png"),
        yellow: asset_server.load("scene/yellow.png"),
    };

    *background_assets = BackGroundAssets{
        gate: asset_server.load("background/gate2.png"),
        street: asset_server.load("background/street.png"),
    };

    next_state.set(GameState::InGame);
}