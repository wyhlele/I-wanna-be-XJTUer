use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use bevy::sprite::Sprite;
use crate::asset_loader::{AchievementAssets, BackGroundAssets, ImageAssets, SceneAssets};
use crate::base::ground::spawn_single_box;
use crate::base::kid::{spawn_single_kid, Kid};
use crate::base::savepointer::spawn_single_savepointer;
use crate::base::spike::spawn_single_spike_fixed;
use crate::base::wrap::spawn_single_warp;
use crate::boss::boss::BossStart;
use crate::kid_saver::KidSaver;
use crate::menu::achievement::Achievement;
use crate::schedule::InGameSet;
use crate::state::BGMReload;

const BASEX: f32 = 800.0*2.;
const BASEY: f32 = 0.0;

pub struct BuildingEPlugin;

impl Plugin for BuildingEPlugin{
    fn build(&self,app: &mut App){
        app.add_systems(PostStartup,spawn_once)
        .add_systems(Update,do_trans.in_set(InGameSet::CalcAutoMove));
    }
}

#[derive(Component, Debug, Default)]
pub struct Top;

fn spawn_once(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    scene_assets: Res<SceneAssets>,
    bg_assets: Res<BackGroundAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>
){
    commands.spawn(
        Sprite{
            image: bg_assets.inside_e.clone(),
            ..Default::default()
        }
    ).insert(
        Transform::from_xyz(BASEX, BASEY, -0.5)
    );
    commands.spawn(
        Sprite{
            image: scene_assets.building_e.clone(),
            ..Default::default()
        }
    ).insert(
        Transform::from_xyz(BASEX, BASEY, -0.3)
    );

    spawn_single_box(&mut commands,0.,-9.,BASEX,BASEY,12.5,0.5);
    spawn_single_box(&mut commands,-12.,-0.5,BASEX,BASEY,0.5,8.);
    spawn_single_box(&mut commands,-6.5,8.5,BASEX,BASEY,6.,1.);
    spawn_single_box(&mut commands,6.5,8.5,BASEX,BASEY,6.,1.);
    spawn_single_box(&mut commands,11.5,6.5,BASEX,BASEY,1.,1.);
    spawn_single_box(&mut commands,11.5,-2.,BASEX,BASEY,1.,6.5);
    spawn_single_box(&mut commands,-1.,-7.,BASEX,BASEY,1.5,1.5);
    spawn_single_box(&mut commands,-1.5,-3.,BASEX,BASEY,1.,0.5);
    spawn_single_box(&mut commands,0.,0.,BASEX,BASEY,0.5,5.5);
    spawn_single_box(&mut commands,2.5,5.5,BASEX,BASEY,1.,2.);
    spawn_single_box(&mut commands,6.5,2.5,BASEX,BASEY,1.,1.);
    spawn_single_box(&mut commands,5.,-2.,BASEX,BASEY,2.5,3.5);
    spawn_single_box(&mut commands,-10.5,-4.,BASEX,BASEY,1.,0.5);
    spawn_single_box(&mut commands,-6.,-4.5,BASEX,BASEY,0.5,1.);
    spawn_single_box(&mut commands,-7.,-1.,BASEX,BASEY,1.5,0.5);
    spawn_single_box(&mut commands,-6.,0.,BASEX,BASEY,1.5,0.5);
    spawn_single_box(&mut commands,-6.,4.,BASEX,BASEY,2.5,0.5);
    spawn_single_box(&mut commands,8.,-1.,BASEX,BASEY,0.5,0.5);
    spawn_single_box(&mut commands,9.,-5.,BASEX,BASEY,0.5,0.5);

    let sv_layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 2, 1, None, None);
    let sv_atlas_layout = texture_atlases.add(sv_layout);
    let sv_atlas = TextureAtlas{
        layout : sv_atlas_layout,
        index : 0,
    };
    let sv_image = image_assets.save.clone();
    spawn_single_savepointer(&mut commands,&sv_image,&sv_atlas,-8.,-8.,BASEX,BASEY,10);
    spawn_single_savepointer(&mut commands,&sv_image,&sv_atlas,3.,2.,BASEX,BASEY,11);
    spawn_single_savepointer(&mut commands,&sv_image,&sv_atlas,11.,5.,BASEX,BASEY,12);



    spawn_single_spike_fixed(&mut commands, &image_assets.spike, -6., -6., BASEX,BASEY,180.0);
    spawn_single_spike_fixed(&mut commands, &image_assets.spike, -5., -8., BASEX,BASEY,0.0);
    spawn_single_spike_fixed(&mut commands, &image_assets.spike, -4., -8., BASEX,BASEY,0.0);
    spawn_single_spike_fixed(&mut commands, &image_assets.spike, -5., 5., BASEX,BASEY,0.0);
    spawn_single_spike_fixed(&mut commands, &image_assets.spike, -6., 5., BASEX,BASEY,0.0);
    spawn_single_spike_fixed(&mut commands, &image_assets.spike, -5., -4., BASEX,BASEY,-90.0);
    spawn_single_spike_fixed(&mut commands, &image_assets.spike, -3., -3., BASEX,BASEY,90.0);
    spawn_single_spike_fixed(&mut commands, &image_assets.spike, -2., -2., BASEX,BASEY,0.0);
    spawn_single_spike_fixed(&mut commands, &image_assets.spike, -1., -2., BASEX,BASEY,0.0);
    spawn_single_spike_fixed(&mut commands, &image_assets.spike, -4., 0., BASEX,BASEY,-90.0);
    spawn_single_spike_fixed(&mut commands, &image_assets.spike, -3., 4., BASEX,BASEY,-90.0);
    spawn_single_spike_fixed(&mut commands, &image_assets.spike, -7., 7., BASEX,BASEY,180.0);
    spawn_single_spike_fixed(&mut commands, &image_assets.spike, -4., 7., BASEX,BASEY,180.0);
    spawn_single_spike_fixed(&mut commands, &image_assets.spike, 1., 1., BASEX,BASEY,-90.0);
    spawn_single_spike_fixed(&mut commands, &image_assets.spike, 4., 4., BASEX,BASEY,-90.0);
    spawn_single_spike_fixed(&mut commands, &image_assets.spike, 5., 3., BASEX,BASEY,90.0);
    spawn_single_spike_fixed(&mut commands, &image_assets.spike, 6.5, 4., BASEX,BASEY,0.0);
    spawn_single_spike_fixed(&mut commands, &image_assets.spike, 2., -5., BASEX,BASEY,90.0);
    spawn_single_spike_fixed(&mut commands, &image_assets.spike, 5., -8., BASEX,BASEY,0.0);
    spawn_single_spike_fixed(&mut commands, &image_assets.spike, 4., -6., BASEX,BASEY,180.0);
    spawn_single_spike_fixed(&mut commands, &image_assets.spike, 6., -6., BASEX,BASEY,180.0);
    spawn_single_spike_fixed(&mut commands, &image_assets.spike, 10., -7., BASEX,BASEY,90.0);
    spawn_single_spike_fixed(&mut commands, &image_assets.spike, 8., -5., BASEX,BASEY,-90.0);
    spawn_single_spike_fixed(&mut commands, &image_assets.spike, 10., -3., BASEX,BASEY,90.0);
    spawn_single_spike_fixed(&mut commands, &image_assets.spike, 10., 2., BASEX,BASEY,90.0);
    spawn_single_spike_fixed(&mut commands, &image_assets.spike, 8., 3., BASEX,BASEY,-90.0);
    spawn_single_spike_fixed(&mut commands, &image_assets.spike, 9., -1., BASEX,BASEY,-90.0);
    

    let wr_layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 4, 1, None, None);
    let wr_atlas_layout = texture_atlases.add(wr_layout);
    let wr_atlas = TextureAtlas{
        layout : wr_atlas_layout,
        index : 0,
    };
    let wr_image = image_assets.warp.clone();

    let warp = spawn_single_warp(&mut commands,&wr_image,&wr_atlas,BASEX+384.,BASEY+160.,-2400.,608.);
    commands.entity(warp).insert(BGMReload{id:13}).insert(BossStart);


    commands.spawn(
        RigidBody::Fixed
    ).insert(
        GravityScale(0.0)
    ).insert(
        Transform::from_xyz(BASEX,BASEY + 320.,0.0)
    ).insert(
        Collider::cuboid(16., 16.)
    ).insert(
        LockedAxes::ROTATION_LOCKED
    ).insert(
        CollisionGroups::new(
            Group::GROUP_2,
            Group::GROUP_1|Group::GROUP_4,
        )
    ).insert(SolverGroups::new(
        Group::GROUP_2,
        Group::GROUP_1,
    )).insert(Top);
}

fn do_trans(
    mut collision_events: EventReader<CollisionEvent>,
    kid_query: Query<&Kid>,
    top_query: Query<&Top>,
    achievement_assets: Res<AchievementAssets>,
    kid_saver: Res<KidSaver>,
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
){
    let layout = TextureAtlasLayout::from_grid(UVec2::new(25, 24), 4, 6, None, None);
    let atlas_layout = texture_atlases.add(layout);
    let atlas = TextureAtlas{
        layout : atlas_layout,
        index : 0,
    };
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity_a, entity_b, _) => {
                let is_entity1_b = kid_query.get(*entity_b).is_ok();
                let is_entity2_a = top_query.get(*entity_a).is_ok();
                let is_entity1_a = kid_query.get(*entity_a).is_ok();
                let is_entity2_b = top_query.get(*entity_b).is_ok();

                if is_entity1_b && is_entity2_a{
                    commands.entity(*entity_b).despawn_recursive();
                    spawn_single_kid(&mut commands,&image_assets.kid,&atlas,2400.-96.,608.+128.);
                    if (kid_saver.achi>>6)&1==0{
                        commands.spawn(Achievement{time: 72, id: 6})
                        .insert(Sprite{
                            image: achievement_assets.achievement6.clone(),
                            ..Default::default()
                        }).insert(Transform::from_xyz(0., 0., -5.0));
                    }
                }else if is_entity1_a && is_entity2_b{
                    commands.entity(*entity_a).despawn_recursive();
                    spawn_single_kid(&mut commands,&image_assets.kid,&atlas,2400.-96.,608.+128.);
                    if (kid_saver.achi>>6)&1==0{
                        commands.spawn(Achievement{time: 72, id: 6})
                        .insert(Sprite{
                            image: achievement_assets.achievement6.clone(),
                            ..Default::default()
                        }).insert(Transform::from_xyz(0., 0., -5.0));
                    }
                }
                
            }
            _ => {}
        }
    }
}