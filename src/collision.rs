use bevy::{prelude::*, sprite::collide_aabb::{Collision, collide}};


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

fn spawn_map(mut commands:Commands) {
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