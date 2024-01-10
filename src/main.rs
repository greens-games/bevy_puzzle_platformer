use bevy::prelude::*; 
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_puzzle_platformer::map::MapPlugin;
use bevy_puzzle_platformer::player::PlayerPlugin; 
use bevy_puzzle_platformer::collision::{CollisionPlugin, WallCollisionEvent, Static}; 
use bevy_puzzle_platformer::enemy::EnemyPlugin;
use bevy_puzzle_platformer::power_up::PowerUpPlugin;
/*
Puzzle platformer using Bevy.
Core Mechanics:
    Player movement (RUnning, Jump, Dash, etc...)
    Aim and throw a ball
    Solve puzzles by activating all switches
    Avoid traps, and enemies?
 */

 /*
 Test out more collision stuff using enemy,
 Remove enemy after
 Do some more collision refactoring
  */


fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(WorldInspectorPlugin::new())
    .add_plugins(MapPlugin)
    .add_plugins(PlayerPlugin)
    .add_plugins(EnemyPlugin)
    .add_plugins(CollisionPlugin)
    .add_plugins(PowerUpPlugin)
    .add_systems(Startup, initial_spawn)
    .add_event::<WallCollisionEvent>()
    .run();
}



fn initial_spawn(
    mut commands: Commands,
    window_query: Query<&Window>
) {
    let window = window_query.get_single().unwrap(); 
    let window_width= window.resolution.width(); 
    let window_height= window.resolution.height(); 
    commands.spawn((Camera2dBundle {
        transform: Transform::from_xyz(window_width/2., -window_height/2., 0.),
        ..Default::default()
    }, Static));
    println!("Window height: {}, Window width: {}",window.resolution.height(), window.resolution.width());
}
