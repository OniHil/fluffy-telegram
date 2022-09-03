use bevy::{ecs::system::Commands, input::mouse::MouseWheel, prelude::*};
/// Everything conserning movement that is controlled by the mouse should be inside this file.
/// Everything that uses this plugin will zoom and move at the same rate and in sync.
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(change_cursor_state)
            .add_system(transform_cursor)
            .add_system(mark_moving_status)
            .add_system(moving)
            .add_system(still)
            .add_system(zooming);
    }
}

// Components
/// If you want something to be zoomable, then insert this component.
#[derive(Component)]
pub struct Zoomable;
/// If it's movable (with the mouse) then add this one.
#[derive(Component)]
pub struct Movable;
#[derive(Component)]
struct Moving;
#[derive(Component)]
struct Still;
#[derive(Component)]
struct Cursor;

// Resources
#[derive(Default)]
struct CursorState {
    position: Vec2,
    holding: bool,
}

fn setup(mut commands: Commands) {
    commands
        .spawn()
        .insert_bundle((Transform::default(), GlobalTransform::default(), Cursor));
    commands.insert_resource(CursorState::default());
}

/// Updates the cursor state
fn change_cursor_state(
    mut e_mouse_motion: EventReader<CursorMoved>,
    i_mouse_button: Res<Input<MouseButton>>,
    mut cursor_state: ResMut<CursorState>,
    windows: Res<Windows>,
    q_camera: Query<&Transform, With<Camera>>,
) {
    if i_mouse_button.just_pressed(MouseButton::Left) {
        cursor_state.holding = true;
    } else if i_mouse_button.just_released(MouseButton::Left) {
        cursor_state.holding = false;
    }

    for mouse_motion in e_mouse_motion.iter() {
        let window = windows.get_primary().unwrap();
        let cam_transform = q_camera.iter().last().unwrap();

        cursor_state.position = cursor_to_screen(window, cam_transform, mouse_motion.position);
    }
}

/// Relative position of the cursor, (0, 0) is the middle
fn cursor_to_screen(window: &Window, cam_transform: &Transform, cursor_pos: Vec2) -> Vec2 {
    let window_size = Vec2::new(window.width() as f32, window.height() as f32);
    let screen_pos = cursor_pos - window_size / 2.;
    let out = cam_transform.compute_matrix() * screen_pos.extend(0.0).extend(1.0);

    Vec2::new(out.x, out.y)
}

/// The Cursor component needs to be in sync with the cursor.
fn transform_cursor(
    cursor_state: ResMut<CursorState>,
    mut q_cursor: Query<&mut Transform, With<Cursor>>,
) {
    for mut transform in q_cursor.iter_mut() {
        transform.translation.x = cursor_state.position.x;
        transform.translation.y = cursor_state.position.y;
    }
}

/// We mark the entities that are going to be moving or were just dropped
fn mark_moving_status(
    mut commands: Commands,
    cursor_state: Res<CursorState>,
    q_movable: Query<Entity, With<Movable>>,
    q_moving: Query<Entity, With<Moving>>,
) {
    if cursor_state.holding {
        for movable_entity in q_movable.iter() {
            commands.entity(movable_entity).insert(Moving);
        }
    } else {
        for movable_entity in q_moving.iter() {
            commands.entity(movable_entity).remove::<Moving>();
            commands.entity(movable_entity).insert(Still);
        }
    }
}

/// If something is moving, sync it with the cursor (as a parent)
fn moving(
    mut commands: Commands,
    mut q_moving: Query<(Entity, &mut Transform, &GlobalTransform), Added<Moving>>,
    q_cursor: Query<(Entity, &GlobalTransform), With<Cursor>>,
) {
    if let Some((cursor_entity, cursor_transform)) = q_cursor.iter().next() {
        for (moving_entity, mut transform, global_transform) in q_moving.iter_mut() {
            let global_pos = global_transform.translation() - cursor_transform.translation();

            commands.entity(cursor_entity).add_child(moving_entity);

            transform.translation.x = global_pos.x;
            transform.translation.y = global_pos.y;
        }
    }
}

/// If we dropped it, then since we want them to be still, remove cursor as a parent and mark it as still
fn still(
    mut commands: Commands,
    mut q_still: Query<(Entity, &Parent), (Added<Still>, With<Parent>)>,
) {
    for (entity, parent) in q_still.iter_mut() {
        commands.entity(entity).remove::<Still>();
        commands.entity(**parent).remove_children(&[entity]);
    }
}

/// If we want to zoom in or out we do that here
/// TODO: Find a better way to zoom, do it logrithimcally / exponentially maybe?
fn zooming(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query: Query<&mut Transform, With<Zoomable>>,
) {
    for mut map in &mut query {
        for scroll in mouse_wheel_events.iter() {
            map.scale = Vec3::splat(f32::max(scroll.y as f32 + map.scale.max_element(), 0.5));
            println!("{:?}", map)
        }
    }
}
