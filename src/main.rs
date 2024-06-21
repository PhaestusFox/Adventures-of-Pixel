use bevy::{diagnostic::{FrameTimeDiagnosticsPlugin}, prelude::*};
use bevy_editor_pls::EditorPlugin;
use bevy_rapier3d::render::RapierDebugRenderPlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
    .add_systems(Startup, spawn_camera)
    .add_plugins((pixel::PixelPlugin, world::PipePlugin))
    .add_plugins(bevy_rapier3d::plugin::RapierPhysicsPlugin::<()>::default());
    
    #[cfg(debug_assertions)]
    app.add_plugins((EditorPlugin::default(), FrameTimeDiagnosticsPlugin::default(), RapierDebugRenderPlugin::default()));

    app.run()
}

mod pixel;
mod world;

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::Z * 50.),
            ..Default::default()
        });
}
