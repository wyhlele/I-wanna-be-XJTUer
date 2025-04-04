use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct KidSaver{
    pub position: Vec2,
    pub save_id: u8,
}

pub struct KidSaverPlugin;

impl Plugin for KidSaverPlugin{
    fn build(&self,app: &mut App){
        app.init_resource::<KidSaver>()
            .add_systems(Startup,create_saver);
    }
}

fn create_saver(
    mut kid_saver: ResMut<KidSaver>,
){
    *kid_saver = KidSaver{
        position: Vec2::new(0.0, 0.0),
        save_id: 0
    };
}