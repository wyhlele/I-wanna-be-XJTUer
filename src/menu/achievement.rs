use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy::sprite::Sprite;

use std::fs::File;
use std::io::Write;
use std::path::Path;

const BASEX: f32 = 0.0;
const BASEY: f32 = 608.0;

use crate::asset_loader::{AchievementAssets, ImageAssets, MusicAssets, SceneAssets};
use crate::base::kid::Kid;
use crate::base::wrap::spawn_single_warp;
use crate::kid_saver::KidSaver;
use crate::schedule::InGameSet;

#[derive(Resource)]
struct AchieveTimer(Timer);

#[derive(Component, Debug, Default)]
pub struct Achievement{
    pub time: i16,
    pub id: i16,
}

#[derive(Component, Debug)]
struct Trig0;

#[derive(Component, Debug)]
pub struct Trig2;

#[derive(Component, Debug)]
struct Trig8;

pub struct AchievementPagePlugin;
impl Plugin for AchievementPagePlugin{
    fn build(&self, app: &mut App){
        app.add_systems(PostStartup,spawn_menu)
        .add_systems(Update,do_show)
        .add_systems(Update,(do_trig0,do_trig8).in_set(InGameSet::EntityUpdates));
    }
}

fn spawn_menu(
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
){
    commands.spawn(
        Sprite{
            image: scene_assets.world.clone(),
            ..Default::default()
        }
    ).insert(
        Transform{
            translation: Vec3::new(BASEX, BASEY+240., -0.4),
            scale: Vec3::new(0.67, 0.67, 0.0),
            ..Default::default()
        }
    );

    commands.spawn(Collider::cuboid(16.0, 304.0))
    .insert(Transform::from_xyz(BASEX+384.,BASEY, 0.0))
    .insert(Trig0)
    .insert(
        CollisionGroups::new(
            Group::GROUP_4,
            Group::GROUP_1|Group::GROUP_4,
        )
    ).insert(SolverGroups::new(
        Group::GROUP_4,
        Group::NONE,
        )
    );
    
    commands.spawn(Collider::ball(80.))
    .insert(Transform::from_xyz(-2.*800.-272.,-2.*608.+144., 0.0))
    .insert(Trig2)
    .insert(
        CollisionGroups::new(
            Group::GROUP_3,
            Group::GROUP_1|Group::GROUP_4,
        )
    ).insert(SolverGroups::new(
        Group::GROUP_3,
        Group::NONE,
        )
    );

    commands.spawn(Collider::cuboid(32.0, 32.0))
    .insert(Transform::from_xyz(-2.*800.,0., 0.0))
    .insert(Trig8)
    .insert(
        CollisionGroups::new(
            Group::GROUP_4,
            Group::GROUP_1|Group::GROUP_4,
        )
    ).insert(SolverGroups::new(
        Group::GROUP_4,
        Group::NONE,
        )
    );

    commands.insert_resource(AchieveTimer(Timer::from_seconds(0.04, TimerMode::Repeating)));

}


fn do_show(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<AchieveTimer>,
    mut query: Query<(Entity,&mut Achievement, &mut Transform),(With<Achievement>,Without<Camera2d>)>,
    camera_query: Query<&Transform,(With<Camera2d>, Without<Achievement>)>,
    mut kid_saver: ResMut<KidSaver>,
    achievement_assets: Res<AchievementAssets>,
    music_assets: Res<MusicAssets>,
    image_assets: Res<ImageAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>
){
    let Ok(camera) = camera_query.get_single()
    else{
        return;
    };
    timer.0.tick(time.delta());
    if timer.0.finished() {
        for (entity, mut achi, mut trans) in query.iter_mut(){
            if achi.time >= 137{
                trans.translation = Vec3::new(camera.translation.x+256., camera.translation.y-256.-((achi.time-137) as f32)*8., 0.5);
                if (kid_saver.achi>>achi.id)&1==0{
                    kid_saver.achi |= 1<<achi.id;
                    commands.spawn(AudioPlayer::new(music_assets.achievement.clone()));

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
                    let image;
                    let pos;
                    match achi.id {
                        0 =>{
                            image = achievement_assets.achievement0.clone();
                            pos = Vec3::new(BASEX+-256.,BASEY+144.,0.7);
                        },
                        1 =>{
                            image = achievement_assets.achievement1.clone();
                            pos = Vec3::new(BASEX+0.,BASEY+144.,0.7);
                        },
                        2 =>{
                            image = achievement_assets.achievement2.clone();
                            pos = Vec3::new(BASEX+256.,BASEY+144.,0.7);
                        },
                        3 =>{
                            image = achievement_assets.achievement3.clone();
                            pos = Vec3::new(BASEX-256.,BASEY+32.,0.7);
                        },
                        4 =>{
                            image = achievement_assets.achievement4.clone();
                            pos = Vec3::new(BASEX+0.,BASEY+32.,0.7);
                        },
                        5 =>{
                            image = achievement_assets.achievement5.clone();
                            pos = Vec3::new(BASEX+256.,BASEY+32.,0.7);
                        },
                        6 =>{
                            image = achievement_assets.achievement6.clone();
                            pos = Vec3::new(BASEX-256.,BASEY-80.,0.7);
                        },
                        7 =>{
                            image = achievement_assets.achievement7.clone();
                            pos = Vec3::new(BASEX+0.,BASEY-80.,0.7);
                        },
                        8 =>{
                            image = achievement_assets.achievement8.clone();
                            pos = Vec3::new(BASEX+256.,BASEY-80.,0.7);
                        },
                        _ =>{
                            image = achievement_assets.achievement9.clone();
                            pos = Vec3::new(BASEX+0.,BASEY-192.,0.8);
                        },
                    }
                    commands.spawn(
                        Sprite{
                            image: image,
                            ..Default::default()
                        }
                    ).insert(Transform{
                        translation: pos,
                        scale: Vec3::new(0.8,0.8,1.),
                        ..Default::default()
                    });
                }
            }else if achi.time<=12{
                trans.translation = Vec3::new(camera.translation.x+256., camera.translation.y-256.-((12-achi.time) as f32)*8., 0.5);
                if kid_saver.achi == 511{
                    commands.spawn(Achievement{time: 165, id: 9})
                    .insert(Sprite{
                        image: achievement_assets.achievement9.clone(),
                        ..Default::default()
                    })
                    .insert(Transform::from_xyz(0., 0., -5.0));
                    kid_saver.achi |= 1<<9;

                    commands.spawn(AudioPlayer::new(music_assets.fireworks.clone()));
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
                    commands.spawn(
                        Sprite{
                            image: achievement_assets.achievement9.clone(),
                            ..Default::default()
                        }
                    ).insert(Transform{
                        translation: Vec3::new(BASEX+0.,BASEY-192.,0.8),
                        scale: Vec3::new(0.8,0.8,1.),
                        ..Default::default()
                    });

                    let wr_layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 4, 1, None, None);
                    let wr_atlas_layout = texture_atlases.add(wr_layout);
                    let wr_atlas = TextureAtlas{
                        layout : wr_atlas_layout,
                        index : 0,
                    };
                    let wr_image = image_assets.warp.clone();

                    spawn_single_warp(&mut commands,&wr_image,&wr_atlas,-1600.-384.,-256.,BASEX,BASEY-256.);
                    spawn_single_warp(&mut commands,&wr_image,&wr_atlas,BASEX-384.,BASEY-256.,-1600.,-256.);
                }
            }else{
                trans.translation = Vec3::new(camera.translation.x+256., camera.translation.y-256., 0.5);
            }
            if achi.time < 0{
                commands.entity(entity).despawn_recursive();
            }
            achi.time -= 1;
        }
    }

}


fn do_trig0(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    kid_query: Query<&Kid>,
    trig0_query: Query<&Trig0>,
    achievement_assets: Res<AchievementAssets>,
    kid_saver: Res<KidSaver>,
){
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity_a, entity_b, _) => {
                let is_entity1_b = kid_query.get(*entity_b).is_ok();
                let is_entity2_a = trig0_query.get(*entity_a).is_ok();
                let is_entity1_a = kid_query.get(*entity_a).is_ok();
                let is_entity2_b = trig0_query.get(*entity_b).is_ok();
                if is_entity1_b && is_entity2_a || is_entity1_a && is_entity2_b{
                    if (kid_saver.achi>>0)&1==0{
                        commands.spawn(Achievement{time: 149, id: 0})
                        .insert(Sprite{
                            image: achievement_assets.achievement0.clone(),
                            ..Default::default()
                        }).insert(Transform::from_xyz(0., 0., -5.0));
                    }
                }
            }
            _ => {}
        }
    }
}

fn do_trig8(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    kid_query: Query<&Kid>,
    trig8_query: Query<&Trig8>,
    achievement_assets: Res<AchievementAssets>,
    kid_saver: Res<KidSaver>,
){
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity_a, entity_b, _) => {
                let is_entity1_b = kid_query.get(*entity_b).is_ok();
                let is_entity2_a = trig8_query.get(*entity_a).is_ok();
                let is_entity1_a = kid_query.get(*entity_a).is_ok();
                let is_entity2_b = trig8_query.get(*entity_b).is_ok();
                if is_entity1_b && is_entity2_a || is_entity1_a && is_entity2_b{
                    if (kid_saver.achi>>8)&1==0{
                        commands.spawn(Achievement{time: 149, id: 8})
                        .insert(Sprite{
                            image: achievement_assets.achievement8.clone(),
                            ..Default::default()
                        }).insert(Transform::from_xyz(0., 0., -5.0));
                    }
                }
            }
            _ => {}
        }
    }
}