use bevy::{prelude::*, render::{settings::{Backends, WgpuSettings}, RenderPlugin}};

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
//        .add_plugins(DefaultPlugins)
        .add_plugins(
            DefaultPlugins.set(RenderPlugin {
                render_creation: WgpuSettings {
                    backends: Some(Backends::VULKAN),
                    ..default()
                }
                .into(),
                synchronous_pipeline_compilation: false,
            }),
        )
        .add_systems(Startup, spawn_spaceship)
        .add_systems(Update, (update_position, print_position))
        .run();
}

fn spawn_spaceship(mut commands: Commands) {
    commands.spawn((Position { x: 0.0, y: 0.0 }, Velocity { x: 1.0, y: 1.0 }));
}

fn update_position(mut query: Query<(&Velocity, &mut Position)>) {
    for (velocity, mut position) in query.iter_mut() {
        position.x += velocity.x;
        position.y += velocity.y;
    }
}

fn print_position(query: Query<(Entity, &Position)>) {
    // Log the entity ID and position of each entity with a `Position` component.
    for (entity, position) in query.iter() {
        info!("Entity {:?} is at position {:?},", entity, position);
    }
}
