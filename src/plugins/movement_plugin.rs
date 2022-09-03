use bevy::{ecs::system::Commands, input::mouse::MouseWheel, prelude::*};
/// Everything conserning movement that is controlled by the mouse should be inside this file.
/// Everything that uses this plugin will zoom and move at the same rate and in sync.
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system_to_stage(CoreStage::PreUpdate, change_cursor_state)
            .add_system_to_stage(CoreStage::PreUpdate, transform_cursor)
            .add_system(mark_moving_status)
            .add_system(moving.after(mark_moving_status))
            .add_system_to_stage(CoreStage::Update, zooming)
            .add_system_to_stage(CoreStage::PostUpdate, still);
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
    if let Some((cursor_entity, cursor_global)) = q_cursor.iter().next() {
        for (moving_entity, mut moving_transform, moving_global) in q_moving.iter_mut() {
            let global_pos = moving_global.translation() - cursor_global.translation();

            commands.entity(cursor_entity).add_child(moving_entity);

            moving_transform.translation.x = global_pos.x;
            moving_transform.translation.y = global_pos.y;
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
    mut q_zoomable: Query<&mut Transform, With<Zoomable>>,
    cursor_state: Res<CursorState>,
) {
    for mut zoomable_transform in q_zoomable.iter_mut() {
        for scroll in mouse_wheel_events.iter() {
            let mut scroll_factor = 0.8;
            let old_scroll = zoomable_transform.scale;

            // First set the new scale
            if scroll.y > 0.0 {
                scroll_factor = 1. / scroll_factor;
                zoomable_transform.scale = Vec3::splat(f32::min(
                    zoomable_transform.scale.max_element() * scroll_factor,
                    4.,
                ));
            } else if scroll.y < 0.0 {
                zoomable_transform.scale = Vec3::splat(f32::max(
                    zoomable_transform.scale.max_element() * scroll_factor,
                    0.2,
                ));
            }

            // Do nothing if scale wasn't changed
            if old_scroll == zoomable_transform.scale {
                return;
            }

            // Since we zoom in to where the cursor is, move the zooming entities based on where it is
            let cursor_delta_x =
                (cursor_state.position.x - zoomable_transform.translation.x) * (scroll_factor - 1.);
            let cursor_delta_y =
                (cursor_state.position.y - zoomable_transform.translation.y) * (scroll_factor - 1.);

            zoomable_transform.translation.x = zoomable_transform.translation.x - cursor_delta_x;
            zoomable_transform.translation.y = zoomable_transform.translation.y - cursor_delta_y;
        }
    }
}
