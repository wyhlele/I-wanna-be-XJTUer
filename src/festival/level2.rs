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

const BASEX: f32 = 0.0;
const BASEY: f32 = 608.0*2.;

pub struct Fest2Plugin;

impl Plugin for Fest2Plugin{
    fn build(&self, app: &mut App){
        app.add_systems(PostStartup,spawn_once)
        .add_systems(OnExit(GameState::Reload),spawn_reload)
        .add_systems(Update,(do_trap1,do_trap2,do_bike,do_trap4));
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
struct Trap1;

#[derive(Component, Debug)]
struct Trap2;

#[derive(Component, Debug)]
pub struct Bike;

#[derive(Component, Debug)]
struct Trap4;

#[derive(Component, Debug)]
struct Trap5;

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
            image: scene_assets.festival2.clone(),
            ..Default::default()
        }
    ).insert(
        Transform::from_xyz(BASEX, BASEY, -0.35)
    );

    spawn_single_box(&mut commands,4.,-9.,BASEX,BASEY,8.5,0.5);
    spawn_single_box(&mut commands,-8.,-8.5,BASEX,BASEY,1.5,1.);
    spawn_single_box(&mut commands,-11.5,-9.,BASEX,BASEY,1.,0.5);
    spawn_single_box(&mut commands,-7.5,-7.,BASEX,BASEY,1.,0.5);
    spawn_single_box(&mut commands,4.5,-7.,BASEX,BASEY,2.,0.5);
    spawn_single_box(&mut commands,10.5,-7.,BASEX,BASEY,2.,0.5);
    spawn_single_box(&mut commands,7.5,-8.,BASEX,BASEY,5.,0.5);
    spawn_single_box(&mut commands,3.5,-6.,BASEX,BASEY,1.,0.5);
    spawn_single_box(&mut commands,11.,-6.,BASEX,BASEY,1.5,0.5);
    spawn_single_box(&mut commands,-1.5,-6.,BASEX,BASEY,3.,0.5);
    spawn_single_box(&mut commands,12.,-4.5,BASEX,BASEY,0.5,1.);
    spawn_single_box(&mut commands,-12.,2.,BASEX,BASEY,0.5,7.5);
    spawn_single_box(&mut commands,-10.5,-1.,BASEX,BASEY,1.,1.5);
    spawn_single_box(&mut commands,-5.,-1.,BASEX,BASEY,1.5,1.5);
    spawn_single_box(&mut commands,3.,-1.,BASEX,BASEY,2.5,1.5);
    spawn_single_box(&mut commands,8.,-1.5,BASEX,BASEY,2.5,1.);
    spawn_single_box(&mut commands,-1.,-2.,BASEX,BASEY,11.,0.5);
    spawn_single_box(&mut commands,1.,1.,BASEX,BASEY,0.5,0.5);
    spawn_single_box(&mut commands,1.5,4.,BASEX,BASEY,11.,0.5);
    spawn_single_box(&mut commands,2.5,6.,BASEX,BASEY,3.,0.5);
    spawn_single_box(&mut commands,0.,9.,BASEX,BASEY,12.5,0.5);
    

    spawn_single_box(&mut commands,-13.,0.,BASEX,BASEY,0.5,9.5);
    spawn_single_box(&mut commands,13.,0.,BASEX,BASEY,0.5,9.5);

    spawn_single_spike_fixed(&mut commands, &image_assets.spike, -10., -9., BASEX,BASEY,0.0);
    spawn_single_spike_fixed(&mut commands, &image_assets.spike, -9., -7., BASEX,BASEY,180.0);
    spawn_single_spike_fixed(&mut commands, &image_assets.spike, -9., -6., BASEX,BASEY,0.0);
    spawn_single_spike_fixed(&mut commands, &image_assets.spike, -6., -9., BASEX,BASEY,0.0);
    spawn_single_spike_fixed(&mut commands, &image_assets.spike, -5., -9., BASEX,BASEY,0.0);
    spawn_single_spike_fixed(&mut commands, &image_assets.spike, 8., -7., BASEX,BASEY,0.0);
    spawn_single_spike_fixed(&mut commands, &image_assets.spike, 9., -6., BASEX,BASEY,0.0);
    spawn_single_spike_fixed(&mut commands, &image_assets.spike, 10., -5., BASEX,BASEY,0.0);
    spawn_single_spike_fixed(&mut commands, &image_assets.spike, 7., 0., BASEX,BASEY,0.0);
    spawn_single_spike_fixed(&mut commands, &image_assets.spike, 8., 0., BASEX,BASEY,0.0);
    spawn_single_spike_fixed(&mut commands, &image_assets.spike, 5., 1., BASEX,BASEY,0.0);
    spawn_single_spike_fixed(&mut commands, &image_assets.spike, -9., -1., BASEX,BASEY,0.0);
    spawn_single_spike_fixed(&mut commands, &image_assets.spike, -8., -1., BASEX,BASEY,0.0);
    spawn_single_spike_fixed(&mut commands, &image_assets.spike, -7., -1., BASEX,BASEY,0.0);
    spawn_single_spike_fixed(&mut commands, &image_assets.spike, -3., -1., BASEX,BASEY,0.0);
    spawn_single_spike_fixed(&mut commands, &image_assets.spike, -2., -1., BASEX,BASEY,0.0);
    spawn_single_spike_fixed(&mut commands, &image_assets.spike, -1., -1., BASEX,BASEY,0.0);
    spawn_single_spike_fixed(&mut commands, &image_assets.spike,0., -1., BASEX,BASEY,0.0);
    spawn_single_spike_fixed(&mut commands, &image_assets.spike,2., 8., BASEX,BASEY,180.0);


    let sv_layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 2, 1, None, None);
    let sv_atlas_layout = texture_atlases.add(sv_layout);
    let sv_atlas = TextureAtlas{
        layout : sv_atlas_layout,
        index : 0,
    };
    let sv_image = image_assets.save.clone();
    spawn_single_savepointer(&mut commands,&sv_image,&sv_atlas,-11.,-8.,BASEX,BASEY,2);


    commands.spawn(Collider::cuboid(16.0, 96.0))
    .insert(Transform::from_xyz(BASEX+64.,BASEY-176., 0.0))
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

    commands.spawn(Collider::cuboid(16.0, 32.0))
    .insert(Transform::from_xyz(BASEX,BASEY+240., 0.0))
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

    commands.spawn(Collider::cuboid(16.0, 16.0))
    .insert(Transform::from_xyz(BASEX+128.,BASEY+160., 0.0))
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

    commands.spawn(Collider::cuboid(16.0, 64.0))
    .insert(Transform::from_xyz(BASEX-256.,BASEY+208., 0.0))
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


    let wr_layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 4, 1, None, None);
    let wr_atlas_layout = texture_atlases.add(wr_layout);
    let wr_atlas = TextureAtlas{
        layout : wr_atlas_layout,
        index : 0,
    };
    let wr_image = image_assets.warp.clone();

    let warp = spawn_single_warp(&mut commands,&wr_image,&wr_atlas,BASEX+96.,BASEY+224.,0.,0.);
    commands.entity(warp).insert(Leaf{score:-3}); 
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
        index : 6,
    };
    spawn_single_toucher(&mut commands,&bg_image,&in_bg_atlas,-9.,-1.,BASEX,BASEY);
    spawn_single_toucher(&mut commands,&bg_image,&in_bg_atlas,-8.,-1.,BASEX,BASEY);
    spawn_single_toucher(&mut commands,&bg_image,&in_bg_atlas,-7.,-1.,BASEX,BASEY);
    spawn_single_toucher(&mut commands,&bg_image,&in_bg_atlas,-3.,-1.,BASEX,BASEY);
    spawn_single_toucher(&mut commands,&bg_image,&in_bg_atlas,-2.,-1.,BASEX,BASEY);
    spawn_single_toucher(&mut commands,&bg_image,&in_bg_atlas,-1.,-1.,BASEX,BASEY);
    spawn_single_toucher(&mut commands,&bg_image,&in_bg_atlas,0.,-1.,BASEX,BASEY);

    let u_bg_atlas = TextureAtlas{
        layout : bg_atlas_layout.clone(),
        index : 14,
    };
    spawn_single_toucher(&mut commands,&bg_image,&u_bg_atlas,-9.,0.,BASEX,BASEY);
    spawn_single_toucher(&mut commands,&bg_image,&u_bg_atlas,-8.,0.,BASEX,BASEY);
    spawn_single_toucher(&mut commands,&bg_image,&u_bg_atlas,-7.,0.,BASEX,BASEY);
    spawn_single_toucher(&mut commands,&bg_image,&u_bg_atlas,-3.,0.,BASEX,BASEY);
    spawn_single_toucher(&mut commands,&bg_image,&u_bg_atlas,-2.,0.,BASEX,BASEY);
    spawn_single_toucher(&mut commands,&bg_image,&u_bg_atlas,-1.,0.,BASEX,BASEY);
    spawn_single_toucher(&mut commands,&bg_image,&u_bg_atlas,0.,0.,BASEX,BASEY);


    let l_bg_atlas = TextureAtlas{
        layout : bg_atlas_layout.clone(),
        index : 10,
    };
    spawn_single_hidden(&mut commands,&bg_image,&l_bg_atlas,-4.,-8.,BASEX,BASEY);
    spawn_single_hidden(&mut commands,&bg_image,&l_bg_atlas,-4.,-7.,BASEX,BASEY);


    let lf_image = image_assets.leaf.clone();
    spawn_single_leaf(&mut commands,&lf_image,-1.,-8.,BASEX,BASEY,1);
    spawn_single_leaf(&mut commands,&lf_image,0.,1.,BASEX,BASEY,1);
    spawn_single_leaf(&mut commands,&lf_image,4.,5.,BASEX,BASEY,1);

    let t1 = spawn_single_spike(&mut commands, &image_assets.spike, 2., -6., BASEX, BASEY, 0.0);
    commands.entity(t1)
    .insert(Trap1)
    .insert(Move{
        goal_pos: Vec2::new(BASEX+64.,BASEY-192.),
        linear_speed: 0.,
        status: 0,
        ..Default::default()
    }).insert(NeedReload);


    commands.spawn(
        Sprite{
            image: image_assets.f2_up.clone(),
            ..Default::default()
        },
    ).insert(Trap2)
    .insert(Trap)
    .insert(Transform::from_xyz(BASEX+16., BASEY-192., 0.1))
    .insert(Velocity::zero())
    .insert(RigidBody::Dynamic)
    .insert(Collider::cuboid(30.0, 10.0))
    .insert(GravityScale(0.0))
    .insert(
        CollisionGroups::new(
            Group::GROUP_3,
            Group::GROUP_1|Group::GROUP_4,
        )
    ).insert(SolverGroups::new(
        Group::GROUP_3,
        Group::GROUP_1,
        )
    ).insert(Move{
        goal_pos: Vec2::new(BASEX+16.,BASEY+224.),
        linear_speed: 0.,
        status: 0,
        ..Default::default()
    }).insert(NeedReload);


    commands.spawn(
        Sprite{
            image: image_assets.bike.clone(),
            ..Default::default()
        },
    ).insert(Bike)
    .insert(Trap)
    .insert(
        Transform{
            translation: Vec3::new(BASEX+304., BASEY+160., -0.4),
            scale: Vec3::new(0.09,0.09,1.),
            ..Default::default()
        },
    )
    .insert(Velocity::zero())
    .insert(RigidBody::Dynamic)
    .insert(Collider::cuboid(30.0, 10.0))
    .insert(GravityScale(0.0))
    .insert(
        CollisionGroups::new(
            Group::GROUP_3,
            Group::GROUP_1|Group::GROUP_4,
        )
    ).insert(SolverGroups::new(
        Group::GROUP_3,
        Group::GROUP_1,
        )
    ).insert(Move{
        goal_pos: Vec2::new(BASEX-272.,BASEY+160.),
        linear_speed: 0.,
        status: 0,
        ..Default::default()
    }).insert(NeedReload);


    let tm_image = scene_assets.f2_timer.clone();
    let tm_layout = TextureAtlasLayout::from_grid(UVec2::new(39, 16), 1, 2, None, None);
    let tm_atlas_layout = texture_atlases.add(tm_layout);
    let tm_atlas = TextureAtlas{
        layout : tm_atlas_layout.clone(),
        index : 0,
    };
    commands.spawn(
        Sprite{
            image: tm_image,
            texture_atlas: Some(tm_atlas),
            ..Default::default()
        },
    ).insert(Trap4)
    .insert(Trap)
    .insert(Transform::from_xyz(BASEX+375., BASEY+222., 0.1))
    .insert(NeedReload);


    let t51 = spawn_single_spike(&mut commands, &image_assets.spike, 4., 8., BASEX, BASEY, 180.0);
    commands.entity(t51)
    .insert(Trap5)
    .insert(Move{
        goal_pos: Vec2::new(BASEX+256.,BASEY+256.),
        linear_speed: 0.,
        status: 0,
        ..Default::default()
    }).insert(NeedReload);

    let t52 = spawn_single_spike(&mut commands, &image_assets.spike, 4., 7., BASEX, BASEY, 0.0);
    commands.entity(t52)
    .insert(Trap5)
    .insert(Move{
        goal_pos: Vec2::new(BASEX+256.,BASEY+224.),
        linear_speed: 0.,
        status: 0,
        ..Default::default()
    }).insert(NeedReload);
}


fn do_trap1(
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
                        trap1.goal_pos = Vec2::new(BASEX+64.,BASEY+224.);
                        trap1.linear_speed = 500.;
                        trap1.status = 1;
                        commands.spawn(AudioPlayer::new(music_assets.trap.clone()));
                    }else if trap1.status==1{
                        trap1.goal_pos = Vec2::new(BASEX+64.,BASEY-192.);
                        trap1.linear_speed = 100.;
                        trap1.status = 2;
                        commands.spawn(AudioPlayer::new(music_assets.trap.clone()));
                    }
                }
            }
            _ => {}
        }
    }
}

fn do_trap2(
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
                    trap2.linear_speed = 2000.;
                    commands.spawn(AudioPlayer::new(music_assets.trap.clone()));
                }
            }
            _ => {}
        }
    }
}

fn do_bike(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    kid_query: Query<&Kid>,
    trig3_query: Query<&Trig3>,
    mut bike_query: Query<&mut Move,(With<Bike>,Without<Trap5>)>,
    leaf_num: Res<LeafNum>,
    mut trig5_query: Query<&mut Move,(With<Trap5>,Without<Bike>)>,
    music_assets: Res<MusicAssets>,
){
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity_a, entity_b, _) => {
                let is_entity1_b = kid_query.get(*entity_b).is_ok();
                let is_entity2_a = trig3_query.get(*entity_a).is_ok();
                let is_entity1_a = kid_query.get(*entity_a).is_ok();
                let is_entity2_b = trig3_query.get(*entity_b).is_ok();
                let Ok(mut bike) = bike_query.get_single_mut()
                else{
                    continue;
                };
                if is_entity1_b && is_entity2_a || is_entity1_a && is_entity2_b{
                    if bike.linear_speed <200.{
                        bike.linear_speed = 200.;
                        commands.spawn(AudioPlayer::new(music_assets.trap.clone()));
                    }
                    if leaf_num.num==3 {
                        let mut flag = false;
                        for mut item in trig5_query.iter_mut(){
                            if item.linear_speed < 100.{
                                flag = true;
                            }
                            item.linear_speed = 100.;
                        }
                        if flag{
                            commands.spawn(AudioPlayer::new(music_assets.bike1.clone()));
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

fn do_trap4(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    kid_query: Query<&Kid>,
    trig4_query: Query<&Trig4>,
    mut trap4_query: Query<&mut Sprite,With<Trap4>>,
    music_assets: Res<MusicAssets>,
){
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity_a, entity_b, _) => {
                let is_entity1_b = kid_query.get(*entity_b).is_ok();
                let is_entity2_a = trig4_query.get(*entity_a).is_ok();
                let is_entity1_a = kid_query.get(*entity_a).is_ok();
                let is_entity2_b = trig4_query.get(*entity_b).is_ok();
                let Ok(mut trap4) = trap4_query.get_single_mut()
                else{
                    continue;
                };
                if is_entity1_b && is_entity2_a || is_entity1_a && is_entity2_b{
                    if let Some(atlas) = &mut trap4.texture_atlas{
                        if atlas.index == 0{
                            atlas.index = 1;
                            commands.spawn(AudioPlayer::new(music_assets.bell.clone()));
                        }
                    }
                }
            }
            _ => {}
        }
    }
}