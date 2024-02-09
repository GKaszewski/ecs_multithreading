use bevy::prelude::*;

mod game_of_life;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Game of Life".to_string(),
                    ..default()
                }),
                ..default()
            })
        )
        .add_plugins((game_of_life::GameOfLifePlugin,))
        .run();
}
