use bevy::{prelude::*, ui::FocusPolicy};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(mark_menu_expand_retract)
            .add_system(expand_menu.after(mark_menu_expand_retract))
            .add_system(retract_menu.after(mark_menu_expand_retract));
    }
}
const MENU_ICON_SIZE: f32 = 32.;
const MAX_MENU_WIDTH: f32 = 400.;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(MenuState::default());
    commands
        .spawn_bundle(NodeBundle {
            color: UiColor(Color::rgba(0., 0., 0., 0.)),
            transform: Transform {
                translation: Vec3::new(-MAX_MENU_WIDTH, 0.0, 10.0),
                ..default()
            },
            style: Style {
                padding: UiRect::all(Val::Px(1.)),
                size: Size::new(Val::Px(MAX_MENU_WIDTH), Val::Percent(100.)),
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                overflow: Overflow::Hidden,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(NodeBundle {
                color: UiColor(Color::rgb(200., 200., 200.)),
                style: Style {
                    max_size: Size::new(
                        Val::Px(MAX_MENU_WIDTH - MENU_ICON_SIZE),
                        Val::Percent(100.),
                    ),
                    size: Size::new(Val::Px(MAX_MENU_WIDTH - MENU_ICON_SIZE), Val::Percent(100.)),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ..default()
            });
            parent.spawn_bundle(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(MENU_ICON_SIZE), Val::Px(MENU_ICON_SIZE)),
                    align_self: AlignSelf::FlexEnd,
                    ..default()
                },
                image: UiImage(asset_server.load("menu/right-arrow.png")),
                ..default()
            });
        })
        .insert(Menu)
        .insert(Retract);
}

#[derive(Component)]
struct Menu;
#[derive(Component)]
struct Expand;
#[derive(Component)]
struct Retract;

#[derive(Default)]
struct MenuState {
    expanded: bool,
}

fn mark_menu_expand_retract(
    mut commands: Commands,
    q_button: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    mut q_menu: Query<Entity, With<Menu>>,
    mut menu_state: ResMut<MenuState>,
) {
    if let Some(interaction) = q_button.iter().last() {
        for menu_entity in &mut q_menu {
            match *interaction {
                Interaction::Clicked => {
                    if menu_state.expanded {
                        commands.entity(menu_entity).insert(Retract);
                    } else {
                        commands.entity(menu_entity).insert(Expand);
                    }
                    menu_state.expanded = !menu_state.expanded;
                }
                Interaction::Hovered => {}
                Interaction::None => {}
            }
        }
    }
}

fn expand_menu(
    mut commands: Commands,
    mut q_menu: Query<(Entity, &mut Transform), (With<Menu>, Added<Expand>)>,
) {
    for (menu_entity, mut transform) in q_menu.iter_mut() {
        transform.translation.x = 0.;
        println!("{:?}", transform);

        commands.entity(menu_entity).remove::<Expand>();
    }
}

fn retract_menu(
    mut commands: Commands,
    mut q_menu: Query<(Entity, &mut Transform), (With<Menu>, Added<Retract>)>,
) {
    for (menu_entity, mut transform) in q_menu.iter_mut() {
        transform.translation.x = -MAX_MENU_WIDTH;
        println!("{:?}", transform);

        commands.entity(menu_entity).remove::<Retract>();
    }
}
