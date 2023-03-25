use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Startup Systems
            .add_startup_system(spawn_player)
            // Systems
            .add_systems((player_movement, confine_player_movement).chain())
            .add_system(player_enemy_collision)
            .add_system(player_star_collision);
    }
}
