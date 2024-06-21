use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::world::Kill;

pub struct PixelPlugin;

const PIXEL_FRAME_TIME: f32 = 1. / 10.;

impl Plugin for PixelPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<PlayerEvent>()
            .add_systems(Startup, spawn_pixel)
            .add_systems(Update, (flap, player_events, resets));
    }
}

#[derive(Component)]
enum PixelPlayer {
    Idel,
    WalkLeft,
    WalkRight,
}

#[derive(Default, Clone, Copy)]
enum Direction {
    Up,
    #[default]
    Down
}

impl core::ops::Mul<f32> for Direction {
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Self::Output {
        rhs * match self {
            Direction::Up => Vec3::Y,
            Direction::Down => Vec3::NEG_Y,
        }
    }
}

fn flap(
    mut player: Query<(&mut ExternalImpulse), With<PixelPlayer>>,
    input: Res<ButtonInput<KeyCode>>,
    mut residual: Local<f32>,
) {
    for mut player in &mut player {
        if input.pressed(KeyCode::KeyA) {
            player.impulse.x -= 1.;
        }
        if input.pressed(KeyCode::KeyD) {
            player.impulse.x += 1.;
        }
        if input.just_pressed(KeyCode::KeyW) {
            player.impulse.y += 16.;
            *residual = 8.;
        } else if input.pressed(KeyCode::KeyW) {
            player.impulse.y += *residual;
            *residual *= 0.5;
        }
        if input.pressed(KeyCode::KeyS) {
            player.impulse.y -= 0.5;
        }
    }
}

fn spawn_pixel(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshs: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
) {

    commands.spawn((PbrBundle {
        mesh: meshs.add(Cuboid::new(1., 1., 1.)),
        material: mats.add(StandardMaterial{
            base_color_texture: Some(asset_server.load("pixel.png")),
            ..Default::default()
        }),
        ..Default::default()
    }, RigidBody::Dynamic,
    ExternalImpulse::default(),
    PixelPlayer::Idel,
    Velocity::zero(),
    Collider::cuboid(0.5, 0.5, 0.5),
    GravityScale(5.),
    Restitution::coefficient(0.5),
    Damping {
        linear_damping: 0.5,
        angular_damping: 0.5,
    },
    CollidingEntities::default(),
    ActiveEvents::all(),
    LockedAxes::TRANSLATION_LOCKED_Z,
    )).with_children(|p| {

    });
}

#[derive(Event)]
enum PlayerEvent {
    Reset,
}

fn player_events(
    mut query: Query<(&mut Transform, &mut Velocity), With<PixelPlayer>>,
    mut events: EventReader<PlayerEvent>,
) {
    for event in events.read() {
        match event {
            PlayerEvent::Reset => {
                for (mut pos, mut vel) in &mut query {
                    *pos = Transform::IDENTITY;
                    *vel = Velocity::zero();
                }
            },
        }
    }
}

fn resets(
    mut player: EventWriter<PlayerEvent>,
    input: Res<ButtonInput<KeyCode>>,
    hits: Query<&CollidingEntities, With<PixelPlayer>>,
    death: Query<(), With<Kill>>
) {
    if input.just_pressed(KeyCode::Space) {
        player.send(PlayerEvent::Reset);
    }
    let hits = hits.single();
    for hit in hits.iter() {
        if death.contains(hit) {
            player.send(PlayerEvent::Reset);
            break;
        }
    }
}