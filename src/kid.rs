use std::alloc::Layout;

use bevy::prelude::*;
use bevy::input::ButtonInput;
use bevy::sprite::{TextureAtlas, TextureAtlasLayout,Sprite};
use bevy::utils::info;

use crate::movement::{Velocity,Accelerration,MovingObjectBundle};

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0,0.0,-20.0);
const MOVE_SPEED: f32 = 200.0;

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
        app.add_systems(Startup, spawn_kid)
            .add_systems(Update,(kid_sprite_controls,kid_movement_controls));
    }
}

fn spawn_kid(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
){
    let image = asset_server.load("images/kid_all1.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(25, 24), 4, 3, None, None);
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
        Kid{state: 0},
    )).insert((
        MovingObjectBundle{
            transform:Transform::from_xyz(0.0,0.0,0.0),
            velocity:Velocity { value_x: (0.0), value_y: (0.0) },
            accelerration:Accelerration::new(-600.0),
        },
        Kid{state: 0},
    ));

    commands.insert_resource(AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)));
}

fn kid_movement_controls(
    mut query: Query<(&mut Transform,&mut Velocity, &mut Accelerration, &mut Kid),With<Kid>>,
    keyboard_input:Res<ButtonInput<KeyCode>>,
    time: Res<Time>
){
    for (mut transform, mut velocity,mut accelerration, mut kid) in query.iter_mut(){
    // let (mut transform, mut velocity,mut accelerration, mut kid) = query.single_mut();
        let mut movement = 0.0;
        if keyboard_input.pressed(KeyCode::KeyA){
            movement = -MOVE_SPEED;
            kid.state = 1;
        }else if keyboard_input.pressed(KeyCode::KeyD){
            movement = MOVE_SPEED;
            kid.state = 0;
        }
        if keyboard_input.pressed(KeyCode::ShiftLeft){
            velocity.value_y = 200.0;
        }
        velocity.value_x = movement;
        if kid.state==1{
            transform.scale = Vec3::new(-1.0,1.0,1.0);
        }else{
            transform.scale = Vec3::new(1.0,1.0,1.0);
        }
    }
}

fn kid_sprite_controls(
    mut query: Query<(&mut Sprite,&mut Velocity),With<Kid>>,
    time: Res<Time>,
    mut timer: ResMut<AnimationTimer>,
){
    timer.0.tick(time.delta());
    if timer.0.finished() {
        for (mut sprite, mut velocity) in query.iter_mut(){
            if let Some(atlas) = &mut sprite.texture_atlas{
                if velocity.value_y > 0.5{
                    atlas.index = 8;
                }else if velocity.value_y < -0.5{
                    atlas.index = 9;
                }else if velocity.value_x == 0.0{
                    atlas.index = (atlas.index + 1) % 4;
                }else{
                    atlas.index = (atlas.index + 1) % 4 + 4;
                }
            }
        }
    }
    
}
