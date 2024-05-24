use bevy::{diagnostic::{FrameTimeDiagnosticsPlugin}, prelude::*};
use bevy_editor_pls::EditorPlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
    .add_systems(Startup, spawn_camera)
    .add_plugins(pixel::PixelPlugin);
    
    #[cfg(debug_assertions)]
    app.add_plugins((EditorPlugin::default(), FrameTimeDiagnosticsPlugin::default()));

    app.run()
}

mod pixel;

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default());
}
