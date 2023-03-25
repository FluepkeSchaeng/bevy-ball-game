use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

//#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
//pub struct MovementSystemSet;

//#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
//pub struct ConfinementSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PlayerSystemSet {
    Movement,
    Confinement,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(PlayerSystemSet::Movement.before(PlayerSystemSet::Confinement))
            // Startup Systems
            .add_startup_system(spawn_player)
            // Systems
            .add_system(player_movement.in_set(PlayerSystemSet::Movement))
            .add_system(confine_player_movement.in_set(PlayerSystemSet::Confinement))
            .add_system(player_enemy_collision)
            .add_system(player_star_collision);
    }
}
