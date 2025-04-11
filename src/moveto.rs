use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::schedule::InGameSet;
use crate::asset_loader::ImageAssets;
use crate::apple::spawn_single_apple;
use crate::state::{GameState, NeedReload};

const EPSILON: f32 = 1.0;
const TO_RAD :f32 = 3.1415926 / 180.0;

pub struct MovePlugin;

#[derive(Component, Debug, Default)]
pub struct Move{
    pub goal_pos: Vec2,
    pub linear_speed: f32,
    pub goal_angle: f32,
    pub angle_speed: f32,
    pub status: i8,
}

impl Plugin for MovePlugin{
    fn build(&self, app: &mut App){
        app.add_systems(OnExit(GameState::Reload), spawn_something)
        .add_systems(Update, update_move.in_set(InGameSet::CalcAutoMove));

    }
}

fn spawn_something(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
){
    let apple_layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 2,1, None, None);
    let apple_atlas_layout = texture_atlases.add(apple_layout);
    let apple_atlas = TextureAtlas{
        layout : apple_atlas_layout,
        index : 0,
    };
    let apple_image = image_assets.apple.clone();
    
    let apple1 = spawn_single_apple(&mut commands, &apple_image, &apple_atlas, 320.0, 300.0);
    commands.entity(apple1).insert(
        Move{
            goal_pos: Vec2::new(320.0,300.0),
            linear_speed: 1500.0,
            goal_angle: 0.0,
            angle_speed: 0.0,
            status: 0,
        }
    ).insert(NeedReload);
}

fn update_move(
    mut query: Query<(&mut Move,&mut Transform,&mut Velocity),With<Move>>,
){
    for (config, trans, mut velocity) in query.iter_mut(){
        let x= trans.translation.x;
        let y= trans.translation.y;
        if (config.goal_pos - Vec2::new(x,y)).length() <= EPSILON{
            velocity.linvel = Vec2::ZERO;
        }else{
            velocity.linvel = (config.goal_pos - Vec2::new(x,y)).normalize_or_zero() * config.linear_speed;
        }

        let w = trans.rotation;
        if (w - Quat::from_rotation_z(config.goal_angle * TO_RAD)).length() <= EPSILON{
            velocity.angvel = config.angle_speed;
        }else{
            velocity.angvel = 0.0;
        }
    }
}