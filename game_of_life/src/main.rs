use std::time::Duration;

use bevy::{app::ScheduleRunnerPlugin, prelude::*};

mod game_of_life;
mod gas_sim;

fn main() {
    App::new()
        // .add_plugins(
        //     MinimalPlugins
        //         .set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
        //             1.0 / 60.0,
        //         )))
        //         .set(TaskPoolPlugin {
        //             task_pool_options: TaskPoolOptions::with_num_threads(12),
        //         }),
        // )
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Gas Sim".to_string(),
                present_mode: bevy::window::PresentMode::Immediate,
                ..default()
            }),
            ..default()
        }))
        //.add_plugins((game_of_life::GameOfLifePlugin,))
        .add_plugins(gas_sim::GasSimPlugin)
        .run();
}
