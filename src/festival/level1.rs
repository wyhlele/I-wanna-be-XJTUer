use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy::sprite::{TextureAtlas, TextureAtlasLayout,Sprite};
use crate::asset_loader::{ImageAssets,SceneAssets,BackGroundAssets};
use crate::base::ground::spawn_single_box;
use crate::base::hidden::spawn_single_hidden;
use crate::base::kid::Kid;
use crate::base::moveto::Move;
use crate::base::savepointer::spawn_single_savepointer;
use crate::base::spike::spawn_single_spike_fixed;
use crate::base::toucher::spawn_single_toucher;
use crate::base::trap::Trap;
use crate::base::wrap::spawn_single_warp;
use crate::state::{GameState, NeedReload};

use super::leaf::{spawn_single_leaf, Leaf, LeafNum};

const BASEX: f32 = 800.0;
const BASEY: f32 = 608.0;

pub struct Fest1Plugin;

impl Plugin for Fest1Plugin{
    fn build(&self, app: &mut App){
        app.add_systems(PostStartup,spawn_once)
        .add_systems(OnExit(GameState::Reload),spawn_reload)
        .add_systems(Update,(do_trap1,do_trap2,do_trap3));
    }
}

#[derive(Component, Debug)]
struct Trig1;

#[derive(Component, Debug)]
struct Trig2;

#[derive(Component, Debug)]
struct Trig3;

#[derive(Component, Debug)]
struct Trap1;

#[derive(Component, Debug)]
struct Trap2;

#[derive(Component, Debug)]
struct Trap3;

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
            image: scene_assets.festival1.clone(),
            ..Default::default()
        }
    ).insert(
        Transform::from_xyz(BASEX, BASEY, -0.3)
    );

    spawn_single_box(&mut commands,0.,-9.,BASEX,BASEY,12.5,0.5);
    spawn_single_box(&mut commands,-3.5,-8.,BASEX,BASEY,1.,0.5);
    spawn_single_box(&mut commands,-2.5,-7.,BASEX,BASEY,1.,0.5);
    spawn_single_box(&mut commands,0.,-6.,BASEX,BASEY,2.5,0.5);
    spawn_single_box(&mut commands,2.5,-5.,BASEX,BASEY,3.,0.5);
    spawn_single_box(&mut commands,8.,-4.,BASEX,BASEY,4.5,0.5);
    spawn_single_box(&mut commands,8.5,-3.,BASEX,BASEY,4.,0.5);
    spawn_single_box(&mut commands,-9.,-5.,BASEX,BASEY,3.5,0.5);
    spawn_single_box(&mut commands,-8.5,-4.,BASEX,BASEY,4.,0.5);
    spawn_single_box(&mut commands,-8.,-3.,BASEX,BASEY,4.5,0.5);
    spawn_single_box(&mut commands,-6.5,-2.,BASEX,BASEY,6.,0.5);
    spawn_single_box(&mut commands,-9.5,-0.5,BASEX,BASEY,3.,1.);
    spawn_single_box(&mut commands,-1.5,-1.5,BASEX,BASEY,3.,1.);
    spawn_single_box(&mut commands,-4.,-0.,BASEX,BASEY,0.5,0.5);
    spawn_single_box(&mut commands,-12.,4.5,BASEX,BASEY,0.5,4.);
    spawn_single_box(&mut commands,-11.,7.5,BASEX,BASEY,0.5,2.);
    spawn_single_box(&mut commands,1.,6.5,BASEX,BASEY,11.5,3.);
    spawn_single_box(&mut commands,13.,-0.,BASEX,BASEY,0.5,9.5);

    spawn_single_spike_fixed(&mut commands, &image_assets.spike, -5., -1., BASEX,BASEY,0.0);
    spawn_single_spike_fixed(&mut commands, &image_assets.spike, -6., -1., BASEX,BASEY,0.0);
    spawn_single_spike_fixed(&mut commands, &image_assets.spike, -12., -8., BASEX,BASEY,0.0);

    let sv_layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 2, 1, None, None);
    let sv_atlas_layout = texture_atlases.add(sv_layout);
    let sv_atlas = TextureAtlas{
        layout : sv_atlas_layout,
        index : 0,
    };
    let sv_image = image_assets.save.clone();
    spawn_single_savepointer(&mut commands,&sv_image,&sv_atlas,-9.,-8.,BASEX,BASEY,1);

    let wr_layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 4, 1, None, None);
    let wr_atlas_layout = texture_atlases.add(wr_layout);
    let wr_atlas = TextureAtlas{
        layout : wr_atlas_layout,
        index : 0,
    };
    let wr_image = image_assets.warp.clone();
    spawn_single_warp(&mut commands,&wr_image,&wr_atlas,BASEX-352.,BASEY+160.,BASEX+352.,BASEY-224.);
    spawn_single_warp(&mut commands,&wr_image,&wr_atlas,BASEX+352.,BASEY-160.,BASEX-352.,BASEY+98.);


    commands.spawn(Collider::cuboid(16.0, 16.0))
    .insert(Transform::from_xyz(BASEX-352.,BASEY+160., 0.0))
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

    commands.spawn(Collider::cuboid(32.0, 32.0))
    .insert(Transform::from_xyz(BASEX+16.,BASEY-240., 0.0))
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
    .insert(Transform::from_xyz(BASEX+288.,BASEY+16., 0.0))
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

    let warp = spawn_single_warp(&mut commands,&wr_image,&wr_atlas,BASEX+384.,BASEY+32.,-384.,960.);
    commands.entity(warp).insert(Leaf{score: -2});
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

    spawn_single_toucher(&mut commands,&bg_image,&in_bg_atlas,-5.,-1.,BASEX,BASEY);
    spawn_single_toucher(&mut commands,&bg_image,&in_bg_atlas,-6.,-1.,BASEX,BASEY);
    spawn_single_toucher(&mut commands,&bg_image,&in_bg_atlas,-11.,5.,BASEX,BASEY);

    let d_bg_atlas = TextureAtlas{
        layout : bg_atlas_layout.clone(),
        index : 4,
    };
    spawn_single_toucher(&mut commands,&bg_image,&d_bg_atlas,-11.,4.,BASEX,BASEY);

    let u_bg_atlas = TextureAtlas{
        layout : bg_atlas_layout.clone(),
        index : 14,
    };
    spawn_single_toucher(&mut commands,&bg_image,&u_bg_atlas,-5.,0.,BASEX,BASEY);
    spawn_single_toucher(&mut commands,&bg_image,&u_bg_atlas,-6.,0.,BASEX,BASEY);

    let ori_image = scene_assets.bg.clone();
    let ori_layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 6, 9, None, None);
    let ori_atlas_layout = texture_atlases.add(ori_layout);
    let ori_atlas = TextureAtlas{
        layout : ori_atlas_layout.clone(),
        index : 53,
    };

    let mut y: f32 = -2.;
    while y<=3. {
        let tmp = spawn_single_hidden(&mut commands,&ori_image,&ori_atlas,10.,y,BASEX,BASEY);
        commands.entity(tmp).insert(Trap2);
        y = y + 1.;
    }

    let lf_image = image_assets.leaf.clone();
    spawn_single_leaf(&mut commands,&lf_image,-10.,2.,BASEX,BASEY,1);
    spawn_single_leaf(&mut commands,&lf_image,3.,-6.,BASEX,BASEY,1);
    spawn_single_leaf(&mut commands,&lf_image,-2.,-8.,BASEX,BASEY,1);


    let tr1_image = image_assets.fest1_1.clone();
    commands.spawn(
        Sprite{
            image: tr1_image,
            ..Default::default()
        }
    ).insert(Transform::from_xyz(BASEX+160., BASEY-224., 0.0))
    .insert(Trap1)
    .insert(NeedReload);

    let tr3_image = image_assets.fest1_2.clone();
    commands.spawn(
        Sprite{
            image: tr3_image,
            ..Default::default()
        }
    ).insert(Transform::from_xyz(BASEX+278., BASEY+167., 0.0))
    .insert(RigidBody::Dynamic)
    .insert(Trap3)
    .insert(Trap)
    .insert(Velocity::zero())
    .insert(Collider::cuboid(72.0, 16.0))
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
    ).insert(NeedReload);
}

fn do_trap1(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    kid_query: Query<&Kid>,
    trig1_query: Query<&Trig1>,
    trap1_query: Query<Entity,With<Trap1>>,
){
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity_a, entity_b, _) => {
                let is_entity1_b = kid_query.get(*entity_b).is_ok();
                let is_entity2_a = trig1_query.get(*entity_a).is_ok();
                let is_entity1_a = kid_query.get(*entity_a).is_ok();
                let is_entity2_b = trig1_query.get(*entity_b).is_ok();
                let Ok(trap1) = trap1_query.get_single()
                else{
                    continue;
                };
                if is_entity1_b && is_entity2_a || is_entity1_a && is_entity2_b{
                    commands.entity(trap1).despawn_recursive();
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
                let is_entity2_a = trig2_query.get(*entity_a).is_ok();
                let is_entity1_a = kid_query.get(*entity_a).is_ok();
                let is_entity2_b = trig2_query.get(*entity_b).is_ok();
                if is_entity1_b && is_entity2_a{
                    spawn_single_hidden(&mut commands, &ori_image, &ori_atlas, 3., -8., BASEX, BASEY);
                    spawn_single_hidden(&mut commands, &ori_image, &ori_atlas, 3., -7., BASEX, BASEY);
                }else if is_entity1_a && is_entity2_b{
                    spawn_single_hidden(&mut commands, &ori_image, &ori_atlas, 3., -8., BASEX, BASEY);
                    spawn_single_hidden(&mut commands, &ori_image, &ori_atlas, 3., -7., BASEX, BASEY);
                }
            }
            _ => {}
        }
    }
}

fn do_trap3(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    kid_query: Query<&Kid>,
    trig3_query: Query<&Trig3>,
    mut trap2_query: Query<Entity,With<Trap2>>,
    trap3_query: Query<Entity,With<Trap3>>,
    leaf_num: Res<LeafNum>,
){
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity_a, entity_b, _) => {
                let is_entity1_b = kid_query.get(*entity_b).is_ok();
                let is_entity2_a = trig3_query.get(*entity_a).is_ok();
                let is_entity1_a = kid_query.get(*entity_a).is_ok();
                let is_entity2_b = trig3_query.get(*entity_b).is_ok();
                let Ok(trap3) = trap3_query.get_single()
                else{
                    continue;
                };
                if (is_entity1_b && is_entity2_a) || (is_entity1_a && is_entity2_b){
                    if leaf_num.num == 1{
                        commands.entity(trap3).insert(Move{
                            goal_pos: Vec2::new(BASEX+278., BASEY-64.),
                            linear_speed: 1000.,
                            goal_angle: 0.,
                            angle_speed: 0.,
                            status: 0,
                        });
                    }else if leaf_num.num == 2{
                        for item in trap2_query.iter_mut(){
                            commands.entity(item).despawn_recursive();
                        }
                    }
                }
            }
            _ => {}
        }
    }
}