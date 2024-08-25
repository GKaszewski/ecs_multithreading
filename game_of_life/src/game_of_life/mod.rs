use std::{collections::HashMap, time::Instant};

use bevy::prelude::*;
use resources::{Durations, Generations, GlobalTime, SystemsMeasureTime};

use self::resources::{CellPositions, CellsChanged, Grid, PlacementMode};

mod components;
mod resources;
mod systems;
mod utils;

pub struct GameOfLifePlugin;

impl Plugin for GameOfLifePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SimulationState>()
            .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
            .insert_resource(Grid {
                width: 600,
                height: 400,
            })
            .insert_resource(CellsChanged(true))
            .insert_resource(CellPositions {
                map: HashMap::new(),
            })
            .insert_resource(PlacementMode::Single)
            .insert_resource(Durations(Vec::new()))
            .insert_resource(SystemsMeasureTime(Instant::now()))
            .insert_resource(GlobalTime(Instant::now()))
            .insert_resource(Generations(0))
            .add_systems(
                Startup,
                (
                    systems::spawn_cells_without_graphic,
                    systems::initialize.before(systems::spawn_cells),
                ),
            )
            .add_systems(
                Update,
                (
                    //systems::start_measurement,
                    systems::rebuild_cell_positions,
                    systems::update_neighbors_brute_force_system,
                    systems::update_cells_system,
                    systems::rebuild_cell_positions,
                    systems::update_neighbors_brute_force_system,
                    //systems::stop_measurement,
                )
                    .chain()
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_systems(
                Update,
                (
                    // systems::handle_camera_system,
                    // systems::handle_placement_mode,
                    // systems::handle_cell_click_system,
                    // systems::toggle_simulation_system,
                    // systems::do_one_step_system,
                    systems::exit_after_n_generations_system,
                ),
            );
    }
}

#[derive(States, Clone, Copy, Eq, PartialEq, Hash, Default, Debug)]
pub enum SimulationState {
    #[default]
    Paused,
    Running,
    Exit,
}
