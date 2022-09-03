use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
    sprite::MaterialMesh2dBundle,
    window::{PresentMode, WindowMode},
};

mod plugins;
use plugins::{map_plugin, movement_plugin, ui_plugin};

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
        .add_startup_system(load_continent_polygons)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(map_plugin::MapPlugin)
        .add_plugin(movement_plugin::MovementPlugin)
        .add_plugin(ui_plugin::UIPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn load_continent_polygons(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut vandagar_mesh = Mesh::new(PrimitiveTopology::LineStrip);
    vandagar_mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![[100.0, 0.0, 1.0], [0.0, 100.0, 1.0]],
    );

    vandagar_mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, vec![[0.0, 0.0, 0.0, 1.0]; 2]);
    vandagar_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 1.0, 0.0]; 2]);
    vandagar_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0.0, 0.0]; 2]);

    vandagar_mesh.set_indices(Some(Indices::U32(vec![0, 1])));

    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(vandagar_mesh).into(),
            transform: Transform::default(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            ..default()
        })
        .insert(movement_plugin::Movable)
        .insert(movement_plugin::Zoomable);
}
