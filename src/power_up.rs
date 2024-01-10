use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

use crate::collision::Collider;
use crate::constants::scale_factor;
use crate::map::TileMap;
use crate::map::TileType;
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

fn spawn_power_ups(
    mut commands:Commands,
    tile_map: Res<TileMap>
) {
    for tile in &tile_map.map {
        match tile.tile_type {
            TileType::POWER_UP => {
                let start_x = f32::from(tile.x as i16) * (8. * scale_factor);
                let start_y = f32::from(tile.x as i16) * (8. * scale_factor);
                commands.spawn((SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(POWER_UP_SIZE.truncate().mul_add(Vec2::new(scale_factor, scale_factor), Vec2::ZERO)),
                        color: Color::RED,
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: Vec3::new(start_x, -start_y, 0.),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                PowerUp {
                    power_up_type: PowerUpType::JumpPowerUp
                }
                ));
            },
            _ => {}
            
        }
    }
}

/**
 * finds all powerups in map
 * grabs player
 * checks for collision on a powerup with player
 * if collided applies powerup to player
 */
fn power_up_system(
    power_up_query: Query<(Entity, &PowerUp, &Transform), With<PowerUp>>,
    mut player_query: Query<(&mut PlayerMovement, &Collider, &Transform), With<Player>>,
    mut commands: Commands
) {
    let (mut player_movement, player_collider, player_transform)= player_query.get_single_mut().unwrap();

    for (entity, power_up, power_up_transform) in &power_up_query {
        let collision = collide(
            power_up_transform.translation, POWER_UP_SIZE.truncate().mul_add(Vec2::new(scale_factor, scale_factor), Vec2::ZERO),
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