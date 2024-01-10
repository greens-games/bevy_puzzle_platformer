use bevy::{prelude::*, sprite::collide_aabb::{Collision, collide}};

use crate::map::{TileMap, TileType};


pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, spawn_map)
        .add_systems(Update, detect_collision);
    }
}

#[derive(Component)]
pub struct Static;

#[derive(Component)]
pub struct Collider {
    pub width: f32,
    pub height: f32
}

#[derive(Event, Clone, Copy)]
pub struct WallCollisionEvent {
    pub source: Entity,
    pub hit_wall: bool,
    pub collision_dir: Collision
}

fn spawn_map(
    mut commands:Commands,
    tile_map: Res<TileMap>
) {

    let scale_factor = 5.;
    for tile in &tile_map.map {
        match tile.tile_type {
            TileType::WALL=> {
                let start_x = f32::from(tile.x as i16) * (16. * scale_factor);
                let start_y = f32::from(tile.y as i16) * (16. * scale_factor);
                commands.spawn((SpriteBundle {
                    sprite: Sprite {
                        custom_size : Some(Vec2::new(16. * scale_factor, 16. * scale_factor)),
                        ..Default::default()
                    },
                    transform: Transform {
                    translation: Vec3::new(start_x, -start_y, 0.0),
                    ..Default::default() 
                    },
                    ..Default::default()
                },
                Static, 
                Collider {
                    width: 16. * scale_factor,
                    height: 16. * scale_factor
                }));
            },
            _ => {}
        }
    }
}

fn detect_collision(
    static_query: Query<(Entity, &Transform, &Collider), With<Collider>>,
    mut ev_collision: EventWriter<WallCollisionEvent>,
) {

    for (source, transform, collider) in &static_query {
        
        for (inner_source, inner_transform, inner_collider) in &static_query {

            if source.index() != inner_source.index() {
                //check for collision
                let collision = collide(
                    inner_transform.translation,
                    Vec2::new(inner_collider.width, inner_collider.height),
                    transform.translation,
                    Vec2::new(collider.width, collider.height)
                );

                if let Some(collision) = collision {
                        ev_collision.send(WallCollisionEvent {
                        source: source,
                        hit_wall: true,
                        collision_dir: collision
                    });
                }
            }        
        }
    }

}