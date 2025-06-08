use bevy::prelude::*;

use crate::state::GameState;

#[derive(Resource, Default)]
pub struct ImageAssets{
    pub apple: Handle<Image>,
    pub beam: Handle<Image>,
    pub bike: Handle<Image>,
    pub bullet: Handle<Image>,
    pub ferris: Handle<Image>,
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
    pub arrow: Handle<Image>,
    pub boss: Handle<Image>,
    pub bg: Handle<Image>,
    pub blood: Handle<Image>,
    pub building_center: Handle<Image>,
    pub building_e: Handle<Image>,
    pub discription_a: Handle<Image>,
    pub discription_b: Handle<Image>,
    pub discription_c: Handle<Image>,
    pub discription_d: Handle<Image>,
    pub festival1: Handle<Image>,
    pub festival2: Handle<Image>,
    pub festival3: Handle<Image>,
    pub f2_timer: Handle<Image>,
    pub hint: Handle<Image>,
    pub main_building: Handle<Image>,
    pub museum: Handle<Image>,
    pub numbers: Handle<Image>,
    pub quiz_title: Handle<Image>,
    pub right_eq: Handle<Image>,
    pub sjtu: Handle<Image>,
    pub thanks: Handle<Image>,
    pub title: Handle<Image>,
    pub unfilled: Handle<Image>,
    pub world: Handle<Image>,
    pub xjtu: Handle<Image>,
    pub yellow: Handle<Image>,
}

#[derive(Resource, Default)]
pub struct BackGroundAssets{
    pub center: Handle<Image>,
    pub classroom: Handle<Image>,
    pub gate: Handle<Image>,
    pub inside_e: Handle<Image>,
    pub library: Handle<Image>,
    pub museum: Handle<Image>,
    pub street: Handle<Image>,
    pub top: Handle<Image>,
}

#[derive(Resource, Default)]
pub struct MusicAssets{
    pub beam: Handle<AudioSource>,
    pub bell: Handle<AudioSource>,
    pub bike1: Handle<AudioSource>,
    pub bike2: Handle<AudioSource>,
    pub boss: Handle<AudioSource>,
    pub building: Handle<AudioSource>,
    pub coin: Handle<AudioSource>,
    pub dead: Handle<AudioSource>,
    pub festival: Handle<AudioSource>,
    pub gate: Handle<AudioSource>,
    pub hit: Handle<AudioSource>,
    pub jump1: Handle<AudioSource>,
    pub jump2: Handle<AudioSource>,
    pub museum: Handle<AudioSource>,
    pub shoot: Handle<AudioSource>,
    pub trap: Handle<AudioSource>,
    pub win: Handle<AudioSource>,
    pub xjtu: Handle<AudioSource>,
}


#[derive(Resource, Default)]
pub struct QuizAssets{
    pub a1: Handle<Image>,
    pub a2: Handle<Image>,
    pub a3: Handle<Image>,
    pub a4: Handle<Image>,
    pub a5: Handle<Image>,
    pub b1: Handle<Image>,
    pub b2: Handle<Image>,
    pub b3: Handle<Image>,
    pub b4: Handle<Image>,
    pub b5: Handle<Image>,
    pub c1: Handle<Image>,
    pub c2: Handle<Image>,
    pub c3: Handle<Image>,
    pub c4: Handle<Image>,
    pub c5: Handle<Image>,
    pub h1: Handle<Image>,
    pub h2: Handle<Image>,
    pub h3: Handle<Image>,
    pub h5: Handle<Image>,
    pub q1: Handle<Image>,
    pub q2: Handle<Image>,
    pub q3: Handle<Image>,
    pub q4: Handle<Image>,
    pub q5: Handle<Image>,
}

#[derive(Resource, Default)]
pub struct BuildingAssets{
    pub alphabeta: Handle<Image>,
    pub block0: Handle<Image>,
    pub block1: Handle<Image>,
    pub block2: Handle<Image>,
    pub block3: Handle<Image>,
    pub block4: Handle<Image>,
    pub block5: Handle<Image>,
    pub block6: Handle<Image>,
    pub block7: Handle<Image>,
    pub block8: Handle<Image>,
    pub block9: Handle<Image>,
    pub countmap: Handle<Image>,
    pub hua: Handle<Image>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin{
    fn build(&self,app: &mut App){
        app.init_resource::<ImageAssets>()
            .init_resource::<SceneAssets>()
            .init_resource::<BackGroundAssets>()
            .init_resource::<MusicAssets>()
            .init_resource::<QuizAssets>()
            .init_resource::<BuildingAssets>()
            .add_systems(Startup,load_assets);
    }
}

fn load_assets(
    mut image_assets: ResMut<ImageAssets>, 
    mut scene_assets: ResMut<SceneAssets>, 
    mut background_assets: ResMut<BackGroundAssets>, 
    mut music_assets: ResMut<MusicAssets>, 
    mut quiz_assets: ResMut<QuizAssets>, 
    mut building_assets: ResMut<BuildingAssets>, 
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<GameState>>,
){
    
    *image_assets = ImageAssets{
        apple: asset_server.load("images/apple.png"),
        beam: asset_server.load("images/beam.png"),
        bike: asset_server.load("images/bike1.png"),
        bullet: asset_server.load("images/bullet.png"),
        ferris: asset_server.load("images/ferris.png"),
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
        arrow: asset_server.load("scene/arrow.png"),
        boss: asset_server.load("scene/boss.png"),
        bg: asset_server.load("scene/bg.png"),
        blood: asset_server.load("scene/blood.png"),
        building_center: asset_server.load("scene/building_center.png"),
        building_e: asset_server.load("scene/building_E.png"),
        discription_a: asset_server.load("scene/discription_a.png"),
        discription_b: asset_server.load("scene/discription_b.png"),
        discription_c: asset_server.load("scene/discription_c.png"),
        discription_d: asset_server.load("scene/discription_d.png"),
        festival1: asset_server.load("scene/festival1.png"),
        festival2: asset_server.load("scene/festival2.png"),
        festival3: asset_server.load("scene/festival3.png"),
        f2_timer: asset_server.load("scene/17_59_18_00.png"),
        hint: asset_server.load("scene/hint.png"),
        main_building: asset_server.load("scene/main_building.png"),
        museum: asset_server.load("scene/quiz.png"),
        numbers: asset_server.load("scene/numbers.png"),
        right_eq: asset_server.load("scene/rightEq.png"),
        sjtu: asset_server.load("scene/sjtu.png"),
        title: asset_server.load("scene/title.png"),
        thanks: asset_server.load("scene/thanks.png"),
        quiz_title: asset_server.load("scene/quiz_title.png"),
        unfilled: asset_server.load("scene/unfilled.png"),
        world: asset_server.load("scene/world.png"),
        xjtu: asset_server.load("scene/xjtu.png"),
        yellow: asset_server.load("scene/yellow.png"),
    };

    *background_assets = BackGroundAssets{
        center: asset_server.load("background/center.png"),
        classroom: asset_server.load("background/classroom.png"),
        gate: asset_server.load("background/gate.png"),
        inside_e: asset_server.load("background/insideE.png"),
        library: asset_server.load("background/library.png"),
        museum: asset_server.load("background/museum.png"),
        street: asset_server.load("background/street.png"),
        top: asset_server.load("background/top.png"),
    };

    *music_assets = MusicAssets{
        beam: asset_server.load("music/beam.ogg"),
        bell: asset_server.load("music/bell.ogg"),
        bike1: asset_server.load("music/bike1.ogg"),
        bike2: asset_server.load("music/bike2.ogg"),
        boss: asset_server.load("music/boss.ogg"),
        building: asset_server.load("music/buildingbgm.ogg"),
        coin: asset_server.load("music/coin.ogg"),
        dead: asset_server.load("music/dead.ogg"),
        festival: asset_server.load("music/festivalbgm.ogg"),
        gate: asset_server.load("music/gatebgm.ogg"),
        hit: asset_server.load("music/hit.ogg"),
        jump1: asset_server.load("music/jump1.ogg"),
        jump2: asset_server.load("music/jump2.ogg"),
        museum: asset_server.load("music/museumbgm.ogg"),
        shoot: asset_server.load("music/shoot.ogg"),
        trap: asset_server.load("music/trap.ogg"),
        win: asset_server.load("music/win.ogg"),
        xjtu: asset_server.load("music/xjtu.ogg"),
    };

    *quiz_assets = QuizAssets{
        a1: asset_server.load("scene/1A.png"),
        a2: asset_server.load("scene/2A.png"),
        a3: asset_server.load("scene/3A.png"),
        a4: asset_server.load("scene/4A.png"),
        a5: asset_server.load("scene/5A.png"),
        b1: asset_server.load("scene/1B.png"),
        b2: asset_server.load("scene/2B.png"),
        b3: asset_server.load("scene/3B.png"),
        b4: asset_server.load("scene/4B.png"),
        b5: asset_server.load("scene/5B.png"),
        c1: asset_server.load("scene/1C.png"),
        c2: asset_server.load("scene/2C.png"),
        c3: asset_server.load("scene/3C.png"),
        c4: asset_server.load("scene/4C.png"),
        c5: asset_server.load("scene/5C.png"),
        h1: asset_server.load("scene/1H.png"),
        h2: asset_server.load("scene/2H.png"),
        h3: asset_server.load("scene/3H.png"),
        h5: asset_server.load("scene/5H.png"),
        q1: asset_server.load("scene/1Q.png"),
        q2: asset_server.load("scene/2Q.png"),
        q3: asset_server.load("scene/3Q.png"),
        q4: asset_server.load("scene/4Q.png"),
        q5: asset_server.load("scene/5Q.png"),
    };

    *building_assets = BuildingAssets{
        alphabeta: asset_server.load("images/alphabeta.png"),
        block0: asset_server.load("images/block0.png"),
        block1: asset_server.load("images/block1.png"),
        block2: asset_server.load("images/block2.png"),
        block3: asset_server.load("images/block3.png"),
        block4: asset_server.load("images/block4.png"),
        block5: asset_server.load("images/block5.png"),
        block6: asset_server.load("images/block6.png"),
        block7: asset_server.load("images/block7.png"),
        block8: asset_server.load("images/block8.png"),
        block9: asset_server.load("images/block9.png"),
        countmap: asset_server.load("images/countmap.png"),
        hua: asset_server.load("images/hua.png"),
    };

    next_state.set(GameState::InGame);
}