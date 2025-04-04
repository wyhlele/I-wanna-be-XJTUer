use bevy::prelude::*;

use crate::state::GameState;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum InGameSet{
    UserInput,
    EntityUpdates,
    CameraFollowed,
}

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin{
    fn build(&self, app: &mut App){
        app.configure_sets(
            Update,
            (
                InGameSet::CameraFollowed,
                InGameSet::UserInput,
                InGameSet::EntityUpdates,
            ).chain().run_if(in_state(GameState::InGame))
        )
        .add_systems(Update,
             apply_deferred
                .after(InGameSet::CameraFollowed)
                .before(InGameSet::UserInput)
        );
    }
}

