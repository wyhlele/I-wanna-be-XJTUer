use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy::input::ButtonInput;
use bevy::sprite::{TextureAtlas, TextureAtlasLayout,Sprite};
use std::time::{Duration, Instant};

use crate::schedule::InGameSet;

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0,0.0,0.0);
const MOVE_SPEED: f32 = 200.0;
const JUMP_FIRST: f32 = 8.5;
const JUMP_SECOND: f32 = 7.0;
const JUMP_SCALE: f32 = 50.0;

#[derive(Component, Debug)]
pub struct Kid{
    pub state:i32,
    pub jump_time:i32,
}

#[derive(Resource)]
struct AnimationTimer(Timer);

pub struct KidPlugin;

impl Plugin for KidPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Startup, spawn_kid)
            .add_systems(
                Update,
                (shift_key_press_system,kid_movement_controls).chain().in_set(InGameSet::UserInput)
            )
            .add_systems(
                Update,
                kid_sprite_controls.in_set(InGameSet::EntityUpdates)
            );
    }
}

fn spawn_kid(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
){
    let image = asset_server.load("images/kid_all2.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(25, 24), 4, 6, None, None);
    let atlas_layout = texture_atlases.add(layout);
    let atlas = TextureAtlas{
        layout : atlas_layout,
        index : 0,
    };
    commands.spawn((
        Sprite{
            image: image.clone(),
            texture_atlas: Some(atlas.clone()),
            ..Default::default()
        },
        Kid{state: 0,jump_time: 2},
    )).insert(
        Transform::from_xyz(0.0,0.0,0.0)
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
        Collider::cuboid(6.0, 10.0)
    ).insert(
        Sleeping::disabled()
    ).insert(
        Ccd::enabled()
    ).insert(
        LockedAxes::ROTATION_LOCKED
    );


    commands.insert_resource(AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)));
}


fn kid_movement_controls(
    mut query: Query<(&mut Transform,&mut Velocity, &mut Kid),With<Kid>>,
    keyboard_input:Res<ButtonInput<KeyCode>>
){
    for (mut transform, mut velocity, mut kid) in query.iter_mut(){
        let mut movement = 0.0;
        if keyboard_input.pressed(KeyCode::ArrowLeft){
            movement = -MOVE_SPEED;
            kid.state = 1;
        }else if keyboard_input.pressed(KeyCode::ArrowRight){
            movement = MOVE_SPEED;
            kid.state = 0;
        }

        velocity.linvel.x = movement;

    }
}


fn shift_key_press_system(
    mut query: Query<(&mut Velocity, &mut Kid),With<Kid>>,
    keyboard_input:Res<ButtonInput<KeyCode>>,
) {

    for (mut velocity, mut kid) in query.iter_mut(){
        
        if keyboard_input.just_pressed(KeyCode::ShiftLeft) || keyboard_input.just_pressed(KeyCode::ShiftRight) {
            if kid.jump_time == 2{
                velocity.linvel.y = JUMP_SCALE * JUMP_FIRST;
                kid.jump_time = 1;
            }else if kid.jump_time == 1 {
                velocity.linvel.y = JUMP_SCALE * JUMP_SECOND;
                kid.jump_time = 0;
            }
        }

        if keyboard_input.just_released(KeyCode::ShiftLeft) || keyboard_input.just_released(KeyCode::ShiftRight) {
            if velocity.linvel.y > 0.0 {
                velocity.linvel.y *= 0.45; 
            }
        }

        if velocity.linvel.y.abs()< 0.5{
            kid.jump_time = 2;
        }else if velocity.linvel.y < -0.5 && kid.jump_time == 2{
            kid.jump_time -= 1;
        }
    }

    
    
}

fn kid_sprite_controls(
    mut query: Query<(&mut Sprite,&mut Velocity, &mut Kid),With<Kid>>,
    time: Res<Time>,
    mut timer: ResMut<AnimationTimer>,
){
    timer.0.tick(time.delta());
    if timer.0.finished() {
        for (mut sprite, velocity, kid) in query.iter_mut(){
            if let Some(atlas) = &mut sprite.texture_atlas{
                info!("{} {}",velocity.linvel,atlas.index);
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
}
