use bevy::prelude::*; 
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_puzzle_platformer::player::PlayerPlugin; 
use bevy_puzzle_platformer::collision::{CollisionPlugin, WallCollisionEvent, Static}; 
use bevy_puzzle_platformer::enemy::EnemyPlugin;
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
    .add_plugins(PlayerPlugin)
    .add_plugins(EnemyPlugin)
    .add_plugins(CollisionPlugin)
    .add_systems(Startup, initial_spawn)
    .add_event::<WallCollisionEvent>()
    .run();
}



fn initial_spawn(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), Static));
}
