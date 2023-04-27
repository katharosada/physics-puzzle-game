use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::ball::Ball;

pub struct LevelPlugin;

#[derive(Component)]
struct Level;

struct ResetLevelEvent(Entity);

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_event::<ResetLevelEvent>()
            .add_system(handle_target_area_collision)
            .add_system(handle_level_reset_event)
            .add_system(detect_ball_out_of_bounds);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(shape::Plane::from_size(10.0).into()),
                material: materials.add(Color::rgb(0.1, 0.3, 0.6).into()),
                ..default()
            },
            RigidBody::Fixed,
            Collider::cuboid(5.0, 0.0, 5.0),
        ))
        .insert(TransformBundle::from(Transform::from_rotation(
            Quat::from_rotation_z((30.0_f32).to_radians()),
        )));

    // Target zone
    commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(shape::Plane::from_size(1.0).into()),
                material: materials.add(Color::rgb(1.0, 1.0, 0.3).into()),
                ..default()
            },
            RigidBody::Fixed,
            ActiveEvents::COLLISION_EVENTS,
            Collider::cuboid(0.5, 0.0, 0.5),
            Sensor,
        ))
        .insert(
            Transform::from_xyz(-4.0, -2.25, 0.0)
                .with_rotation(Quat::from_rotation_z((30.0_f32).to_radians())),
        );

    // Spawn ball
    commands.spawn(Ball);
}

fn handle_target_area_collision(
    mut query: Query<Entity, With<Ball>>,
    mut reset_level_events: EventWriter<ResetLevelEvent>,
    mut contact_events: EventReader<CollisionEvent>,
) {
    for contact_event in contact_events.iter() {
        println!("Collision event!");
        for entity in query.iter_mut() {
            if let CollisionEvent::Started(h1, h2, _event_flag) = contact_event {
                if h1 == &entity || h2 == &entity {
                    reset_level_events.send(ResetLevelEvent(entity));
                }
            }
        }
    }
}

fn detect_ball_out_of_bounds(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Ball>>,
    mut reset_level_events: EventWriter<ResetLevelEvent>,
) {
    for (entity, transform) in query.iter() {
        if transform.translation.y < -10.0 {
            eprintln!("Ball out of bounds!");
            commands.entity(entity).despawn();
            reset_level_events.send(ResetLevelEvent(entity));
        }
    }
}

fn handle_level_reset_event(
    ball_query: Query<Entity, (With<Ball>, With<Transform>)>,
    mut reset_level_events: EventReader<ResetLevelEvent>,
    mut commands: Commands,
) {
    for ev in reset_level_events.iter() {
        eprintln!("Entity {:?} reset the level", ev.0);
        for entity in ball_query.iter() {
            println!("Despawning ball {:?}", entity);
            commands.entity(entity).despawn();
        }
        commands.spawn(Ball);
    }
}
