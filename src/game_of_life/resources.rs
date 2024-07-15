use bevy::prelude::*;
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

#[derive(Resource, Debug)]
pub struct Grid {
    pub width: u32,
    pub height: u32,
}

#[derive(Resource)]
pub struct Generations(pub u32);

#[derive(Resource)]
pub struct CellPositions {
    pub map: HashMap<(i32, i32), bool>,
}

#[derive(Resource)]
pub struct CellsChanged(pub bool);

#[derive(Resource)]
pub struct CellMaterials {
    pub alive_material: Handle<ColorMaterial>,
    pub dead_material: Handle<ColorMaterial>,
}

#[derive(Resource, PartialEq, Eq)]
pub enum PlacementMode {
    Single,
    Block,
    Random,
}

#[derive(Resource)]
pub struct Durations(pub Vec<Duration>);

#[derive(Resource)]
pub struct SystemsMeasureTime(pub Instant);

#[derive(Resource)]
pub struct GlobalTime(pub Instant);
