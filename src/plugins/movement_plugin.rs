use bevy::{ecs::system::Commands, prelude::*};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(change_cursor_state)
            .add_system(transform_cursor)
            .add_system(mark_moving_status)
            .add_system(moving)
            .add_system(still);
    }
}

// Components
#[derive(Component)]
struct Movable;
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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn()
        .insert_bundle((Transform::default(), GlobalTransform::default(), Cursor));
    commands.insert_resource(CursorState::default());
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("map/map.png"),
            transform: Transform {
                scale: Vec3::splat(0.5),
                ..default()
            },
            sprite: Sprite {
                flip_x: false,
                flip_y: false,
                ..default()
            },
            ..default()
        })
        .insert(Movable);
}

fn change_cursor_state(
    mut e_mouse_motion: EventReader<CursorMoved>,
    i_mouse_button: Res<Input<MouseButton>>,
    mut cursor_state: ResMut<CursorState>,
    windows: Res<Windows>,
    q_camera: Query<&Transform, With<Camera>>,
) {
    // Makes sure that the CursorState resource is up-to-date.
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

fn cursor_to_screen(window: &Window, cam_transform: &Transform, cursor_pos: Vec2) -> Vec2 {
    let window_size = Vec2::new(window.width() as f32, window.height() as f32);
    let screen_pos = cursor_pos - window_size / 2.;
    let out = cam_transform.compute_matrix() * screen_pos.extend(0.0).extend(1.0);

    Vec2::new(out.x, out.y)
}

fn transform_cursor(
    cursor_state: ResMut<CursorState>,
    mut q_cursor: Query<&mut Transform, With<Cursor>>,
) {
    for mut transform in q_cursor.iter_mut() {
        transform.translation.x = cursor_state.position.x;
        transform.translation.y = cursor_state.position.y;
    }
}

fn mark_moving_status(
    mut commands: Commands,
    cursor_state: Res<CursorState>,
    q_movable: Query<Entity, With<Movable>>,
    q_moving: Query<Entity, With<Moving>>,
) {
    // Adds or removes a component that marks an entity for moving.
    if cursor_state.holding {
        // If you're holding then add the "Moving" component to the entity
        // because it's moving we add the cursor as a parent so we can sync thier movement
        if let Some(movable_entity) = q_movable.iter().next() {
            commands.entity(movable_entity).insert(Moving);
        }
    } else {
        if let Some(movable_entity) = q_moving.iter().next() {
            commands.entity(movable_entity).remove::<Moving>();
            commands.entity(movable_entity).insert(Still);
        }
    }
}

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

fn still(
    mut commands: Commands,
    mut q_still: Query<(Entity, &Parent), (Added<Still>, With<Parent>)>,
) {
    for (entity, parent) in q_still.iter_mut() {
        commands.entity(entity).remove::<Still>();
        commands.entity(**parent).remove_children(&[entity]);
    }
}
