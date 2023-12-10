use bevy::{prelude::*, ecs::query, diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}};

#[derive(Component, Debug)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component, Debug)]
struct Velocity {
    x: f32,
    y: f32,
}

fn main() {
    App::new()
        //.add_systems(Startup, spawn_spaceship)
        .add_systems(Update,(spawn_spaceship, update_position))
        .add_plugins(DefaultPlugins)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(LogDiagnosticsPlugin::default())
        .run();
}

fn spawn_spaceship(mut commands: Commands) {
    commands.spawn((Position {x:0.0, y: 0.0 }, Velocity {x: 1.0, y: 1.0 }));
}

fn update_position(mut query: Query<(&Velocity, &mut Position)>) {
    for (v, mut p) in query.iter_mut() {
        p.x += v.x;
        p.y += v.y;
    }
}

fn print_position(query: Query<(Entity, &Position)>) {
    // Log the entity ID and position of each with a 'Position' component
    for (entity, position) in query.iter() {
        info!("Entity: {:?} is at position {:?},", entity, position)
    }
}