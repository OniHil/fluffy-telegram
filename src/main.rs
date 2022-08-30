use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::{PresentMode, WindowMode},
};

mod plugins;

fn main() {
    /*
     * Entities are things
     * Components can be assigned as a group to things
     * Systems process components
     *
     * Resources represent state, or globally unique data
     *
     * Get entities by Query<(Entity, &Resource?), With<Component>
     * and get values of resources by referencing Res for reading and ResMut for writing
     */
    App::new()
        .insert_resource(WindowDescriptor {
            present_mode: PresentMode::AutoVsync,
            mode: WindowMode::BorderlessFullscreen,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(plugins::movement_plugin::MovementPlugin)
        .add_plugin(plugins::ui_plugin::UIPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
