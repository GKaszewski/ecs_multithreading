use bevy::prelude::*;

#[derive(Component, PartialEq, Eq, Copy, Clone, Debug, Default)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, PartialEq, Eq, Default)]
pub struct State(pub bool);

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 { write!(f, "Alive") } else { write!(f, "Dead") }
    }
}

#[derive(Component, Debug, Default, PartialEq, Eq)]
pub struct Neighbors(pub u8);

#[derive(Bundle, Default)]
pub struct CellBundle {
    pub position: Position,
    pub state: State,
    pub neighbors: Neighbors,
    pub sprite: SpriteBundle,
}