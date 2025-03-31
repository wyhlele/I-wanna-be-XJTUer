use bevy::prelude::*;

#[derive(Debug, Default, States, Hash, Clone, Copy, Eq, PartialEq)]
pub enum GameState {
    #[default]
    InGame,
    Paused,
    Reload,
    GameOver,
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
        .insert_resource(State::new(GameState::InGame))
        .add_systems(Update, game_state_input_events);
    }
}

pub fn game_state_input_events(
    mut next_state: ResMut<NextState<GameState>>,
    state: Res<State<GameState>>,
    keyboard_input:Res<ButtonInput<KeyCode>>,
){
    if keyboard_input.just_pressed(KeyCode::KeyP){
        match state.get() {
            GameState::InGame => next_state.set(GameState::Paused),
            GameState::Paused => next_state.set(GameState::InGame),
            _ => ()
        }
    }
}
