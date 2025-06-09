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
use crate::schedule::InGameSet;
use crate::state::{BGMReload, GameState, NeedReload};

use super::leaf::{spawn_fake_leaf, spawn_single_leaf, Leaf, LeafNum};

const BASEX: f32 = -800.0;
const BASEY: f32 = 608.0;

pub struct Fest3Plugin;

impl Plugin for Fest3Plugin{
    fn build(&self, app: &mut App){
        app.add_systems(PostStartup,spawn_once)
        .add_systems(OnExit(GameState::Reload),spawn_reload)
        .add_systems(Update,(do_trig1,do_trig2,do_trig3,do_trig4,do_trig5,do_trig6,do_trig7,do_trig8,do_trap8).in_set(InGameSet::EntityUpdates));
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
struct Trig5{
    state: i8
}

#[derive(Component, Debug)]
struct Trig6{
    state: i8
}

#[derive(Component, Debug)]
struct Trig7{
    state: i8
}

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

#[derive(Component, Debug)]
struct Trap8;

#[derive(Component, Debug)]
struct Trap9;

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
    spawn_single_savepointer(&mut commands,&sv_image,&sv_atlas,-10.,3.,BASEX,BASEY,3);

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


    commands.spawn(Collider::cuboid(16.0, 96.0))
    .insert(Transform::from_xyz(BASEX-272.,BASEY+176., 0.0))
    .insert(Trig1)
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

    commands.spawn(Collider::cuboid(16.0, 96.0))
    .insert(Transform::from_xyz(BASEX-144.,BASEY+176., 0.0))
    .insert(Trig2)
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

    commands.spawn(Collider::cuboid(16.0, 96.0))
    .insert(Transform::from_xyz(BASEX-16.,BASEY+176., 0.0))
    .insert(Trig3)
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

    commands.spawn(Collider::cuboid(16.0, 96.0))
    .insert(Transform::from_xyz(BASEX+112.,BASEY+176., 0.0))
    .insert(Trig4)
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

    commands.spawn(Collider::cuboid(16.0, 32.0))
    .insert(Transform::from_xyz(BASEX-16.,BASEY-240., 0.0))
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

    let wr_layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 4, 1, None, None);
    let wr_atlas_layout = texture_atlases.add(wr_layout);
    let wr_atlas = TextureAtlas{
        layout : wr_atlas_layout,
        index : 0,
    };
    let wr_image = image_assets.warp.clone();

    let warp = spawn_single_warp(&mut commands,&wr_image,&wr_atlas,BASEX+352.,BASEY-224.,-1600.,-1216.);
    commands.entity(warp).insert(Leaf{score:-3}).insert(BGMReload{id:4});
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

    let ori_image = scene_assets.bg.clone();
    let ori_layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 6, 9, None, None);
    let ori_atlas_layout = texture_atlases.add(ori_layout);
    let ori_atlas = TextureAtlas{
        layout : ori_atlas_layout.clone(),
        index : 53,
    };
    spawn_single_hidden(&mut commands,&ori_image,&ori_atlas,11.,-4.,BASEX,BASEY);
    let o1 = spawn_single_hidden(&mut commands,&ori_image,&ori_atlas,8.,-8.,BASEX,BASEY);
    commands.entity(o1).insert(Trap9);
    let o2 = spawn_single_hidden(&mut commands,&ori_image,&ori_atlas,8.,-7.,BASEX,BASEY);
    commands.entity(o2).insert(Trap9);

    let lf_image = image_assets.leaf.clone();
    spawn_single_leaf(&mut commands,&lf_image,11.,4.,BASEX,BASEY,1);
    spawn_single_leaf(&mut commands,&lf_image,11.,-5.,BASEX,BASEY,0);
    spawn_single_leaf(&mut commands,&lf_image,-11.,-1.,BASEX,BASEY,1);

    let t1 = spawn_fake_leaf(&mut commands, &lf_image, -7., 6., BASEX,BASEY);
    commands.entity(t1)
    .insert(Trap1)
    .insert(GravityScale(0.0))
    .insert(Velocity::zero())
    .insert(Move{
        goal_pos: Vec2::new(BASEX-224., BASEY+96.),
        linear_speed: 0.,
        ..Default::default()
    }).insert(Trap);

    let t2 = spawn_single_leaf(&mut commands, &lf_image, -3., 6., BASEX,BASEY,1);
    commands.entity(t2)
    .insert(Trap2)
    .insert(GravityScale(0.0))
    .insert(Velocity::zero())
    .insert(Move{
        goal_pos: Vec2::new(BASEX-96., BASEY+96.),
        linear_speed: 0.,
        ..Default::default()
    });

    let t3 = spawn_fake_leaf(&mut commands, &lf_image, 1., 6., BASEX,BASEY);
    commands.entity(t3)
    .insert(Trap3)
    .insert(Trap8)
    .insert(GravityScale(0.0))
    .insert(Velocity::zero())
    .insert(Move{
        goal_pos: Vec2::new(BASEX+32., BASEY+96.),
        linear_speed: 0.,
        ..Default::default()
    });
    let t8 = spawn_fake_leaf(&mut commands, &lf_image, 10., 2., BASEX,BASEY);
    commands.entity(t8)
    .insert(Trap8);

    let t5 = spawn_fake_leaf(&mut commands, &lf_image, 1., 6., BASEX,BASEY);
    commands.entity(t5)
    .insert(Trap5)
    .insert(Trap)
    .insert(GravityScale(0.0))
    .insert(Velocity::zero())
    .insert(Move{
        goal_pos: Vec2::new(BASEX+32., BASEY-416.),
        linear_speed: 0.,
        ..Default::default()
    });

    let t6 = spawn_fake_leaf(&mut commands, &lf_image, 1., 6., BASEX,BASEY);
    commands.entity(t6)
    .insert(Trap6)
    .insert(Trap)
    .insert(GravityScale(0.0))
    .insert(Velocity::zero())
    .insert(Move{
        goal_pos: Vec2::new(BASEX+32., BASEY-416.),
        linear_speed: 0.,
        ..Default::default()
    });



    let s1 = spawn_single_spike(&mut commands, &image_assets.spike, 9., 6., BASEX,BASEY,180.0);
    commands.entity(s1)
    .insert(Trap4)
    .insert(Move{
        goal_pos:Vec2::new(BASEX+288., BASEY+160.),
        linear_speed: 0.,
        ..Default::default()
    }).insert(NeedReload);
    let s2 = spawn_single_spike(&mut commands, &image_assets.spike, 9., 6., BASEX,BASEY,180.0);
    commands.entity(s2)
    .insert(Trap4)
    .insert(Move{
        goal_pos:Vec2::new(BASEX+288., BASEY+128.),
        linear_speed: 0.,
        ..Default::default()
    }).insert(NeedReload);
    
    commands.spawn(
        Sprite{
            image: image_assets.tree.clone(),
            ..Default::default()
        },
    ).insert(
        Transform::from_xyz(BASEX+32., BASEY+144., -0.4)
    ).insert(NeedReload)
    .insert(GravityScale(0.0))
    .insert(Velocity::zero())
    .insert(Trap7)
    .insert(Move{
        goal_pos: Vec2::new(BASEX+32., BASEY+144.),
        linear_speed: 200.,
        goal_angle: 0.,
        angle_speed: -5.,
        status: 0,
        ..Default::default()
    }).insert(
        RigidBody::Dynamic
    ).insert(
        CollisionGroups::new(
            Group::GROUP_4,
            Group::GROUP_1|Group::GROUP_4,
        )
    ).insert(SolverGroups::new(
        Group::GROUP_4,
        Group::NONE,
        )
    );

    commands.spawn(Collider::cuboid(16.0, 32.0))
    .insert(Transform::from_xyz(BASEX+320.,BASEY-16., 0.0))
    .insert(Trig5{state:0})
    .insert(
        CollisionGroups::new(
            Group::GROUP_4,
            Group::GROUP_1|Group::GROUP_4,
        )
    ).insert(SolverGroups::new(
        Group::GROUP_4,
        Group::NONE,
        )
    ).insert(NeedReload);

    commands.spawn(Collider::cuboid(16.0, 16.0))
    .insert(Transform::from_xyz(BASEX-96.,BASEY-176., 0.0))
    .insert(Trig6{state:0})
    .insert(
        CollisionGroups::new(
            Group::GROUP_4,
            Group::GROUP_1|Group::GROUP_4,
        )
    ).insert(SolverGroups::new(
        Group::GROUP_4,
        Group::NONE,
        )
    ).insert(NeedReload);

    commands.spawn(Collider::cuboid(2.0, 64.0))
    .insert(Transform::from_xyz(BASEX+48.,BASEY-48., 0.0))
    .insert(Trig7{state:0})
    .insert(
        CollisionGroups::new(
            Group::GROUP_4,
            Group::GROUP_1|Group::GROUP_4,
        )
    ).insert(SolverGroups::new(
        Group::GROUP_4,
        Group::NONE,
        )
    ).insert(NeedReload);
}



fn do_trig1(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    kid_query: Query<&Kid>,
    trig1_query: Query<&Trig1>,
    mut trap1_query: Query<&mut Move,With<Trap1>>,
    music_assets: Res<MusicAssets>,
){
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity_a, entity_b, _) => {
                let is_entity1_b = kid_query.get(*entity_b).is_ok();
                let is_entity2_a = trig1_query.get(*entity_a).is_ok();
                let is_entity1_a = kid_query.get(*entity_a).is_ok();
                let is_entity2_b = trig1_query.get(*entity_b).is_ok();
                let Ok(mut trap1) = trap1_query.get_single_mut()
                else{
                    continue;
                };
                if is_entity1_b && is_entity2_a || is_entity1_a && is_entity2_b{
                    if trap1.status==0{
                        trap1.linear_speed = 500.;
                        trap1.status = 1;
                        commands.spawn(AudioPlayer::new(music_assets.trap.clone())).insert(NeedReload);
                    }
                }
            }
            _ => {}
        }
    }
}

fn do_trig2(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    kid_query: Query<&Kid>,
    trig2_query: Query<&Trig2>,
    mut trap2_query: Query<&mut Move,With<Trap2>>,
    music_assets: Res<MusicAssets>,
){
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity_a, entity_b, _) => {
                let is_entity1_b = kid_query.get(*entity_b).is_ok();
                let is_entity2_a = trig2_query.get(*entity_a).is_ok();
                let is_entity1_a = kid_query.get(*entity_a).is_ok();
                let is_entity2_b = trig2_query.get(*entity_b).is_ok();
                let Ok(mut trap2) = trap2_query.get_single_mut()
                else{
                    continue;
                };
                if is_entity1_b && is_entity2_a || is_entity1_a && is_entity2_b{
                    if trap2.status==0{
                        trap2.linear_speed = 500.;
                        trap2.status = 1;
                        commands.spawn(AudioPlayer::new(music_assets.trap.clone())).insert(NeedReload);
                    }
                }
            }
            _ => {}
        }
    }
}

fn do_trig3(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    kid_query: Query<&Kid>,
    trig3_query: Query<&Trig3>,
    mut trap3_query: Query<&mut Move,With<Trap3>>,
    music_assets: Res<MusicAssets>,
){
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity_a, entity_b, _) => {
                let is_entity1_b = kid_query.get(*entity_b).is_ok();
                let is_entity2_a = trig3_query.get(*entity_a).is_ok();
                let is_entity1_a = kid_query.get(*entity_a).is_ok();
                let is_entity2_b = trig3_query.get(*entity_b).is_ok();
                let Ok(mut trap3) = trap3_query.get_single_mut()
                else{
                    continue;
                };
                if is_entity1_b && is_entity2_a || is_entity1_a && is_entity2_b{
                    if trap3.status==0{
                        trap3.linear_speed = 500.;
                        trap3.status = 1;
                        commands.spawn(AudioPlayer::new(music_assets.trap.clone())).insert(NeedReload);
                    }
                }
            }
            _ => {}
        }
    }
}

fn do_trig4(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    kid_query: Query<&Kid>,
    trig4_query: Query<&Trig4>,
    mut trap3_query: Query<&mut Move,With<Trap3>>,
    music_assets: Res<MusicAssets>,
){
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity_a, entity_b, _) => {
                let is_entity1_b = kid_query.get(*entity_b).is_ok();
                let is_entity2_a = trig4_query.get(*entity_a).is_ok();
                let is_entity1_a = kid_query.get(*entity_a).is_ok();
                let is_entity2_b = trig4_query.get(*entity_b).is_ok();
                let Ok(mut trap3) = trap3_query.get_single_mut()
                else{
                    continue;
                };
                if is_entity1_b && is_entity2_a || is_entity1_a && is_entity2_b{
                    if trap3.status==1{
                        trap3.goal_pos = Vec2::new(BASEX+352., BASEY+96.);
                        trap3.linear_speed = 500.;
                        trap3.status = 2;
                        commands.spawn(AudioPlayer::new(music_assets.trap.clone())).insert(NeedReload);
                    }
                }
            }
            _ => {}
        }
    }
}


fn do_trig5(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    kid_query: Query<&Kid>,
    mut trig5_query: Query<&mut Trig5>,
    scene_assets: Res<SceneAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
){
    let ori_image = scene_assets.bg.clone();
    let ori_layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 6, 9, None, None);
    let ori_atlas_layout = texture_atlases.add(ori_layout);
    let ori_atlas = TextureAtlas{
        layout : ori_atlas_layout.clone(),
        index : 53,
    };
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity_a, entity_b, _) => {
                let is_entity1_b = kid_query.get(*entity_b).is_ok();
                let is_entity2_a = trig5_query.get(*entity_a).is_ok();
                let is_entity1_a = kid_query.get(*entity_a).is_ok();
                let is_entity2_b = trig5_query.get(*entity_b).is_ok();
                if is_entity1_b && is_entity2_a || is_entity1_a && is_entity2_b{
                    let Ok(mut trig5) = trig5_query.get_single_mut()
                    else{
                        continue;
                    };
                    if trig5.state==0{
                        trig5.state = 1;
                        spawn_single_hidden(&mut commands, &ori_image, &ori_atlas, 8., 0., BASEX, BASEY);
                        spawn_single_hidden(&mut commands, &ori_image, &ori_atlas, 8., -1., BASEX, BASEY);
                    }
                }
            }
            _ => {}
        }
    }
}


fn do_trig6(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    kid_query: Query<&Kid>,
    mut trig6_query: Query<&mut Trig6>,
    scene_assets: Res<SceneAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
){
    let ori_image = scene_assets.bg.clone();
    let ori_layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 6, 9, None, None);
    let ori_atlas_layout = texture_atlases.add(ori_layout);
    let ori_atlas = TextureAtlas{
        layout : ori_atlas_layout.clone(),
        index : 53,
    };
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity_a, entity_b, _) => {
                let is_entity1_b = kid_query.get(*entity_b).is_ok();
                let is_entity2_a = trig6_query.get(*entity_a).is_ok();
                let is_entity1_a = kid_query.get(*entity_a).is_ok();
                let is_entity2_b = trig6_query.get(*entity_b).is_ok();
                if is_entity1_b && is_entity2_a || is_entity1_a && is_entity2_b{
                    let Ok(mut trig6) = trig6_query.get_single_mut()
                    else{
                        continue;
                    };
                    if trig6.state==0{
                        trig6.state = 1;
                        spawn_single_hidden(&mut commands, &ori_image, &ori_atlas, -3., -4., BASEX, BASEY);
                    }
                }
            }
            _ => {}
        }
    }
}

fn do_trig7(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    kid_query: Query<&Kid>,
    mut trig7_query: Query<&mut Trig7>,
    mut trap5_query: Query<&mut Move,(With<Trap5>,Without<Trap6>,Without<Trap7>)>,
    mut trap6_query: Query<&mut Move,(With<Trap6>,Without<Trap7>,Without<Trap5>)>,
    mut e7_query: Query<(Entity,&mut Move),With<Trap7>>,
    music_assets: Res<MusicAssets>,
){
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity_a, entity_b, _) => {
                let is_entity1_b = kid_query.get(*entity_b).is_ok();
                let is_entity2_a = trig7_query.get(*entity_a).is_ok();
                let is_entity1_a = kid_query.get(*entity_a).is_ok();
                let is_entity2_b = trig7_query.get(*entity_b).is_ok();
                let Ok(mut trig7) = trig7_query.get_single_mut()
                else{
                    continue;
                };
                if is_entity1_b && is_entity2_a || is_entity1_a && is_entity2_b{
                    if trig7.state == 0{
                        let Ok(mut trap5) = trap5_query.get_single_mut()
                        else{
                            continue;
                        };
                        trap5.linear_speed = 2000.;
                        trig7.state = 1;
                        commands.spawn(AudioPlayer::new(music_assets.trap.clone())).insert(NeedReload);
                    }else if trig7.state == 1{
                        let Ok(mut trap6) = trap6_query.get_single_mut()
                        else{
                            continue;
                        };
                        trap6.linear_speed = 2000.;
                        trig7.state = 2;
                        commands.spawn(AudioPlayer::new(music_assets.trap.clone())).insert(NeedReload);
                    }else if trig7.state == 2{
                        let Ok((e7,mut mv)) = e7_query.get_single_mut()
                        else{
                            continue;
                        };
                        commands.entity(e7).insert(Trap)
                        .insert(Collider::cuboid(48., 64.));
                        *mv = Move{
                            goal_pos: Vec2::new(BASEX+32., BASEY-48.),
                            linear_speed: 200.,
                            goal_angle: -180.,
                            angle_speed: -5.,
                            status: 1,
                            ..Default::default()
                        };
                        trig7.state = 3;
                        commands.spawn(AudioPlayer::new(music_assets.trap.clone())).insert(NeedReload);
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
    mut trap7_query: Query<&mut Move,With<Trap7>>,
    trap9_query:Query<Entity,With<Trap9>>,
    music_assets: Res<MusicAssets>,
    leaf_num: Res<LeafNum>,
){
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity_a, entity_b, _) => {
                let is_entity1_b = kid_query.get(*entity_b).is_ok();
                let is_entity2_a = trig8_query.get(*entity_a).is_ok();
                let is_entity1_a = kid_query.get(*entity_a).is_ok();
                let is_entity2_b = trig8_query.get(*entity_b).is_ok();
                if leaf_num.num == 3{
                    for item in trap9_query.iter(){
                        commands.entity(item).despawn_recursive();
                    }
                }
                let Ok(mut trap7) = trap7_query.get_single_mut()
                else{
                    info!("aaaa");
                    continue;
                };
                if is_entity1_b && is_entity2_a || is_entity1_a && is_entity2_b{
                    if trap7.status != 2{
                        trap7.goal_pos = Vec2::new(BASEX+32., BASEY-416.);
                        trap7.linear_speed = 500.;
                        trap7.status = 2;
                        trap7.goal_angle = -180.;
                        commands.spawn(AudioPlayer::new(music_assets.trap.clone())).insert(NeedReload);
                    }
                }
            }
            _ => {}
        }
    }
}

fn do_trap8(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    kid_query: Query<&Kid>,
    trap8_query: Query<&Trap8>,
    mut trap4_query: Query<&mut Move,With<Trap4>>,
    music_assets: Res<MusicAssets>,
){
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity_a, entity_b, _) => {
                let is_entity1_b = kid_query.get(*entity_b).is_ok();
                let is_entity2_a = trap8_query.get(*entity_a).is_ok();
                let is_entity1_a = kid_query.get(*entity_a).is_ok();
                let is_entity2_b = trap8_query.get(*entity_b).is_ok();
                if is_entity1_b && is_entity2_a || is_entity1_a && is_entity2_b{
                    for mut item in trap4_query.iter_mut(){
                        item.linear_speed = 1000.;
                    }
                    if is_entity2_a{
                        commands.entity(*entity_a).despawn_recursive();
                        commands.spawn(AudioPlayer::new(music_assets.coin.clone())).insert(NeedReload);
                    }else{
                        commands.entity(*entity_b).despawn_recursive();
                        commands.spawn(AudioPlayer::new(music_assets.coin.clone())).insert(NeedReload);
                    }
                }
            }
            _ => {}
        }
    }
}