use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy::sprite::{TextureAtlas, TextureAtlasLayout,Sprite};
use crate::asset_loader::{BackGroundAssets, ImageAssets, MusicAssets, QuizAssets, SceneAssets};
use crate::base::ground::spawn_single_box;
use crate::base::kid::Kid;
use crate::base::savepointer::spawn_single_savepointer;
use crate::base::wrap::{spawn_fake_warp, spawn_single_warp};
use crate::schedule::InGameSet;
use crate::state::{GameState, NeedReload};

const BASEX: f32 = -2.*800.0;
const BASEY: f32 = -2.*608.0;

pub struct Quiz1Plugin;

impl Plugin for Quiz1Plugin{
    fn build(&self, app: &mut App){
        app.add_systems(PostStartup,spawn_once)
        .add_systems(OnExit(GameState::Reload),spawn_reload)
        .add_systems(Update,(do_wrong,do_accept).in_set(InGameSet::EntityUpdates));
    }
}

#[derive(Component, Debug)]
struct Ques;

#[derive(Component, Debug)]
struct Wrong;

#[derive(Component, Debug)]
struct Accept;

fn spawn_once(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    scene_assets: Res<SceneAssets>,
    bg_assets: Res<BackGroundAssets>,
    quiz_assets: Res<QuizAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>
){
    commands.spawn(
        Sprite{
            image: bg_assets.museum.clone(),
            ..Default::default()
        }
    ).insert(
        Transform::from_xyz(BASEX, BASEY, -0.5)
    );

    commands.spawn(
        Sprite{
            image: scene_assets.museum.clone(),
            ..Default::default()
        }
    ).insert(
        Transform::from_xyz(BASEX, BASEY, -0.3)
    );

    commands.spawn(
        Sprite{
            image: scene_assets.museum.clone(),
            ..Default::default()
        }
    ).insert(
        Transform::from_xyz(BASEX, BASEY, -0.5)
    );

    spawn_single_box(&mut commands,0.,-1.,BASEX,BASEY,2.5,0.5);
    spawn_single_box(&mut commands,0.,-9.,BASEX,BASEY,12.5,0.5);
    spawn_single_box(&mut commands,-3.5,-7.5,BASEX,BASEY,2.,1.);
    spawn_single_box(&mut commands,3.5,-7.5,BASEX,BASEY,2.,1.);
    spawn_single_box(&mut commands,-10.5,-7.5,BASEX,BASEY,2.,1.);
    spawn_single_box(&mut commands,10.5,-7.5,BASEX,BASEY,2.,1.);
    spawn_single_box(&mut commands,-11.,-6.,BASEX,BASEY,1.5,0.5);
    spawn_single_box(&mut commands,11.,-6.,BASEX,BASEY,1.5,0.5);
    spawn_single_box(&mut commands,-3.,-6.,BASEX,BASEY,1.5,0.5);
    spawn_single_box(&mut commands,3.,-6.,BASEX,BASEY,1.5,0.5);
    spawn_single_box(&mut commands,-3.5,-5.,BASEX,BASEY,1.,0.5);
    spawn_single_box(&mut commands,3.5,-5.,BASEX,BASEY,1.,0.5);
    spawn_single_box(&mut commands,-4.,-3.5,BASEX,BASEY,0.5,1.);
    spawn_single_box(&mut commands,4.,-3.5,BASEX,BASEY,0.5,1.);
    spawn_single_box(&mut commands,-11.5,-1.5,BASEX,BASEY,1.,4.);
    spawn_single_box(&mut commands,11.5,-1.5,BASEX,BASEY,1.,4.);
    spawn_single_box(&mut commands,-12.,6.,BASEX,BASEY,0.5,3.5);
    spawn_single_box(&mut commands,12.,6.,BASEX,BASEY,0.5,3.5);
    spawn_single_box(&mut commands,-11.,8.,BASEX,BASEY,0.5,1.5);
    spawn_single_box(&mut commands,10.,8.,BASEX,BASEY,0.5,0.5);
    spawn_single_box(&mut commands,-10.,8.,BASEX,BASEY,0.5,0.5);
    spawn_single_box(&mut commands,0.,9.,BASEX,BASEY,12.5,0.5);


    let sv_layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 2, 1, None, None);
    let sv_atlas_layout = texture_atlases.add(sv_layout);
    let sv_atlas = TextureAtlas{
        layout : sv_atlas_layout,
        index : 0,
    };
    let sv_image = image_assets.save.clone();
    spawn_single_savepointer(&mut commands,&sv_image,&sv_atlas,0.,0.,BASEX,BASEY,4);


    commands.spawn(
        Sprite{
            image: scene_assets.quiz_title.clone(),
            ..Default::default()
        },
    ).insert(
        Transform{
            translation: Vec3::new(BASEX, BASEY+272., -0.2),
            scale: Vec3::new(1.,1.,1.),
            ..Default::default()
        },
    );

    commands.spawn(
        Sprite{
            image: scene_assets.sjtu.clone(),
            ..Default::default()
        },
    ).insert(
        Transform{
            translation: Vec3::new(BASEX-272., BASEY+144., -0.1),
            scale: Vec3::new(0.15,0.15,1.),
            ..Default::default()
        },
    );


    commands.spawn(
        Sprite{
            image: quiz_assets.a1.clone(),
            ..Default::default()
        },
    ).insert(
        Transform{
            translation: Vec3::new(BASEX-224., BASEY-128., -0.2),
            scale: Vec3::new(1.,1.,1.),
            ..Default::default()
        },
    );

    commands.spawn(
        Sprite{
            image: quiz_assets.b1.clone(),
            ..Default::default()
        },
    ).insert(
        Transform{
            translation: Vec3::new(BASEX, BASEY-128., -0.2),
            scale: Vec3::new(1.,1.,1.),
            ..Default::default()
        },
    );

    commands.spawn(
        Sprite{
            image: quiz_assets.c1.clone(),
            ..Default::default()
        },
    ).insert(
        Transform{
            translation: Vec3::new(BASEX+224., BASEY-128., -0.2),
            scale: Vec3::new(1.,1.,1.),
            ..Default::default()
        },
    );

    let wr_layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 4, 1, None, None);
    let wr_atlas_layout = texture_atlases.add(wr_layout);
    let wr_atlas = TextureAtlas{
        layout : wr_atlas_layout,
        index : 0,
    };
    let wr_image = image_assets.warp.clone();
    
    let wp1 = spawn_single_warp(&mut commands,&wr_image,&wr_atlas,BASEX,BASEY-256.,BASEX+800.,BASEY+608.);
    let wp2 = spawn_fake_warp(&mut commands,&wr_image,&wr_atlas,BASEX-224.,BASEY-256.);
    let wp3 = spawn_fake_warp(&mut commands,&wr_image,&wr_atlas,BASEX+224.,BASEY-256.);
    
    commands.entity(wp1).insert(Accept);
    commands.entity(wp2).insert(Wrong);
    commands.entity(wp3).insert(Wrong);

}

fn spawn_reload(
    mut commands: Commands,
    quiz_assets: Res<QuizAssets>,
){
    commands.spawn(
        Sprite{
            image: quiz_assets.q1.clone(),
            ..Default::default()
        },
    ).insert(
        Transform{
            translation: Vec3::new(BASEX+80., BASEY+160., -0.1),
            scale: Vec3::new(1.,1.,1.),
            ..Default::default()
        },
    ).insert(Ques)
    .insert(NeedReload);
}

fn do_wrong(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    kid_query: Query<&Kid>,
    wrong_query: Query<&Wrong>,
    ques_query: Query<Entity,With<Ques>>,
    quiz_assets: Res<QuizAssets>,
){
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity_a, entity_b, _) => {
                let is_entity1_b = kid_query.get(*entity_b).is_ok();
                let is_entity2_a = wrong_query.get(*entity_a).is_ok();
                let is_entity1_a = kid_query.get(*entity_a).is_ok();
                let is_entity2_b = wrong_query.get(*entity_b).is_ok();
                if is_entity1_b && is_entity2_a || is_entity1_a && is_entity2_b{
                    for item in ques_query.iter(){
                        commands.entity(item).despawn_recursive();
                    }
                    commands.spawn(
                        Sprite{
                            image: quiz_assets.h1.clone(),
                            ..Default::default()
                        },
                    ).insert(
                        Transform{
                            translation: Vec3::new(BASEX+80., BASEY+144., -0.1),
                            scale: Vec3::new(1.,1.,1.),
                            ..Default::default()
                        },
                    ).insert(NeedReload);
                }
            }
            _ => {}
        }
    }
}

fn do_accept(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    kid_query: Query<&Kid>,
    accept_query: Query<&Accept>,
    music_assets: Res<MusicAssets>,
){
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity_a, entity_b, _) => {
                let is_entity1_b = kid_query.get(*entity_b).is_ok();
                let is_entity2_a = accept_query.get(*entity_a).is_ok();
                let is_entity1_a = kid_query.get(*entity_a).is_ok();
                let is_entity2_b = accept_query.get(*entity_b).is_ok();
                if is_entity1_b && is_entity2_a || is_entity1_a && is_entity2_b{
                    commands.spawn(AudioPlayer::new(music_assets.coin.clone())).insert(NeedReload);
                }
            }
            _ => {}
        }
    }
}