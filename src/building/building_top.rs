use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy::sprite::Sprite;

use crate::asset_loader::{AchievementAssets, BackGroundAssets, ImageAssets};
use crate::base::ground::spawn_single_box;
use crate::base::kid::Kid;
use crate::base::trap::Trap;
use crate::base::wrap::spawn_single_warp;
use crate::kid_saver::KidSaver;
use crate::menu::achievement::Achievement;

const BASEX: f32 = 800.0*3.;
const BASEY: f32 = 608.0;

#[derive(Component, Debug)]
pub struct Traptop;

pub struct BuildingTopPlugin;

impl Plugin for BuildingTopPlugin{
    fn build(&self,app: &mut App){
        app.add_systems(PostStartup,spawn_once)
        .add_systems(Update, do_traptop);
    }
}


fn spawn_once(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    bg_assets: Res<BackGroundAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>
){
    commands.spawn(
        Sprite{
            image: bg_assets.top.clone(),
            ..Default::default()
        }
    ).insert(
        Transform::from_xyz(BASEX, BASEY, -0.5)
    );
    

    // spawn_single_box(&mut commands,0.,-10.,BASEX,BASEY,12.5,0.5);
    spawn_single_box(&mut commands,0.,10.,BASEX,BASEY,12.5,0.5);
    spawn_single_box(&mut commands,-13.,0.,BASEX,BASEY,0.5,9.5);
    spawn_single_box(&mut commands,13.,0.,BASEX,BASEY,0.5,9.5);
    spawn_single_box(&mut commands,-4.5,-3.,BASEX,BASEY,3.5,6.5);


    commands.spawn(
        RigidBody::Fixed
    ).insert(
        GravityScale(0.0)
    ).insert(
        Transform::from_xyz(BASEX,BASEY - 320.,0.0)
    ).insert(
        Collider::cuboid(32.*12.5, 32.*0.5)
    ).insert(
        LockedAxes::ROTATION_LOCKED
    ).insert(
        CollisionGroups::new(
            Group::GROUP_2,
            Group::GROUP_1,
        )
    ).insert(SolverGroups::new(
        Group::GROUP_2,
        Group::GROUP_1,
    )).insert(Trap)
    .insert(Traptop);

    let wr_layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 4, 1, None, None);
    let wr_atlas_layout = texture_atlases.add(wr_layout);
    let wr_atlas = TextureAtlas{
        layout : wr_atlas_layout,
        index : 0,
    };
    let wr_image = image_assets.warp.clone();
    spawn_single_warp(&mut commands,&wr_image,&wr_atlas,BASEX-192.,BASEY+128.,1600.,192.);
}


fn do_traptop(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    kid_query: Query<&Kid>,
    traptop_query: Query<&Traptop>,
    achievement_assets: Res<AchievementAssets>,
    kid_saver: Res<KidSaver>,
){
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity_a, entity_b, _) => {
                let is_entity1_b = kid_query.get(*entity_b).is_ok();
                let is_entity2_a = traptop_query.get(*entity_a).is_ok();
                let is_entity1_a = kid_query.get(*entity_a).is_ok();
                let is_entity2_b = traptop_query.get(*entity_b).is_ok();
                if is_entity1_b && is_entity2_a || is_entity1_a && is_entity2_b{
                    if (kid_saver.achi>>7)&1==0{
                        commands.spawn(Achievement{time: 149, id: 7})
                        .insert(Sprite{
                            image: achievement_assets.achievement7.clone(),
                            ..Default::default()
                        }).insert(Transform::from_xyz(0., 0., -5.0));
                    }
                }
            }
            _ => {}
        }
    }
}