use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use bevy::sprite::Sprite;
use rand::Rng;
use crate::asset_loader::{BackGroundAssets, ImageAssets, MusicAssets, SceneAssets};
use crate::base::apple::spawn_single_apple;
use crate::base::ground::spawn_single_box;
use crate::base::kid::Kid;
use crate::base::moveto::Move;
use crate::base::trap::Trap;
use crate::schedule::InGameSet;
use crate::state::{GameState, NeedReload};

const BASEX: f32 = -800.0*3.;
const BASEY: f32 = 608.0;
const EPSILON: f32 = 1.0;

pub struct BossPlugin;

impl Plugin for BossPlugin{
    fn build(&self,app: &mut App){
        app.add_systems(PostStartup,spawn_once)
        .add_systems(OnExit(GameState::Reload),spawn_reload)
        .add_systems(Update, (update_boss, do_attack, do_start).chain().in_set(InGameSet::EntityUpdates))
        .add_systems(Update, (do_split, do_beam).chain().in_set(InGameSet::EntityUpdates));
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
    pub weapon: i8,
    pub attack: i16,
    pub pos: f32,
    pub dir: f32,
}

#[derive(Component, Debug, Default)]
pub struct Blood;

#[derive(Component, Debug, Default)]
pub struct Beam{
    time: i16,
    speed: f32,
    direction: Vec2,
}

#[derive(Resource)]
struct BeamTimer(Timer);

#[derive(Component, Debug, Default)]
pub struct Split{
    time: i16,
    size: f32,
}

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
    commands.insert_resource(BeamTimer(Timer::from_seconds(0.01, TimerMode::Repeating)));

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
    Vec2::new(-80., 100.),
    Vec2::new(0., 128.),];

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
            weapon: -1,
            attack: 100,
            pos: BASEX,
            dir: 0.,
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
        Transform::from_xyz(BASEX, BASEY+256., 1.1)
    ).insert(Blood)
    .insert(NeedReload);



}

fn update_boss(
    mut commands: Commands,
    mut query: Query<(&mut Boss, &mut Transform,&mut Velocity, &mut Move, &mut Sprite),(With<Boss>,Without<Kid>)>,
    kid_query: Query<&Transform,With<Kid>>,
    image_assets: Res<ImageAssets>,
    time: Res<Time>,
    mut timer: ResMut<BossTimer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>
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
        let apple_layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 2,1, None, None);
        let apple_atlas_layout = texture_atlases.add(apple_layout);
        let apple_atlas = TextureAtlas{
            layout : apple_atlas_layout,
            index : 0,
        };
        let apple_image = image_assets.apple.clone();


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
                if atlas.index !=0{
                    atlas.index = 0;
                    let Ok(transform) = kid_query.get_single()
                    else{
                        return ;
                    } ;
                    let vel = (transform.translation - trans.translation).normalize();
                    let app = spawn_single_apple(&mut commands, &apple_image, &apple_atlas, 
                        trans.translation.x, trans.translation.y,
                        1.5, Vec2::new(vel.x, vel.y) * moveto.linear_speed * 0.7,
                    );
                    commands.entity(app).insert(Split{time:1,size:15.}).insert(NeedReload);
                }
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

fn do_attack(
    mut commands: Commands,
    mut query: Query<(&mut Boss, &mut Transform,&mut Move),(With<Boss>,Without<Kid>)>,
    kid_query: Query<&Transform,With<Kid>>,
    image_assets: Res<ImageAssets>,
    music_assets: Res<MusicAssets>,
    time: Res<Time>,
    mut timer: ResMut<BossTimer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>
){
    timer.0.tick(time.delta());
    if timer.0.finished() {
        let Ok((mut boss, trans,  moveto)) = query.get_single_mut()
        else{
            return;
        };
        if boss.able == false{
            return;
        }
        let apple_layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 2,1, None, None);
        let apple_atlas_layout = texture_atlases.add(apple_layout);
        let apple_atlas = TextureAtlas{
            layout : apple_atlas_layout,
            index : 0,
        };
        let apple_image = image_assets.apple.clone();

        if boss.attack>0{
            boss.attack -= 1;
            let time = ((boss.attack as f32) * (moveto.linear_speed/300.)) as i16;
            let Ok(transform) = kid_query.get_single()
            else{
                return ;
            } ;
            if boss.weapon==0{
                if time % 100 == 0{
                    let vel = (transform.translation - trans.translation).normalize();
                    let app = spawn_single_apple(&mut commands, &apple_image, &apple_atlas, 
                        trans.translation.x, trans.translation.y,
                        1.0, Vec2::new(vel.x, vel.y) * moveto.linear_speed * 0.7,
                    );
                    commands.entity(app).insert(Split{time:0,size:10.}).insert(NeedReload);
                }
            }else if boss.weapon==1{
                if time % 200 == 0{
                    let vel = (transform.translation - trans.translation).normalize();
                    let mut ori = Vec2::new(vel.x, vel.y);
                    let angle_step = PI / 8.0;
                    for _ in 0..16 {
                        let app = spawn_single_apple(&mut commands, &apple_image, &apple_atlas, 
                            trans.translation.x, trans.translation.y,
                            1.0,  ori.clone()* moveto.linear_speed * 1.2,
                        );
                        commands.entity(app).insert(Split{time:0,size:10.}).insert(NeedReload);
                        ori = Vec2::new(ori.x*angle_step.cos() - ori.y*angle_step.sin(),
                            ori.x*angle_step.sin() + ori.y*angle_step.cos());
                    }
                }
            }else if boss.weapon>=2{
                if time >= 300{
                    commands.spawn(
                        Sprite{
                            image: image_assets.beam.clone(),
                            ..Default::default()
                        }
                    ).insert(Beam{
                        time: 150 / ((moveto.linear_speed/300.) as i16),
                        speed: moveto.linear_speed * 5.0,
                        direction: Vec2::new(-boss.dir, 0.)
                    }).insert(
                        Transform{
                            translation: Vec3::new(BASEX+boss.dir*384., boss.pos, 1.2),
                            scale: Vec3::new(1., 1., 1.),
                            ..Default::default()
                        }
                    ).insert(
                        Velocity::zero()
                    ).insert(
                        RigidBody::Dynamic
                    ).insert(
                        GravityScale(0.0)
                    ).insert(
                        Collider::cuboid(16., 16.)
                    ).insert(
                        CollisionGroups::new(
                            Group::GROUP_3,
                            Group::GROUP_1,
                        )
                    ).insert(SolverGroups::new(
                        Group::GROUP_3,
                        Group::GROUP_1,
                        )
                    ).insert(Trap)
                    .insert(NeedReload);
                }
            }
        }else{
            boss.attack = (400./(moveto.linear_speed/300.)) as i16;
            let ran;
            if boss.state <= 3{
                ran = 4;
            }else if boss.state <= 5{
                ran = 2;
            }else{
                ran = 1;
            }
            let mut rng = rand::thread_rng();
            boss.weapon = rng.gen_range(0..ran);
            if boss.weapon >= 2{
                commands.spawn(AudioPlayer::new(music_assets.beam.clone())).insert(NeedReload);
                let Ok(transform) = kid_query.get_single()
                else{
                    return ;
                } ;
                let y = ((transform.translation.y/32.)as i32 *32)as f32 + 16.;
                boss.pos = y;
                if trans.translation.x < BASEX{
                    boss.dir = 1.;
                }else{
                    boss.dir = -1.;
                }
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

fn do_split(
    mut commands: Commands,
    query: Query<(Entity, &Split, &Transform, &Velocity),With<Split>>,
    image_assets: Res<ImageAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>
){
    let apple_layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 2,1, None, None);
    let apple_atlas_layout = texture_atlases.add(apple_layout);
    let apple_atlas = TextureAtlas{
        layout : apple_atlas_layout,
        index : 0,
    };
    let apple_image = image_assets.apple.clone();
    for (entity, split, trans, velocity) in query.iter(){
        if trans.translation.x - split.size <= BASEX - 368. ||
            trans.translation.x + split.size >= BASEX + 368. ||
            trans.translation.y - split.size <= BASEY - 272. ||
            trans.translation.y + split.size >= BASEY + 272.{
            if split.time > 0{
                let vel = velocity.linvel;
                let mut ori = Vec2::new(vel.x, vel.y);
                let angle_step = PI / 2.0;
                for _ in 0..4 {
                    let app = spawn_single_apple(&mut commands, &apple_image, &apple_atlas, 
                        trans.translation.x, trans.translation.y,
                        0.5,  ori.clone(),
                    );
                    commands.entity(app).insert(Split{time:split.time-1,size:0.5*split.size}).insert(NeedReload);
                    ori = Vec2::new(ori.x*angle_step.cos() - ori.y*angle_step.sin(),
                        ori.x*angle_step.sin() + ori.y*angle_step.cos());
                }
            }
            commands.entity(entity).despawn();
        }
    }
}

fn do_beam(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<BeamTimer>,
    mut query: Query<(Entity, &mut Beam, &Transform, &mut Velocity),With<Beam>>,
){
    timer.0.tick(time.delta());
    if timer.0.finished() {
        for (entity, mut beam, trans, mut velocity) in query.iter_mut(){
            if beam.time > 0{
                beam.time -= 1;
            }else if beam.time==0{
                velocity.linvel = beam.speed * beam.direction;
                beam.time = -1;
            }else if trans.translation.x <= BASEX - 400. ||
                trans.translation.x >= BASEX + 400. ||
                trans.translation.y <= BASEY - 304. ||
                trans.translation.y >= BASEY + 304.{
                commands.entity(entity).despawn();
            }
        }
    }

}