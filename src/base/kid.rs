use bevy::audio::{PlaybackMode, Volume};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng; 
use bevy::input::ButtonInput;
use bevy::sprite::{TextureAtlas, TextureAtlasLayout,Sprite};

use crate::asset_loader::{ImageAssets, MusicAssets};
use crate::festival::level2::Bike;
use crate::kid_saver::KidSaver;
use crate::schedule::InGameSet;
use crate::state::{GameState, NeedReload, BGM};
use crate::base::trap::Trap;

const MOVE_SPEED: f32 = 150.0;
const JUMP_FIRST: f32 = 8.5;
const JUMP_SECOND: f32 = 7.0;
const JUMP_SCALE: f32 = 50.0;

#[derive(Component, Debug)]
pub struct Kid{
    pub state:i32,
    pub jump_time:i32,
    pub dead: i32
}

#[derive(Resource)]
pub struct AnimationTimer(Timer);

pub struct KidPlugin;

impl Plugin for KidPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(PostStartup, spawn_timer)
            .add_systems(OnExit(GameState::Reload), spawn_kid)
            .add_systems(OnEnter(GameState::GameOver), do_dead)
            .add_systems(
                Update,
                (
                    kid_jump_controls,
                    kid_movement_controls
                ).chain().in_set(InGameSet::UserInput)
            )
            .add_systems(
                Update,
                kid_sprite_controls.in_set(InGameSet::EntityUpdates)
            ).add_systems(
                Update, kid_display_events
            );
    }
}

fn spawn_timer(
    mut commands: Commands, 
){
    commands.insert_resource(AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)));
}


pub fn spawn_single_kid(
    commands: &mut Commands,
    sprtie: &Handle<Image>,
    atlas: &TextureAtlas,
    x: f32,y: f32,
){
    commands.spawn((
        Sprite{
            image: sprtie.clone(),
            texture_atlas: Some(atlas.clone()),
            ..Default::default()
        },
        Kid{state: 0,jump_time: 1, dead: 0},
    )).insert(
        Transform::from_xyz(x,y,0.0)
    ).insert(
        Velocity {
            linvel: Vec2::new(0.0, 0.0),
            angvel: 0.0,
        }
    ).insert(
        RigidBody::Dynamic
    ).insert(
        GravityScale(1.0)
    ).insert(
        Collider::cuboid(6.3, 10.5)
    ).insert(
        ColliderMassProperties::Mass(2.0)
    ).insert(
        Sleeping::disabled()
    ).insert(
        Ccd::enabled()
    ).insert(
        LockedAxes::ROTATION_LOCKED
    ).insert(
        CollisionGroups::new(
            Group::GROUP_1,
            Group::GROUP_2|Group::GROUP_3|Group::GROUP_4,
        )
    ).insert(SolverGroups::new(
        Group::GROUP_1,
        Group::GROUP_2|Group::GROUP_3,
        )
    ).insert(
        KinematicCharacterController::default()
    ).insert(ActiveEvents::COLLISION_EVENTS)
    .insert(NeedReload);
}

fn spawn_kid(
    mut commands: Commands, 
    image_assets: Res<ImageAssets>,
    kid_saver: Res<KidSaver>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
){
    let layout = TextureAtlasLayout::from_grid(UVec2::new(25, 24), 4, 6, None, None);
    let atlas_layout = texture_atlases.add(layout);
    let atlas = TextureAtlas{
        layout : atlas_layout,
        index : 0,
    };
    commands.spawn((
        Sprite{
            image: image_assets.kid.clone(),
            texture_atlas: Some(atlas.clone()),
            ..Default::default()
        },
        Kid{state: 0,jump_time: 1,dead: 0},
    )).insert(
        Transform::from_xyz(kid_saver.position.x,kid_saver.position.y,0.0)
    ).insert(
        Velocity {
            linvel: Vec2::new(0.0, 0.0),
            angvel: 0.0,
        }
    ).insert(
        RigidBody::Dynamic
    ).insert(
        GravityScale(1.0)
    ).insert(
        Collider::cuboid(6.0, 10.5)
    ).insert(
        ColliderMassProperties::Mass(2.0)
    ).insert(
        Sleeping::disabled()
    ).insert(
        Ccd::enabled()
    ).insert(
        LockedAxes::ROTATION_LOCKED
    ).insert(
        Friction::coefficient(0.0)
    ).insert(
        CollisionGroups::new(
            Group::GROUP_1,
            Group::GROUP_2|Group::GROUP_3|Group::GROUP_4,
        )
    ).insert(SolverGroups::new(
        Group::GROUP_1,
        Group::GROUP_2|Group::GROUP_3,
        )
    ).insert(
        KinematicCharacterController::default()
    ).insert(ActiveEvents::COLLISION_EVENTS)
    .insert(NeedReload);
}


fn kid_movement_controls(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform, &mut Velocity, &mut Kid),With<Kid>>,
    keyboard_input:Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
){
    let Ok((entity, trans ,mut velocity, mut kid)) = query.get_single_mut() 
    else {
        return;
    };
    if keyboard_input.pressed(KeyCode::ArrowLeft){
        velocity.linvel.x = -MOVE_SPEED;
        kid.state = 1;
    }else if keyboard_input.pressed(KeyCode::ArrowRight){
        velocity.linvel.x = MOVE_SPEED;
        kid.state = 0;
    }else if keyboard_input.pressed(KeyCode::KeyQ){
        next_state.set(GameState::GameOver);
        let mut rng = rand::thread_rng();
        let direction = Vec2::new(
            rng.gen_range(-1.0..=1.0),
            rng.gen_range(-1.0..=1.0),
        ).normalize();
        commands.entity(entity).insert(
            ExternalImpulse::at_point(
            direction * 1600.0, 
            trans.translation.truncate(), 
            trans.translation.truncate()
        ));
    }else{
        velocity.linvel.x = 0.;
    }
}


fn kid_jump_controls(
    mut commands: Commands,
    mut query: Query<(&mut Velocity, &mut Kid),With<Kid>>,
    keyboard_input:Res<ButtonInput<KeyCode>>,
    music_assets: Res<MusicAssets>,
) {
    let Ok((mut velocity, mut kid)) = query.get_single_mut() 
    else {
        return;
    };
    if keyboard_input.just_pressed(KeyCode::ShiftLeft) || keyboard_input.just_pressed(KeyCode::ShiftRight) {
        if kid.jump_time == 2{
            velocity.linvel.y = JUMP_SCALE * JUMP_FIRST;
            kid.jump_time = 1;
            commands.spawn(AudioPlayer::new(music_assets.jump1.clone()));
        }else if kid.jump_time == 1 {
            velocity.linvel.y = JUMP_SCALE * JUMP_SECOND;
            kid.jump_time = 0;
            commands.spawn(AudioPlayer::new(music_assets.jump2.clone()));
        }
    }
    if keyboard_input.just_released(KeyCode::ShiftLeft) || keyboard_input.just_released(KeyCode::ShiftRight) {
        if velocity.linvel.y > 0.0 {
            velocity.linvel.y *= 0.45; 
        }
    }
    if velocity.linvel.y.abs()< 0.001{
        kid.jump_time = 2;
    }else if velocity.linvel.y < -0.001 && kid.jump_time == 2{
        kid.jump_time -= 1;
    }
    if velocity.linvel.y < -9.4 * JUMP_SCALE{
        velocity.linvel.y = -9.4 * JUMP_SCALE;
    }
}

fn kid_sprite_controls(
    mut query: Query<(&mut Sprite,&mut Velocity, &mut Kid),With<Kid>>,
    time: Res<Time>,
    mut timer: ResMut<AnimationTimer>,
){
    timer.0.tick(time.delta());
    if timer.0.finished() {
        let Ok((mut sprite, velocity, kid)) = query.get_single_mut() 
        else {
            return;
        };
        if let Some(atlas) = &mut sprite.texture_atlas{
            if kid.state == 0{
                if velocity.linvel.y > 0.5{
                    atlas.index = 8;
                }else if velocity.linvel.y < -0.5{
                    atlas.index = 9;
                }else if velocity.linvel.x.abs() < 0.5{
                    atlas.index = (atlas.index + 1) % 4;
                }else{
                    atlas.index = (atlas.index + 1) % 4 + 4;
                }
            }else{
                if velocity.linvel.y > 0.5{
                    atlas.index = 23;
                }else if velocity.linvel.y < -0.5{
                    atlas.index = 22;
                }else if velocity.linvel.x.abs() < 0.5{
                    atlas.index = (atlas.index + 3) % 4 + 12;
                }else{
                    atlas.index = (atlas.index + 3) % 4 + 16;
                }
            }
        }
    }
}


fn kid_display_events(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    state: Res<State<GameState>>,
    mut collision_events: EventReader<CollisionEvent>,
    mut kid_query: Query<&mut Kid>,
    trap_query: Query<&Trap>,
    transforms: Query<&Transform>,
    bike_query: Query<&Bike>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity_a, entity_b, _) => {
                let is_entity1_b = kid_query.get(*entity_b).is_ok();
                let is_entity2_a = trap_query.get(*entity_a).is_ok();
                let is_bike1 = bike_query.get(*entity_a).is_ok();
                if is_entity1_b && is_entity2_a{
                    commands.entity(*entity_b).remove::<LockedAxes>();

                    let transform_a = transforms.get(*entity_a).unwrap();
                    let transform_b = transforms.get(*entity_b).unwrap();
                    let direction = (transform_b.translation.truncate() 
                        - transform_a.translation.truncate()).normalize();

                    commands.entity(*entity_b).insert(
                        ExternalImpulse::at_point(
                        direction * 1600.0, 
                        transform_a.translation.truncate(), 
                        transform_b.translation.truncate()
                    ));
                    if is_bike1{
                        for mut item in kid_query.iter_mut(){
                            item.dead = 1;
                        }
                    }else{
                        for mut item in kid_query.iter_mut(){
                            item.dead = 0;
                        }
                    }
                    match state.get() {
                        GameState::InGame => next_state.set(GameState::GameOver),
                        _ => {},
                    }
                }


                let is_entity1_a = kid_query.get(*entity_a).is_ok();
                let is_entity2_b = trap_query.get(*entity_b).is_ok();
                let is_bike2 = bike_query.get(*entity_b).is_ok();
                if is_entity1_a && is_entity2_b{
                    commands.entity(*entity_a).remove::<LockedAxes>();

                    let transform_a = transforms.get(*entity_a).unwrap();
                    let transform_b = transforms.get(*entity_b).unwrap();
                    let direction = (transform_a.translation.truncate() 
                        - transform_b.translation.truncate()).normalize();

                    commands.entity(*entity_a).insert(
                        ExternalImpulse::at_point(
                        direction * 1600.0, 
                        transform_b.translation.truncate(), 
                        transform_a.translation.truncate()
                    ));
                    if is_bike2{
                        for mut item in kid_query.iter_mut(){
                            item.dead = 1;
                        }
                    }else{
                        for mut item in kid_query.iter_mut(){
                            item.dead = 0;
                        }
                    }
                    match state.get() {
                        GameState::InGame => next_state.set(GameState::GameOver),
                        _ => {},
                    }
                }
            }
            _ => {}
        }
    }
}

fn do_dead(
    mut commands: Commands,
    bgm_query: Query<Entity,With<BGM>>,
    kid_query: Query<&Kid>,
    music_assets: Res<MusicAssets>,
){
    let kid = kid_query.single().dead;
    let mut bgm = music_assets.dead.clone();
    let mut vol= 0.3;
    if kid == 1{
        bgm = music_assets.bike2.clone();
        vol = 1.0;
    }
    commands.spawn(AudioPlayer::new(bgm))
    .insert(PlaybackSettings {
        mode: PlaybackMode::Once,
        volume: Volume::new(vol),
        speed: 1.0,
        paused: false,
        spatial: false,
        spatial_scale: None,
    })
    .insert(NeedReload);
    for bgm in bgm_query.iter(){
        commands.entity(bgm).despawn_recursive();
    }
}
