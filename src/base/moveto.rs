use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::schedule::InGameSet;

const EPSILON: f32 = 1.0;
const TO_RAD :f32 = 3.1415926 / 180.0;

pub struct MovePlugin;

#[derive(Component, Default)]
pub struct Move{
    pub goal_pos: Vec2,
    pub linear_speed: f32,
    pub goal_angle: f32,
    pub angle_speed: f32,
    pub status: i8,
}

impl Plugin for MovePlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Update, update_move.in_set(InGameSet::CalcAutoMove));

    }
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