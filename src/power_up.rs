use bevy::ecs::system::Despawn;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy::transform::commands;

use crate::collision::Collider;
use crate::player;
use crate::player::Player;
use crate::player::PlayerMovement;

pub struct PowerUpPlugin;

impl Plugin for PowerUpPlugin {
    fn build(&self, app:&mut App) {
        app
        .add_systems(Startup, spawn_power_ups)
        .add_systems(Update, power_up_system);
    }
}

const POWER_UP_SIZE:Vec3 = Vec3::new(8., 8., 0.);

#[derive(Component)]
struct PowerUp {
    power_up_type: PowerUpType
}

#[derive(PartialEq)]
enum PowerUpType {
    JumpPowerUp
}

fn spawn_power_ups(mut commands:Commands) {
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            custom_size: Some(POWER_UP_SIZE.truncate()),
            color: Color::RED,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(-32., 0., 0.),
            ..Default::default()
        },
        ..Default::default()
    },
    PowerUp {
        power_up_type: PowerUpType::JumpPowerUp
    }
    ));
}

/**
 * finds all powerups in map
 * grabs player
 * checks for collision on a powerup with player
 * if collided applies powerup to player
 */
fn power_up_system(
    mut power_up_query: Query<(Entity, &PowerUp, &Transform), With<PowerUp>>,
    mut player_query: Query<(&mut PlayerMovement, &Collider, &Transform), With<Player>>,
    mut commands: Commands
) {
    let (mut player_movement, player_collider, player_transform)= player_query.get_single_mut().unwrap();

    for (entity, power_up, power_up_transform) in &power_up_query {
        let collision = collide(
            power_up_transform.translation, POWER_UP_SIZE.truncate(),
            player_transform.translation, Vec2::new(player_collider.width, player_collider.height)
        );

        if collision.is_some() {
            if power_up.power_up_type == PowerUpType::JumpPowerUp {
                player_movement.jumps = 1;
            }
            commands.entity(entity).despawn_recursive();
        }
    }
}