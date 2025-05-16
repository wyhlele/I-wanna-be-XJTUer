use bevy::prelude::*;

// const BEGINX: f32 = -256.0;
// const BEGINY: f32 = -224.0;

const BEGINX: f32 = 1600.0;
const BEGINY: f32 = 608.*2.+64.;

const SAVE: i8 = 9;

#[derive(Resource, Debug, Default)]
pub struct KidSaver{
    pub position: Vec2,
    pub save_id: i8,
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
        position: Vec2::new(BEGINX, BEGINY),
        save_id: SAVE,
    };
}