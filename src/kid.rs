use std::alloc::Layout;

use bevy::prelude::*;
use bevy::input::ButtonInput;
use bevy::sprite::{TextureAtlas, TextureAtlasLayout,Sprite};

use crate::movement::{Velocity,Accelerration};

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0,0.0,-20.0);
const MOVE_SPEED: f32 = 1.0;

#[derive(Component, Debug)]
pub struct Kid{
    pub state:i32,
}
impl Kid{
    pub fn new(value: i32) -> Self { Self {state:value}}
}

#[derive(Resource)]
struct AnimationTimer(Timer);

pub struct KidPlugin;

impl Plugin for KidPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Startup,spawn_kid)
            .add_systems(Update,(kid_movement_controls,kid_sprite_controls));
    }
}

fn spawn_kid(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
){
    let image = asset_server.load("images/kid_all.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(25, 23), 4, 2, None, None);
    let atlas_layout = texture_atlases.add(layout);
    let atlas = TextureAtlas{
        layout : atlas_layout,
        index : 0,
    };
    commands.spawn((
        Sprite{
            image: image,
            texture_atlas: Some(atlas),
            ..Default::default()
        },
        Transform::from_xyz(0.0,0.0,0.0),
        Velocity::new(Vec3::ZERO),
        Accelerration::new(Vec3::ZERO),
        Kid{state: 0},
    ));

    commands.insert_resource(AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)));
}

fn kid_movement_controls(
    mut query: Query<(&mut Transform,&mut Velocity, &mut Kid),With<Kid>>,
    keyboard_input:Res<ButtonInput<KeyCode>>
){
    let (mut transform, mut velocity, mut kid) = query.single_mut();
    let mut movement = 0.0;
    if keyboard_input.pressed(KeyCode::KeyA){
        movement = -MOVE_SPEED;
        kid.state = 1;
    }else if keyboard_input.pressed(KeyCode::KeyD){
        movement = MOVE_SPEED;
        kid.state = 0;
    }
    velocity.value = -transform.forward() * movement;
    if kid.state==1{
        transform.scale = Vec3::new(-5.0,5.0,5.0);
    }else{
        transform.scale = Vec3::new(5.0,5.0,5.0);
    }
}

fn kid_sprite_controls(
    mut query: Query<(&mut Sprite,&mut Velocity),With<Kid>>,
    time: Res<Time>,
    mut timer: ResMut<AnimationTimer>,
){
    let (mut sprite, mut velocity) = query.single_mut();
    timer.0.tick(time.delta());

    // 如果计时器完成一次周期
    if timer.0.finished() {
        if velocity.value.x == 0.0{
            sprite.texture_atlas.clone().unwrap().index = (sprite.texture_atlas.clone().unwrap().index + 1) % 4;
        }
    }
    
}

