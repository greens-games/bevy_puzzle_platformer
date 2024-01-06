use std::vec;

use bevy::{prelude::*, sprite::collide_aabb::{collide, Collision}, transform};
use bevy_inspector_egui::quick::WorldInspectorPlugin;


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

const CELL_SIZE:Vec2 = Vec2::new(16., 16.);
const PLAYER_SIZE:Vec2 = Vec2::new(16., 16.);
const GRAVITY:f32 = 200.;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(WorldInspectorPlugin::new())
    .add_systems(Startup, initial_spawn)
    .add_systems(Update, ((move_player, move_enemy, collision_detect).chain())) // Due to events detect_collision needs to be done before move_Player
    .add_event::<WallCollisionEvent>()
    .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct PlayerMovement {
    jumps: i16,
    is_jumping: bool,
    velocity_x: f32,
    velocity_y: f32,
    dashes: i16
}

#[derive(Component)]
struct Static;

#[derive(Component)]
struct Collider {
    width: f32,
    height: f32
}

#[derive(Event, Clone, Copy)]
struct WallCollisionEvent {
    source: Entity,
    hit_wall: bool,
    collision_dir: Collision
}

#[derive(Component)]
struct Wall;

#[derive(Component)]
struct Floor;

fn initial_spawn(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), Static));

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


    //ENEMY REMOVE LATER
    commands.spawn((SpriteBundle {
        sprite : Sprite {
            custom_size : Some(PLAYER_SIZE),
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

    //spawn wall
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            custom_size : Some(Vec2::new(25.0, 100.0)),
            ..Default::default()
        },
        transform: Transform {
           translation: Vec3::new(60.0, 0.0, 0.0),
           ..Default::default() 
        },
        ..Default::default()
    }, 
    Static,
    Collider {
        width: 25.,
        height: 100.
    }
    ));


    //spawn floor
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            custom_size : Some(Vec2::new(100., 16.)),
            ..Default::default()
        },
        transform: Transform {
           translation: Vec3::new(0., -50., 0.),
           ..Default::default() 
        },
        ..Default::default()
    }, 
    Static,
    Collider {
        width: 100.,
        height: 16.
    }
    ));

}

fn collision_detect(
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
                Collision::Bottom => {falling = false}, //We are on floor
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
