use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::{App, ClearColor, Color, Commands};
use bevy::DefaultPlugins;
use bevy::sprite::SpriteBundle;
use ns_defaults::camera::{CameraPlugin, GGFCamera2dBundle};

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::BLACK));
    app.add_plugin(CameraPlugin);
    app.add_startup_system(setup);
    app.add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default());

    app.run();
}

pub fn setup(mut commands: Commands){
    commands.spawn(SpriteBundle::default());
    commands
        .spawn(GGFCamera2dBundle::default());
}