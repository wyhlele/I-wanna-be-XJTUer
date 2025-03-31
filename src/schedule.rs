use bevy::prelude::*;

use crate::state::GameState;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum InGameSet{
    UserInput,
    EntityUpdates,
    CollisionDetection,
}

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin{
    fn build(&self, app: &mut App){
        app.configure_sets(
            Update,
            (
                InGameSet::CollisionDetection,
                InGameSet::UserInput,
                InGameSet::EntityUpdates,
            ).chain().run_if(in_state(GameState::InGame))
        )
        .add_systems(Update,
             apply_deferred
                .after(InGameSet::CollisionDetection)
                .before(InGameSet::UserInput)
        );
    }
}

