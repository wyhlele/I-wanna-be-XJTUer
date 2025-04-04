use bevy::prelude::*;

use crate::state::GameState;

#[derive(Resource, Debug, Default)]
pub struct ImageAssets{
    pub kid : Handle<Image>,
    pub bg: Handle<Image>,
    pub bullet: Handle<Image>,
    pub spike: Handle<Image>,
    pub save: Handle<Image>,
    pub gameover: Handle<Image>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin{
    fn build(&self,app: &mut App){
        app.init_resource::<ImageAssets>()
            .add_systems(Startup,load_assets);
    }
}

fn load_assets(
    mut image_assets: ResMut<ImageAssets>, 
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<GameState>>,
){
    *image_assets = ImageAssets{
        kid: asset_server.load("images/kid_all2.png"),
        bg: asset_server.load("images/bg.png"),
        bullet: asset_server.load("images/bullet.png"),
        spike: asset_server.load("images/spike.png"),
        save: asset_server.load("images/save.png"),
        gameover: asset_server.load("images/gameover.png"),
    };
    next_state.set(GameState::InGame);
}