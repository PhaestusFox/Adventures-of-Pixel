#[derive(Component)]
pub struct Pipe;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier3d::prelude::*;
use rand::Rng;

#[derive(Resource)]
struct PipeAssets {
    pipe: Handle<Mesh>,
    ground: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

impl FromWorld for PipeAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        PipeAssets {
            pipe: asset_server.load("pipe.glb#Mesh0/Primitive0"),
            ground: asset_server.load("pipe.glb#Mesh1/Primitive0"),
            material: world.resource_mut::<Assets<StandardMaterial>>().add(StandardMaterial {
                base_color: Color::GREEN,
                ..Default::default()
            }),
        }
    }
}

#[derive(Component)]
pub struct Kill;

pub struct PipePlugin;

impl Plugin for PipePlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<PipeAssets>()
        .add_systems(Startup, spawn_ground)
        .add_systems(Update, (move_pipe, spawn_pipe, despawn_pipe));
    }
}

fn spawn_ground(
    mut commands: Commands,
    assets: Res<PipeAssets>,
) {
    commands.spawn((PbrBundle {
        mesh: assets.ground.clone(),
        material: assets.material.clone(),
        transform: Transform::from_translation(Vec3::NEG_Y * 12.).with_scale(Vec3::new(10., 2., 10.)),
        ..Default::default()
    }, RigidBody::Fixed, Collider::cuboid(5., 0.5, 5.), Kill));
}

fn move_pipe(
    mut pipes: Query<&mut Transform, With<Pipe>>,
    time: Res<Time>
) {
    for mut pipe in &mut pipes {
        pipe.translation.x -= 10. * time.delta_seconds();
    }
}

fn spawn_pipe(
    mut commands: Commands,
    assets: Res<PipeAssets>,
    time: Res<Time>,
    mut last: Local<f32>,
    mesh: Res<Assets<Mesh>>,
) {
    *last += time.delta_seconds();
    if *last < 0. {return;}
    *last -= 2.;

    let offset = rand::thread_rng().gen_range(-10f32..18.0);

    let Some(pipe_mesh) = mesh.get(assets.pipe.id()) else {error!("Pipe Mesh Not Loaded"); return;};
    let pipe_colider = Collider::from_bevy_mesh(pipe_mesh, &ComputedColliderShape::TriMesh).expect("Mesh to Work");

    commands.spawn((SpatialBundle {
        transform: Transform::from_translation(Vec3::X * 35.),
        ..Default::default()
    }, Pipe)).with_children(|p| {
        p.spawn((PbrBundle {
            material: assets.material.clone(),
            mesh: assets.pipe.clone(),
            transform: Transform::from_translation(Vec3::new(0.,offset + 1.5, 0.)),
            ..Default::default()
        }, pipe_colider.clone(), Kill));

        p.spawn((
            
        ));

        p.spawn((PbrBundle {
            material: assets.material.clone(),
            mesh: assets.pipe.clone(),
            transform: Transform::from_rotation(Quat::from_rotation_z(180.0f32.to_radians())).with_translation(Vec3::new(0.,offset - 1.5, 0.)),
            ..Default::default()
        }, pipe_colider, Kill));
    });
}

fn despawn_pipe(
    pipes: Query<(Entity, &Transform), With<Pipe>>,
    mut commands: Commands
) {
    for (pipe, pos) in &pipes {
        if pos.translation.x < -35. {
            commands.entity(pipe).despawn_recursive();
        }
    }
}