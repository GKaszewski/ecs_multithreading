use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;
use std::time::Instant;

use crate::game_of_life::utils::save_durations_to_file;

use super::components::{CellBundle, Neighbors, Position};
use super::resources::{
    CellPositions, CellsChanged, Durations, Grid, PlacementMode, SystemsMeasureTime,
};
use super::SimulationState;

use super::components;

pub fn rebuild_cell_positions(
    query: Query<(&Position, &components::State)>,
    mut cell_positions: ResMut<CellPositions>,
    mut cells_changed: ResMut<CellsChanged>,
) {
    if !cells_changed.0 {
        return;
    }

    cell_positions.map.clear();
    for (pos, state) in query.iter() {
        cell_positions.map.insert((pos.x, pos.y), state.0);
    }

    cells_changed.0 = false;
}

pub fn spawn_cells(mut commands: Commands, grid: Res<Grid>, asset_server: Res<AssetServer>) {
    let start = Instant::now();
    let width = grid.width.clone();
    let height = grid.height;
    let cells_to_spawn_count = width * height;
    let texture: Handle<Image> = asset_server.load("cell.png");
    let to_spawn = (0..cells_to_spawn_count).map(move |i| {
        let x = i % width;
        let y = i / width;
        let position = Position {
            x: x as i32,
            y: y as i32,
        };
        let mut rng = rand::thread_rng();
        let state = components::State(rng.gen_bool(0.5));
        let sprite = SpriteBundle {
            sprite: Sprite {
                color: if state.0 { Color::GREEN } else { Color::BLACK },
                ..default()
            },
            texture: texture.clone(),
            transform: Transform::from_translation(Vec3::new(x as f32, y as f32, 0.0)),
            ..default()
        };
        (CellBundle {
            position,
            state,
            sprite,
            ..Default::default()
        },)
    });

    commands.spawn_batch(to_spawn);
    println!("Spawning {:?} cells", cells_to_spawn_count);
    let duration = start.elapsed();
    println!("Spawning cells took {:?}", duration);
}

pub fn initialize(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        ..Default::default()
    });
}

pub fn update_neighbors_brute_force_system(
    mut query: Query<(&mut Neighbors, &Position)>,
    grid: Res<Grid>,
    cell_positions: Res<CellPositions>,
) {
    query.par_iter_mut().for_each(|(mut neighbors, pos)| {
        let mut count = 0;
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let x = pos.x + dx;
                let y = pos.y + dy;

                if x >= 0 && x < (grid.width as i32) && y >= 0 && y < (grid.height as i32) {
                    if let Some(state) = cell_positions.map.get(&(x, y)) {
                        if *state {
                            count += 1;
                        }
                    }
                }
            }

            neighbors.0 = count;
        }
    });
}

pub fn update_cells_system(
    mut query: Query<(&mut components::State, &Neighbors, &mut Sprite)>,
    mut cells_changed: ResMut<CellsChanged>,
) {
    for (mut state, neighbors, mut sprite) in query.iter_mut() {
        let previous_state = state.0;
        match (state.0, neighbors.0) {
            (true, 2) | (true, 3) => (),
            (false, 3) => {
                state.0 = true;
                sprite.color = Color::GREEN;
            }
            _ => {
                state.0 = false;
                sprite.color = Color::BLACK;
            }
        }

        if state.0 != previous_state {
            cells_changed.0 = true;
        }
    }
}

pub fn handle_camera_system(
    mut query: Query<(&mut OrthographicProjection, &mut Transform, With<Camera>)>,
    keyboard_input: Res<Input<KeyCode>>,
    mut scroll_event: EventReader<MouseWheel>,
) {
    const SPEED: f32 = 5.0;
    const ZOOM_SPEED: f32 = 0.1;
    for (mut projection, mut transform, _) in query.iter_mut() {
        let mut translation = transform.translation;
        if keyboard_input.pressed(KeyCode::W) {
            translation.y += SPEED;
        }
        if keyboard_input.pressed(KeyCode::A) {
            translation.x -= SPEED;
        }
        if keyboard_input.pressed(KeyCode::S) {
            translation.y -= SPEED;
        }
        if keyboard_input.pressed(KeyCode::D) {
            translation.x += SPEED;
        }
        if keyboard_input.pressed(KeyCode::R) {
            projection.scale = 1.0;
        }

        transform.translation = translation;

        for event in scroll_event.read() {
            let scroll_delta = event.y;
            if scroll_delta > 0.0 {
                projection.scale *= 1.0 - ZOOM_SPEED;
            } else if scroll_delta < 0.0 {
                projection.scale *= 1.0 + ZOOM_SPEED;
            }
        }
    }
}

pub fn handle_placement_mode(
    mut placement_mode: ResMut<PlacementMode>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Key1) {
        *placement_mode = PlacementMode::Single;
    } else if keyboard_input.just_pressed(KeyCode::Key2) {
        *placement_mode = PlacementMode::Block;
    } else if keyboard_input.just_pressed(KeyCode::Key3) {
        *placement_mode = PlacementMode::Random;
    }
}

pub fn handle_cell_click_system(
    mut query: Query<(&Position, &mut Sprite, &mut components::State)>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera>>,
    mouse_button_input: Res<Input<MouseButton>>,
    windows_query: Query<&Window, With<PrimaryWindow>>,
    placement_mode: Res<PlacementMode>,
    mut cells_changed: ResMut<CellsChanged>,
) {
    let (camera, camera_transform) = camera_query.single();
    if let Some(position) = windows_query
        .single()
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        if mouse_button_input.just_pressed(MouseButton::Left) {
            for (pos, mut sprite, mut state) in query.iter_mut() {
                let x = position.x as i32;
                let y = position.y as i32;

                if x == pos.x && y == pos.y {
                    match *placement_mode {
                        PlacementMode::Single => {
                            state.0 = !state.0;
                            sprite.color = if state.0 { Color::GREEN } else { Color::BLACK };
                            cells_changed.0 = true;
                        }
                        PlacementMode::Random => {
                            let mut rng = rand::thread_rng();
                            state.0 = rng.gen_bool(0.5);
                            sprite.color = if state.0 { Color::GREEN } else { Color::BLACK };
                            cells_changed.0 = true;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

pub fn toggle_simulation_system(
    mut commands: Commands,
    simulation_state: Res<State<SimulationState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if *simulation_state == SimulationState::Paused {
            commands.insert_resource(NextState(Some(SimulationState::Running)));
        } else {
            commands.insert_resource(NextState(Some(SimulationState::Paused)));
        }
    }
}

pub fn do_one_step_system(
    mut commands: Commands,
    simulation_state: Res<State<SimulationState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Right) {
        if *simulation_state == SimulationState::Paused {
            commands.insert_resource(NextState(Some(SimulationState::Running)));
            commands.insert_resource(NextState(Some(SimulationState::Paused)));
        }
    }
}

pub fn start_measurement(mut commands: Commands) {
    commands.insert_resource(SystemsMeasureTime(Instant::now()));
}

pub fn stop_measurement(
    systems_measure_time: Res<SystemsMeasureTime>,
    mut durations: ResMut<Durations>,
) {
    let duration = systems_measure_time.0.elapsed();
    durations.0.push(duration);
    println!("System took {:?}", duration);

    save_durations_to_file(&durations);
}
