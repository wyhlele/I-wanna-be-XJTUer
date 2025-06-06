use bevy::math::VectorSpace;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use bevy::sprite::Sprite;
use crate::asset_loader::{BackGroundAssets, ImageAssets, SceneAssets};
use crate::base::ground::spawn_single_box;
use crate::base::kid::Kid;
use crate::base::moveto::Move;
use crate::base::trap::Trap;
use crate::schedule::InGameSet;
use crate::state::{BGMReload, GameState, NeedReload};

const BASEX: f32 = -800.0*3.;
const BASEY: f32 = 608.0;
const EPSILON: f32 = 1.0;

pub struct BossPlugin;

impl Plugin for BossPlugin{
    fn build(&self,app: &mut App){
        app.add_systems(PostStartup,spawn_once)
        .add_systems(OnExit(GameState::Reload),spawn_reload)
        .add_systems(Update, (update_boss, do_start).chain().in_set(InGameSet::EntityUpdates));
    }
}

#[derive(Resource)]
struct BossTimer(Timer);

#[derive(Component, Debug, Default)]
pub struct BossStart;

#[derive(Component, Debug, Default)]
pub struct Boss{
    pub state: usize,
    pub countdown: i16,
    pub able: bool,
}

#[derive(Component, Debug, Default)]
pub struct Blood;

fn spawn_once(
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
    bg_assets: Res<BackGroundAssets>,
){
    commands.spawn(
        Sprite{
            image: bg_assets.library.clone(),
            ..Default::default()
        }
    ).insert(
        Transform::from_xyz(BASEX, BASEY, -0.5)
    );
    commands.spawn(
        Sprite{
            image: scene_assets.boss.clone(),
            ..Default::default()
        }
    ).insert(
        Transform::from_xyz(BASEX, BASEY, -0.3)
    );

    spawn_single_box(&mut commands,0.,-9.,BASEX,BASEY,12.5,0.5);
    spawn_single_box(&mut commands,0.,9.,BASEX,BASEY,12.5,0.5);
    spawn_single_box(&mut commands,-12.,0.,BASEX,BASEY,0.5,8.5);
    spawn_single_box(&mut commands,12.,0.,BASEX,BASEY,0.5,8.5);
    spawn_single_box(&mut commands,0.,-1.,BASEX,BASEY,3.5,0.5);
    spawn_single_box(&mut commands,0.,4.,BASEX,BASEY,2.5,0.5);
    spawn_single_box(&mut commands,0.,-6.,BASEX,BASEY,2.5,0.5);
    spawn_single_box(&mut commands,-7.5,-4.,BASEX,BASEY,2.,0.5);
    spawn_single_box(&mut commands,7.5,-4.,BASEX,BASEY,2.,0.5);
    spawn_single_box(&mut commands,-6.5,1.,BASEX,BASEY,1.,0.5);
    spawn_single_box(&mut commands,6.5,1.,BASEX,BASEY,1.,0.5);
    spawn_single_box(&mut commands,-8.,2.,BASEX,BASEY,1.5,0.5);
    spawn_single_box(&mut commands,8.,2.,BASEX,BASEY,1.5,0.5);

    commands.insert_resource(BossTimer(Timer::from_seconds(0.01, TimerMode::Repeating)));
}

fn spawn_reload(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    scene_assets: Res<SceneAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>
){
    let points = [
    Vec2::new(0., 128.),
    Vec2::new(80., 100.),
    Vec2::new(160., 40.),
    Vec2::new(210., -70.),
    Vec2::new(160., -145.),
    Vec2::new(-160., -145.),
    Vec2::new(-210., -70.),
    Vec2::new(-160., 40.),
    Vec2::new(-80., 100.)];

    let boss_layout = TextureAtlasLayout::from_grid(UVec2::new(432, 300), 1, 2, None, None);
    let boss_atlas_layout = texture_atlases.add(boss_layout);
    let boss_atlas = TextureAtlas{
        layout : boss_atlas_layout,
        index : 0,
    };

    if let Some(bd) = Collider::convex_hull(&points){
        commands.spawn(
            Sprite{
                image: image_assets.ferris.clone(),
                texture_atlas: Some(boss_atlas),
                ..Default::default()
            }
        ).insert(
            Transform{
                translation: Vec3::new(BASEX, BASEY+208., 0.1),
                scale: Vec3::new(0.5, 0.5, 1.0),
                ..Default::default()
            }
        ).insert(Boss{
            state: 10,
            countdown: 0,
            able: false,
        }).insert(NeedReload)
        .insert(Trap)
        .insert(
            Velocity::zero()
        ).insert(
            RigidBody::Dynamic
        ).insert(
            GravityScale(0.0)
        ).insert(
            bd,
        ).insert(
            ColliderMassProperties::Mass(10000.0)
        ).insert(
            CollisionGroups::new(
                Group::GROUP_3,
                Group::GROUP_1|Group::GROUP_4,
            )
        ).insert(SolverGroups::new(
            Group::GROUP_3,
            Group::GROUP_1,
            )
        ).insert(Move{
            goal_pos: Vec2::new(BASEX, BASEY+208.),
            linear_speed: 300.0,
            ..Default::default()
        });
    }

    let bld_layout = TextureAtlasLayout::from_grid(UVec2::new(672, 8), 1, 11, None, None);
    let bld_atlas_layout = texture_atlases.add(bld_layout);
    let bld_atlas = TextureAtlas{
        layout : bld_atlas_layout,
        index : 10,
    };

    commands.spawn(
        Sprite{
            image: scene_assets.blood.clone(),
            texture_atlas: Some(bld_atlas),
            ..Default::default()
        }
    ).insert(
        Transform::from_xyz(BASEX, BASEY+256., 2.)
    ).insert(Blood)
    .insert(NeedReload);



}

fn update_boss(
    mut query: Query<(&mut Boss, &mut Transform,&mut Velocity, &mut Move, &mut Sprite),With<Boss>>,
    time: Res<Time>,
    mut timer: ResMut<BossTimer>,
){
    timer.0.tick(time.delta());
    if timer.0.finished() {
        let Ok((mut boss, trans, velocity, mut moveto, mut sprite)) = query.get_single_mut()
        else{
            return;
        };
        if boss.able == false{
            return;
        }
        if boss.countdown>0{
            if let Some(atlas) = &mut sprite.texture_atlas{
                atlas.index = 1;
            }
            if boss.countdown==200{
                moveto.linear_speed *=1.07;
            }
            boss.countdown -= 1;
        }else{
            if let Some(atlas) = &mut sprite.texture_atlas{
                atlas.index = 0;
            }
        }
        if (velocity.linvel - Vec2::ZERO).length() <= EPSILON{
            if trans.translation.x > BASEX && trans.translation.y > BASEY{
                moveto.goal_pos = Vec2::new(BASEX+320., BASEY-208.);
            }else if trans.translation.x > BASEX && trans.translation.y < BASEY{
                moveto.goal_pos = Vec2::new(BASEX-320., BASEY-208.);
            }else if trans.translation.x < BASEX && trans.translation.y < BASEY{
                moveto.goal_pos = Vec2::new(BASEX-320., BASEY+208.);
            }else{
                moveto.goal_pos = Vec2::new(BASEX+320., BASEY+208.);
            }
        }
        
    }
}

fn do_start(
    mut collision_events: EventReader<CollisionEvent>,
    mut boss_query: Query<&mut Boss,With<Boss>>,
    kid_query: Query<&Kid>,
    start_query: Query<&BossStart>,
){
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity_a, entity_b, _) => {
                let is_entity1_b = kid_query.get(*entity_b).is_ok();
                let is_entity2_a = start_query.get(*entity_a).is_ok();
                let is_entity1_a = kid_query.get(*entity_a).is_ok();
                let is_entity2_b = start_query.get(*entity_b).is_ok();

                if is_entity1_b && is_entity2_a{
                    for mut boss in boss_query.iter_mut(){
                        boss.able = true;
                    }
                }else if is_entity1_a && is_entity2_b{
                    for mut boss in boss_query.iter_mut(){
                        boss.able = true;
                    }
                }
                
            }
            _ => {}
        }
    }
}