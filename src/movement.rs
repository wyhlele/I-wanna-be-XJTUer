use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Velocity{
    pub value: Vec3,
}
impl Velocity{
    pub fn new(value: Vec3) -> Self { Self {value}}
}


#[derive(Component, Debug)]
pub struct Accelerration{
    pub value: Vec3,
}
impl Accelerration{
    pub fn new(value: Vec3) -> Self { Self {value}}
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin{
    fn build(&self, app: &mut App){
        app
        .add_systems(Update,update_position)
        .add_systems(Update,update_velocity);
    }
}

fn update_position(mut query: Query<(&Velocity,&mut Transform)>,time: Res<Time>){
    for (velocity,mut transform) in query.iter_mut(){
        transform.translation += velocity.value * time.delta_secs();
    }
}

fn update_velocity(mut query: Query<(&Accelerration,&mut Velocity)>,time: Res<Time>){
    for (accelerration,mut velocity) in query.iter_mut(){
        velocity.value += accelerration.value * time.delta_secs();
    }
}