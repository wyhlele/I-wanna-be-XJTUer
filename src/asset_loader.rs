use bevy::prelude::*;

use crate::state::GameState;

#[derive(Resource, Default)]
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
    pub tree: Handle<Image>,
    pub warp: Handle<Image>,
}

#[derive(Resource, Default)]
pub struct SceneAssets{
    pub bg: Handle<Image>,
    pub festival1: Handle<Image>,
    pub festival2: Handle<Image>,
    pub festival3: Handle<Image>,
    pub f2_timer: Handle<Image>,
    pub yellow: Handle<Image>,
}

#[derive(Resource, Default)]
pub struct BackGroundAssets{
    pub gate: Handle<Image>,
    pub street: Handle<Image>,
}

#[derive(Resource, Default)]
pub struct MusicAssets{
    pub building: Handle<AudioSource>,
    pub bell: Handle<AudioSource>,
    pub bike1: Handle<AudioSource>,
    pub bike2: Handle<AudioSource>,
    pub coin: Handle<AudioSource>,
    pub dead: Handle<AudioSource>,
    pub festival: Handle<AudioSource>,
    pub gate: Handle<AudioSource>,
    pub jump1: Handle<AudioSource>,
    pub jump2: Handle<AudioSource>,
    pub shoot: Handle<AudioSource>,
    pub trap: Handle<AudioSource>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin{
    fn build(&self,app: &mut App){
        app.init_resource::<ImageAssets>()
            .init_resource::<SceneAssets>()
            .init_resource::<BackGroundAssets>()
            .init_resource::<MusicAssets>()
            .add_systems(Startup,load_assets);
    }
}

fn load_assets(
    mut image_assets: ResMut<ImageAssets>, 
    mut scene_assets: ResMut<SceneAssets>, 
    mut background_assets: ResMut<BackGroundAssets>, 
    mut music_assets: ResMut<MusicAssets>, 
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<GameState>>,
){
    
    *image_assets = ImageAssets{
        apple: asset_server.load("images/apple.png"),
        bike: asset_server.load("images/bike1.png"),
        bullet: asset_server.load("images/bullet.png"),
        fest1_1: asset_server.load("images/fest1-1.png"),
        fest1_2: asset_server.load("images/fest1-2.png"),
        f2_up: asset_server.load("images/f2_up.png"),
        gameover: asset_server.load("images/gameover.png"),
        kid: asset_server.load("images/kid_all2.png"),
        leaf: asset_server.load("images/leaf.png"),
        save: asset_server.load("images/save.png"),
        spike: asset_server.load("images/spike.png"),
        tree: asset_server.load("images/tree.png"),
        warp: asset_server.load("images/warp.png"),
    };

    *scene_assets = SceneAssets{
        bg: asset_server.load("scene/bg.png"),
        festival1: asset_server.load("scene/festival1.png"),
        festival2: asset_server.load("scene/festival2.png"),
        festival3: asset_server.load("scene/festival3.png"),
        f2_timer: asset_server.load("scene/17_59_18_00.png"),
        yellow: asset_server.load("scene/yellow.png"),
    };

    *background_assets = BackGroundAssets{
        gate: asset_server.load("background/gate2.png"),
        street: asset_server.load("background/street.png"),
    };

    *music_assets = MusicAssets{
        building: asset_server.load("music/buildingbgm.ogg"),
        bell: asset_server.load("music/bell.ogg"),
        bike1: asset_server.load("music/bike1.ogg"),
        bike2: asset_server.load("music/bike2.ogg"),
        coin: asset_server.load("music/coin.ogg"),
        dead: asset_server.load("music/dead.ogg"),
        festival: asset_server.load("music/festivalbgm.ogg"),
        gate: asset_server.load("music/gatebgm.ogg"),
        jump1: asset_server.load("music/jump1.ogg"),
        jump2: asset_server.load("music/jump2.ogg"),
        shoot: asset_server.load("music/shoot.ogg"),
        trap: asset_server.load("music/trap.ogg"),
    };

    next_state.set(GameState::InGame);
}