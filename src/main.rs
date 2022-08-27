use bevy::{
    ecs::system::Commands,
    prelude::*,
    window::{PresentMode, WindowMode},
    winit::WinitSettings,
};

fn main() {
    /*
     * Entities are things
     * Components can be assigned as a group to things
     * Systems process components
     *
     * Resources represent state, or globally unique data
     *
     * Get entities by Query<&Component, With<Component>
     * and get values of resources by referencing Res for reading and ResMut for writing
     */
    App::new()
        .insert_resource(WindowDescriptor {
            title: "I am a window!".to_string(),
            present_mode: PresentMode::AutoVsync,
            mode: WindowMode::BorderlessFullscreen,
            ..default()
        })
        .insert_resource(WinitSettings::desktop_app())
        .insert_resource(CursorState::default())
        .add_startup_system(setup)
        .add_system_to_stage(CoreStage::PreUpdate, change_cursor_state)
        .add_system_to_stage(CoreStage::Update, mark_moving)
        .add_system_to_stage(CoreStage::PostUpdate, moving)
        .add_system_to_stage(CoreStage::PostUpdate, still)
        .add_plugins(DefaultPlugins)
        .run();
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
    commands.spawn_bundle(Camera2dBundle::default());
    commands
        .spawn()
        .insert_bundle((Transform::default(), GlobalTransform::default(), Cursor));
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("images/map.png"),
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

        cursor_state.position = mouse_motion.position // cursor_to_screen(window, cam_transform, mouse_motion.position);
    }
}

/*
fn cursor_to_screen(window: &Window, cam_transform: &Transform, cursor_pos: Vec2) -> Vec2 {
    let window_size = Vec2::new(window.width() as f32, window.height() as f32);
    let screen_pos = cursor_pos - window_size / 2.;
    let out = cam_transform.compute_matrix() * screen_pos.extend(0.0).extend(1.0);

    Vec2::new(out.x, out.y)
}*/

fn mark_moving(
    mut commands: Commands,
    cursor_state: Res<CursorState>,
    q_movable: Query<Entity, With<Movable>>,
    q_moving: Query<Entity, With<Moving>>,
) {
    // Adds or removes a component that marks an entity for moving.
    if cursor_state.holding {
        // If you're holding then add the "Moving" component to the entity
        if let Some(entity) = q_movable.iter().next() {
            commands.entity(entity).remove::<Still>();
            commands.entity(entity).insert(Moving);
        }
    } else {
        // if it isn't remove the "Moving" component from the entity
        if let Some(entity) = q_moving.iter().next() {
            commands.entity(entity).remove::<Moving>();
            commands.entity(entity).insert(Still);
        }
    }
}

fn moving(
    mut commands: Commands,
    mut q_moving: Query<(Entity, &mut Transform, &GlobalTransform), With<Moving>>,
    q_cursor: Query<(Entity, &GlobalTransform), With<Cursor>>,
) {
    if let Some((cursor_e, cursor_transform)) = q_cursor.iter().next() {
        for (entity, mut transform, global_transform) in q_moving.iter_mut() {
            println!("{:?}", transform);
            let global_pos = global_transform.translation() - cursor_transform.translation();
            println!("{:?}", global_pos);

            //commands.entity(cursor_e).push_children(&[entity]);
            transform.translation.x = global_pos.x;
            transform.translation.y = global_pos.y;
        }
    }
}

fn still(mut commands: Commands, mut q_still: Query<(Entity, &Parent), Added<Still>>) {
    for (entity, parent) in q_still.iter_mut() {
        println!("{:?}", &entity);
        commands.entity(**parent).remove_children(&[entity]);
        commands.entity(entity).remove::<Still>();
    }
}
