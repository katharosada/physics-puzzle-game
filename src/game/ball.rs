use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct BallPlugin;

#[derive(Component)]
pub struct Ball;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BallResource>()
            .add_system(print_ball_altitude)
            .add_system(render_ball_system);
    }
}

#[derive(Resource)]
struct BallResource {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

impl FromWorld for BallResource {
    fn from_world(world: &mut World) -> Self {
        let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();
        let ball_mesh: Handle<Mesh> = meshes.add(Mesh::from(shape::UVSphere {
            radius: 0.2,
            sectors: 32,
            stacks: 32,
        }));

        let mut materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();

        let ball_material: Handle<StandardMaterial> =
            materials.add(Color::rgb(0.2, 0.8, 0.9).into());

        BallResource {
            mesh: ball_mesh,
            material: ball_material,
        }
    }
}

fn render_ball_system(
    mut commands: Commands,
    ball_resource: Res<BallResource>,
    query: Query<Entity, Added<Ball>>,
) {
    for entity in query.iter() {
        println!("Ball spawned: {:?}", entity);
        commands
            .entity(entity)
            .insert((
                PbrBundle {
                    mesh: ball_resource.mesh.clone(),
                    material: ball_resource.material.clone(),
                    ..default()
                },
                RigidBody::Fixed,
                Collider::ball(0.2),
                Restitution::coefficient(1.1),
            ))
            .insert(TransformBundle::from(Transform::from_xyz(4.0, 5.0, 0.0)));
    }
}

fn print_ball_altitude(positions: Query<&Transform, With<Ball>>) {
    for transform in positions.iter() {
        println!("Ball altitude: {}", transform.translation.y);
    }
}
