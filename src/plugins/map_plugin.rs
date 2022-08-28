use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
    transform::components::Transform,
};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(map_scrolling)
            .add_system(grab_mouse);
    }
}

#[derive(Component)]
struct MapEntity;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle {
        transform: Transform::from_xyz(0., 1., 0.),
        ..default()
    });
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("images/map.png"),
            transform: Transform {
                scale: Vec3::splat(0.5),
                ..default()
            },
            sprite: Sprite {
                // Flip the logo to the left
                flip_x: false,
                // And don't flip it upside-down ( the default )
                flip_y: false,
                ..default()
            },
            ..default()
        })
        .insert(MapEntity);
}

// This system grabs the mouse when the left mouse button is pressed
fn grab_mouse(mut windows: ResMut<Windows>, mouse: Res<Input<MouseButton>>) {
    let window = windows.get_primary_mut().unwrap();
    if mouse.just_pressed(MouseButton::Left) {
        window.set_cursor_visibility(false);
        window.set_cursor_lock_mode(true);
    }
    if mouse.just_released(MouseButton::Left) {
        window.set_cursor_visibility(true);
        window.set_cursor_lock_mode(false);
    }
}

fn map_scrolling(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query: Query<&mut Transform, With<MapEntity>>,
) {
    // for event in mouse_motion_events.iter() {
    //     info!("{:?}", event);
    // }

    for mut map in &mut query {
        for scroll in mouse_wheel_events.iter() {
            map.scale = Vec3::splat(f32::max(scroll.y as f32 + map.scale.max_element(), 0.5));
            println!("{:?}", map)
        }
    }
}
