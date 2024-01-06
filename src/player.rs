use bevy::{prelude::*, sprite::collide_aabb::Collision};

use crate::collision::{Collider, WallCollisionEvent};
use crate::constants::GRAVITY;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, spawn_player)
        .add_systems(Update, move_player);
    }
}

const PLAYER_SIZE:Vec2 = Vec2::new(16., 16.);

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerMovement {
    jumps: i16,
    is_jumping: bool,
    velocity_x: f32,
    velocity_y: f32,
    dashes: i16
}

fn spawn_player(mut commands:Commands) {
    //spawn player
    commands.spawn((SpriteBundle {
        sprite : Sprite {
            custom_size : Some(PLAYER_SIZE),
            ..Default::default()
        },
        ..Default::default()
    },
    Player,
    Collider {
        width: 16.,
        height: 16.
    },
    PlayerMovement {
        jumps: 1,
        is_jumping: false,
        velocity_x: 200.,
        velocity_y: 0.,
        dashes: 1
    }
    ));
}

fn move_player(
    mut query: Query<(Entity, &mut Transform, &mut PlayerMovement), With<Player>>,
    mut ev_collision: EventReader<WallCollisionEvent>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>
) {
    let (entity, mut transform, mut player_movement) = query.get_single_mut().unwrap();

    let mut falling = true;
    let mut move_right:bool = true;
    let mut move_left:bool = true;
    for ev in ev_collision.read() {
        if ev.source.index() == entity.index() {
            match ev.collision_dir {
                Collision::Bottom => {
                    falling = false;
                    player_movement.velocity_y = 0.;
                }, //We are on floor
                Collision::Top => {},
                Collision::Left => {
                    move_left = false;
                },
                Collision::Right => {
                    move_right = false;
                },
                Collision::Inside => {println!("Nothing happened")},
            }
        }

    }



    if input.pressed(KeyCode::A) && move_left {
        transform.translation.x -= player_movement.velocity_x * time.delta_seconds();
    }

    if input.pressed(KeyCode::D) && move_right {
        transform.translation.x += player_movement.velocity_x * time.delta_seconds();
    }

    if input.just_pressed(KeyCode::Space) && !falling {
        player_movement.is_jumping = true;
        player_movement.velocity_y = 400.; //set y velocity to big number
    }

    if falling || player_movement.is_jumping {
        player_movement.is_jumping = false;
        if player_movement.velocity_y > 0. {
            player_movement.velocity_y -= GRAVITY * time.delta_seconds(); //each frame lower y velocity by gravity
        } 
        transform.translation.y += (player_movement.velocity_y - GRAVITY) * time.delta_seconds();
    }

}