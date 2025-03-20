use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Velocity{
    pub value_x: f32,
    pub value_y: f32,
}
impl Velocity{
    pub fn new(value_x: f32,value_y: f32) -> Self { Self {value_x,value_y}}
}


#[derive(Component, Debug)]
pub struct Accelerration{
    pub value: f32,
}
impl Accelerration{
    pub fn new(value: f32) -> Self { Self {value}}
}


#[derive(Bundle)]

pub struct MovingObjectBundle{
    pub velocity : Velocity,
    pub accelerration : Accelerration,
    pub transform: Transform,
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Update,update_position).add_systems(Update,update_velocity);
    }
}


fn update_position(mut query: Query<(&mut Velocity,&mut Transform)>,time: Res<Time>){
    for (mut velocity,mut transform) in query.iter_mut(){
        if transform.translation.y >= 0.0 || velocity.value_y >= 0.0{
            transform.translation += Vec3::new(velocity.value_x,velocity.value_y,0.0) * time.delta_secs();
        }else{
            velocity.value_y = 0.0;
            transform.translation += Vec3::new(velocity.value_x,0.0,0.0) * time.delta_secs();
        }
    }
}

fn update_velocity(mut query: Query<(&Transform,&Accelerration,&mut Velocity)>,time: Res<Time>){
    for (transform,accelerration,mut velocity) in query.iter_mut(){
        velocity.value_y += accelerration.value * time.delta_secs();
    }
}