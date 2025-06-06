use bevy_rapier2d::prelude::*;
use bevy::prelude::*;

use bevy::sprite::Sprite;
use crate::asset_loader::{BackGroundAssets, ImageAssets, SceneAssets};
use crate::base::ground::spawn_single_box;
use crate::base::hidden::spawn_single_hidden;
use crate::base::kid::Kid;
use crate::base::savepointer::spawn_single_savepointer;
use crate::base::wrap::spawn_single_warp;
use crate::state::GameState;


const BASEX: f32 = 800.0*2.;
const BASEY: f32 = 608.0*2.;


#[derive(Resource, Debug, Default)]
pub struct BuildingState{
    pub num: u8,
    pub choose: u8,
}

#[derive(Component, Debug)]
pub struct ClearBuilding;

#[derive(Component, Debug)]
pub struct ChangeBuilding{
    pub delta: u8,
    pub to: u8,
}

#[derive(Component, Debug)]
pub struct Trap1;

pub struct CenterPlugin;

impl Plugin for CenterPlugin{
    fn build(&self,app: &mut App){
        app.init_resource::<BuildingState>()
        .add_systems(PostStartup,spawn_once)
        .add_systems(OnExit(GameState::Reload),spawn_reload)
        .add_systems(Update,(do_clear,do_change));
    }
}


fn spawn_once(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    scene_assets: Res<SceneAssets>,
    bg_assets: Res<BackGroundAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>
){
    commands.spawn(
        Sprite{
            image: bg_assets.center.clone(),
            ..Default::default()
        }
    ).insert(
        Transform{
            translation: Vec3::new(BASEX, BASEY, -0.5),
            scale: Vec3::new(1.33, 1.33, 1.0),
            ..Default::default()
        }
    );
    commands.spawn(
        Sprite{
            image: scene_assets.building_center.clone(),
            ..Default::default()
        }
    ).insert(
        Transform::from_xyz(BASEX, BASEY, -0.3)
    );

    spawn_single_box(&mut commands,0.,-9.,BASEX,BASEY,12.5,0.5);
    spawn_single_box(&mut commands,0.,9.,BASEX,BASEY,12.5,0.5);
    spawn_single_box(&mut commands,-12.,0.,BASEX,BASEY,0.5,8.5);
    spawn_single_box(&mut commands,12.,0.,BASEX,BASEY,0.5,8.5);
    spawn_single_box(&mut commands,-9.,4.,BASEX,BASEY,2.5,0.5);
    spawn_single_box(&mut commands,9.,4.,BASEX,BASEY,2.5,0.5);
    spawn_single_box(&mut commands,-5.5,2.,BASEX,BASEY,1.,0.5);
    spawn_single_box(&mut commands,5.5,2.,BASEX,BASEY,1.,0.5);
    spawn_single_box(&mut commands,0.,1.,BASEX,BASEY,1.5,0.5);
    spawn_single_box(&mut commands,0.,-1.,BASEX,BASEY,4.5,0.5);
    spawn_single_box(&mut commands,0.,-2.,BASEX,BASEY,5.5,0.5);
    spawn_single_box(&mut commands,0.,-3.5,BASEX,BASEY,8.5,1.);
    spawn_single_box(&mut commands,-9.,-3.,BASEX,BASEY,0.5,1.5);
    spawn_single_box(&mut commands,9.,-3.,BASEX,BASEY,0.5,1.5);
    spawn_single_box(&mut commands,-7.5,-5.,BASEX,BASEY,1.,0.5);
    spawn_single_box(&mut commands,7.5,-5.,BASEX,BASEY,1.,0.5);
    spawn_single_box(&mut commands,0.,-6.,BASEX,BASEY,3.5,1.5);
    spawn_single_box(&mut commands,0.,-8.,BASEX,BASEY,4.5,0.5);


    let sv_layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 2, 1, None, None);
    let sv_atlas_layout = texture_atlases.add(sv_layout);
    let sv_atlas = TextureAtlas{
        layout : sv_atlas_layout,
        index : 0,
    };
    let sv_image = image_assets.save.clone();
    spawn_single_savepointer(&mut commands,&sv_image,&sv_atlas,0.,2.,BASEX,BASEY,9);

    let wr_layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 4, 1, None, None);
    let wr_atlas_layout = texture_atlases.add(wr_layout);
    let wr_atlas = TextureAtlas{
        layout : wr_atlas_layout,
        index : 0,
    };
    let wr_image = image_assets.warp.clone();
    let wrb = spawn_single_warp(&mut commands,&wr_image,&wr_atlas,BASEX-288.,BASEY+160.,BASEX-2400.,BASEY+608.);
    commands.entity(wrb).insert(
        ChangeBuilding{
            delta: 0,
            to: 2,
        }
    );
    let wra = spawn_single_warp(&mut commands,&wr_image,&wr_atlas,BASEX+288.,BASEY+160.,BASEX-3200.,BASEY);
    commands.entity(wra).insert(
        ChangeBuilding{
            delta: 0,
            to: 1,
        }
    );
    let wrd = spawn_single_warp(&mut commands,&wr_image,&wr_atlas,BASEX+160.,BASEY-256.,BASEX-800.,BASEY+608.);
    commands.entity(wrd).insert(
        ChangeBuilding{
            delta: 0,
            to: 4,
        }
    );
    let wrc = spawn_single_warp(&mut commands,&wr_image,&wr_atlas,BASEX-160.,BASEY-256.,BASEX-1600.,BASEY+1216.);
    commands.entity(wrc).insert(
        ChangeBuilding{
            delta: 0,
            to: 3,
        }
    );
    
    spawn_single_warp(&mut commands,&wr_image,&wr_atlas,BASEX,BASEY,BASEX-256.,BASEY-1216.-256.);

}


fn spawn_reload(
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
    mut state: ResMut<BuildingState>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>
){
    state.choose = 0;

    let ori_image = scene_assets.bg.clone();
    let ori_layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 6, 9, None, None);
    let ori_atlas_layout = texture_atlases.add(ori_layout);
    let ori_atlas = TextureAtlas{
        layout : ori_atlas_layout.clone(),
        index : 53,
    };

    if state.num != 15{
        let tmp1 = spawn_single_hidden(&mut commands,&ori_image,&ori_atlas,-1.,0.,BASEX,BASEY);
        commands.entity(tmp1).insert(Trap1);
        let tmp2 = spawn_single_hidden(&mut commands,&ori_image,&ori_atlas,1.,0.,BASEX,BASEY);
        commands.entity(tmp2).insert(Trap1);
    }

}


fn do_clear(
    mut collision_events: EventReader<CollisionEvent>,
    kid_query: Query<&Kid>,
    clear_query: Query<&ClearBuilding>,
    mut state: ResMut<BuildingState>,
){
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity_a, entity_b, _) => {
                let is_entity1_b = kid_query.get(*entity_b).is_ok();
                let is_entity2_a = clear_query.get(*entity_a).is_ok();
                let is_entity1_a = kid_query.get(*entity_a).is_ok();
                let is_entity2_b = clear_query.get(*entity_b).is_ok();
                if is_entity1_b && is_entity2_a || is_entity1_a && is_entity2_b{
                    state.num = 0;
                }
            }
            _ => {}
        }
    }
}

fn do_change(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    kid_query: Query<&Kid>,
    change_query: Query<&ChangeBuilding,Without<Trap1>>,
    mut trap_query: Query<Entity,(With<Trap1>,Without<ChangeBuilding>)>,
    mut state: ResMut<BuildingState>,
    mut next_state: ResMut<NextState<GameState>>,
){
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity_a, entity_b, _) => {
                let is_entity1_b = kid_query.get(*entity_b).is_ok();
                let is_entity2_a = change_query.get(*entity_a).is_ok();
                let is_entity1_a = kid_query.get(*entity_a).is_ok();
                let is_entity2_b = change_query.get(*entity_b).is_ok();
                if is_entity1_b && is_entity2_a{
                    state.choose = change_query.get(*entity_a).unwrap().to;
                    state.num |= change_query.get(*entity_a).unwrap().delta;
                    if state.num == 15{
                        for item in trap_query.iter_mut(){
                            commands.entity(item).despawn_recursive();
                        }
                    }
                    next_state.set(GameState::ReForBuilding);
                }else if is_entity1_a && is_entity2_b{
                    state.choose = change_query.get(*entity_b).unwrap().to;
                    state.num |= change_query.get(*entity_b).unwrap().delta;
                    if state.num == 15{
                        for item in trap_query.iter_mut(){
                            commands.entity(item).despawn_recursive();
                        }
                    }
                    next_state.set(GameState::ReForBuilding);
                }
            }
            _ => {}
        }
    }
}



