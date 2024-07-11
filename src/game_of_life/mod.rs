use std::collections::HashMap;

use bevy::prelude::*;

use self::resources::{CellPositions, CellsChanged, Grid, PlacementMode};

mod components;
mod resources;
mod systems;

pub struct GameOfLifePlugin;

impl Plugin for GameOfLifePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SimulationState>()
            .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
            .insert_resource(Grid {
                width: 600,
                height: 400,
            })
            .insert_resource(CellsChanged(true))
            .insert_resource(CellPositions {
                map: HashMap::new(),
            })
            .insert_resource(PlacementMode::Single)
            .add_systems(
                Startup,
                (
                    systems::spawn_cells,
                    systems::initialize.before(systems::spawn_cells),
                ),
            )
            .add_systems(
                Update,
                (
                    systems::rebuild_cell_positions,
                    systems::update_neighbors_brute_force_system,
                    systems::update_cells_system,
                    systems::rebuild_cell_positions,
                    systems::update_neighbors_brute_force_system,
                )
                    .chain()
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_systems(
                Update,
                (
                    systems::handle_camera_system,
                    systems::handle_placement_mode,
                    systems::handle_cell_click_system,
                    systems::toggle_simulation_system,
                    systems::do_one_step_system,
                ),
            );
    }
}

#[derive(States, Clone, Copy, Eq, PartialEq, Hash, Default, Debug)]
pub enum SimulationState {
    #[default]
    Paused,
    Running,
}
