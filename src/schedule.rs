use bevy::prelude::*;

use crate::state::GameState;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum InGameSet{
    UserInput,
    CalcAutoMove,
    EntityUpdates,
    CameraFollowed,
    SaveSpawnPoint,
    EmptyState,
}

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin{
    fn build(&self, app: &mut App){
        app.configure_sets(
            Update,
            (
                InGameSet::EmptyState,
                InGameSet::UserInput,
                InGameSet::CalcAutoMove,
                InGameSet::EntityUpdates,
                InGameSet::CameraFollowed,
                InGameSet::SaveSpawnPoint,
            ).chain().run_if(in_state(GameState::InGame))
        )
        .add_systems(Update,
             apply_deferred
                .after(InGameSet::EmptyState)
                .before(InGameSet::UserInput)
        );
    }
}

