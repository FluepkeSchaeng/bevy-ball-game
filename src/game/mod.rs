use bevy::prelude::*;

pub mod enemy;
mod player;
pub mod score;
pub mod star;
mod systems;
mod ui;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use systems::*;
use ui::GameUIPlugin;

use crate::{events::GameOver, AppState};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // States
            .add_state::<SimulationState>()
            // Events
            .add_event::<GameOver>()
            // OnEnter Systems
            .add_system(pause_simulation.in_schedule(OnEnter(AppState::Game)))
            // My Plugins
            .add_plugin(EnemyPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(ScorePlugin)
            .add_plugin(StarPlugin)
            .add_plugin(GameUIPlugin)
            // Systems
            .add_system(toggle_simulation.run_if(in_state(AppState::Game)))
            // Exit State Systems
            .add_system(resume_simulation.in_schedule(OnExit(AppState::Game)));
    }
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
