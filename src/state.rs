use bevy::prelude::*;

#[derive(Debug, Default, States, Hash, Clone, Copy, Eq, PartialEq)]
pub enum GameState {
    #[default]
    InGame,
    Paused,
    Reload,
    GameOver,
}

#[derive(Component, Debug, Default)]
pub struct NeedReload;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
        .insert_resource(State::new(GameState::Reload))
        .add_systems(Update, game_state_input_events)
        .add_systems(Update, reload_game);
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

fn reload_game(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    keyboard_input:Res<ButtonInput<KeyCode>>,
    query: Query<Entity,With<NeedReload>>,
){
    if keyboard_input.pressed(KeyCode::KeyR){
        next_state.set(GameState::Reload);
    }
    if keyboard_input.just_released(KeyCode::KeyR){
        for entity in &query{
            commands.entity(entity).despawn_recursive();
        }
        next_state.set(GameState::InGame);
    }
}