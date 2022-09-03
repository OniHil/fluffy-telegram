use crate::plugins::movement_plugin::{Movable, Zoomable};
use bevy::{prelude::*, transform::components::Transform};
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("map/Map_of_Verra.png"),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::splat(0.2),
                ..default()
            },
            ..default()
        })
        .insert(Movable)
        .insert(Zoomable);
}
