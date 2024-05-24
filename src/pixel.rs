use bevy::prelude::*;

pub struct PixelPlugin;

const PIXEL_FRAME_TIME: f32 = 1. / 10.;

impl Plugin for PixelPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_pixel)
            .add_systems(Update, (idel_animate_pixel, walk_animate_pixel))
            .add_systems(Update, walk);
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

fn idel_animate_pixel(
    time: Res<Time>,
    mut query: Query<&mut Transform>,
    pixel: Query<&Children, With<PixelPlayer>>,
    mut direction: Local<Direction>
) {
    let pixel = pixel.single();
    let head = pixel[0];
    let legs = pixel[1];
    let Ok(mut head) = query.get_mut(head) else {error!("\"Pixel\" as no Head"); return;};
    if head.translation.y > 2.5 {*direction = Direction::Down}
    else if head.translation.y < -2.5 {*direction = Direction::Up}

    head.translation += *direction * time.delta_seconds() * 2.;
}

fn walk_animate_pixel(
    mut query: Query<(&mut Sprite, &mut TextureAtlas)>,
    pixel: Query<(&Children, &PixelPlayer)>,
    time: Res<Time>,
    mut elapsed: Local<f32>,
) {
    *elapsed += time.delta_seconds();
    if *elapsed < PIXEL_FRAME_TIME {return;}
    *elapsed -= PIXEL_FRAME_TIME;
    let pixel = pixel.single();
    let head = pixel.0[0];
    let legs = pixel.0[1];
    let Ok((mut sprite, mut atlas) ) = query.get_mut(legs) else {error!("\"Pixel\" as no Head"); return;};
    fn next_index(index: usize) -> usize {
        ((index / 10) % 4 + 1)  * 10
    }
    match pixel.1 {
        PixelPlayer::Idel => {
            atlas.index = 0
        },
        PixelPlayer::WalkLeft => {
            sprite.flip_x = true;
            atlas.index = next_index(atlas.index);
        },
        PixelPlayer::WalkRight => {
            sprite.flip_x = false;
            atlas.index = next_index(atlas.index);
        },
    }
}

fn walk(
    input: Res<ButtonInput<KeyCode>>,
    mut pixels: Query<(&mut Transform, &mut PixelPlayer), With<PixelPlayer>>,
) {
    for (mut transform, mut pixel) in &mut pixels {
        let mut delta = 0.;
        if input.any_just_released([KeyCode::ArrowLeft, KeyCode::AltRight, KeyCode::KeyA, KeyCode::KeyD]) {
            *pixel = PixelPlayer::Idel;
        }
        for key in input.get_pressed() {
            match key {   
                KeyCode::ArrowLeft | KeyCode::KeyA => {
                    delta = -1.;
                    *pixel = PixelPlayer::WalkLeft;
                },
                KeyCode::ArrowRight | KeyCode::KeyD => {
                    delta = 1.;
                    *pixel = PixelPlayer::WalkRight;
                },
                _ => {}
            }
        }
        transform.translation.x += delta * 2.;
    }
}

fn spawn_pixel(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas: ResMut<Assets<TextureAtlasLayout>>
) {
    commands
        .spawn((SpatialBundle::default(), PixelPlayer::Idel))
        .with_children(|p| {
            p.spawn(SpriteSheetBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(80.)),
                    ..Default::default()
                },
                texture: asset_server.load("sheet_0.png"),
                atlas: TextureAtlas {
                    index: 1,
                    layout: atlas.add(TextureAtlasLayout::from_grid(Vec2::splat(16.), 10, 10, None, None)),
                },
                transform: Transform::from_translation(Vec3::Z),
                ..Default::default()
            });
            p.spawn(SpriteSheetBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(80.)),
                    ..Default::default()
                },
                texture: asset_server.load("sheet_0.png"),
                atlas: TextureAtlas {
                    index: 0,
                    layout: atlas.add(TextureAtlasLayout::from_grid(Vec2::splat(16.), 10, 10, None, None)),
                },
                ..Default::default()
            });
            p.spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(15.)),
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3::new(15., 20., 2.)),
                texture: asset_server.load("eye_0.png"),
                ..Default::default()
            }).with_children(|e| {
                e.spawn(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::splat(3.32)),
                        ..Default::default()
                    },
                    transform: Transform::from_translation(Vec3::new(-2., -3., 2.)),
                    texture: asset_server.load("pupel.png"),
                    ..Default::default()
                });
            });
            p.spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(15.)),
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3::new(-15., 20., 2.)),
                texture: asset_server.load("eye_0.png"),
                ..Default::default()
            }).with_children(|e| {
                e.spawn(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::splat(3.42)),
                        ..Default::default()
                    },
                    transform: Transform::from_translation(Vec3::new(1., -2., 2.)),
                    texture: asset_server.load("pupel.png"),
                    ..Default::default()
                });
            });
            
        });
}
