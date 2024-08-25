use bevy::prelude::*;

mod components;
mod resources;
mod systems;
mod utils;

pub struct GasSimPlugin;

impl Plugin for GasSimPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::setup_gas_sim);
    }
}
