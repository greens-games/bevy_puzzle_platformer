use bevy::{prelude::*, sprite::collide_aabb::Collision};

use crate::collision::{Collider, WallCollisionEvent};
use crate::constants::GRAVITY;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, spawn_enemy)
        .add_systems(Update, move_enemy);
    }
}

#[derive(Component)]
struct Enemy;

fn spawn_enemy(mut commands:Commands) {
    //ENEMY REMOVE LATER
    commands.spawn((SpriteBundle {
        sprite : Sprite {
            custom_size : Some(Vec2::new(16., 16.)),
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(32., 110., 0.),
            ..Default::default()
        },
        ..Default::default()
    },
    Collider {
        width: 16.,
        height: 16.
    },
    Enemy
    )); 
}

fn move_enemy(
    mut query: Query<(Entity, &mut Transform), With<Enemy>>,
    mut ev_wall_collision: EventReader<WallCollisionEvent>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>
) {
    for (entity, mut transform) in &mut query {

        let mut falling:bool = true;
        for ev in ev_wall_collision.read() {
            if entity.index() == ev.source.index() {
                match ev.collision_dir {
                    Collision::Bottom => {falling = false}, //We are on floor
                    Collision::Top => {},
                    Collision::Left => {},
                    Collision::Right => {},
                    Collision::Inside => {},
                }
            }
        }

        if falling {
            transform.translation.y -= GRAVITY * time.delta_seconds();
        }
    }
}