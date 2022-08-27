mod map_plugin;

use bevy::{
    prelude::*,
    window::{PresentMode, WindowMode},
    winit::WinitSettings,
};

fn main() {
    /*
     * Entities are things
     * Components can be assigned as a group to things
     * Systems process those things
     *
     * Resources represent state, or globally unique data
     */
    App::new()
        .insert_resource(WindowDescriptor {
            title: "I am a window!".to_string(),
            present_mode: PresentMode::AutoVsync,
            mode: WindowMode::BorderlessFullscreen,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(map_plugin::MapPlugin)
        .insert_resource(WinitSettings::desktop_app())
        .run();
}
