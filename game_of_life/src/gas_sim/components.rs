use bevy::prelude::*;

#[derive(Component)]
pub struct GasParticle;

#[derive(Component)]
pub struct Size(pub f32); // radius, because it's a sphere

#[derive(Component)]
pub struct Position(pub Vec3);

#[derive(Component)]
pub struct Velocity(pub Vec3);

#[derive(Component)]
pub struct Acceleration(pub Vec3);

#[derive(Component)]
pub struct Force(pub Vec3);

#[derive(Component)]
pub struct Mass(pub f32);

#[derive(Component)]
pub struct Density(pub f32);
