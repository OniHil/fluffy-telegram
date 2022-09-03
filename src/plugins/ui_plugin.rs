use bevy::{prelude::*, ui::FocusPolicy};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(mark_menu_expand)
            .add_system(expand_retract_menu);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(MenuState::default());
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                padding: UiRect::all(Val::Px(1.)),
                min_size: Size::new(Val::Px(31.), Val::Px(31.)),
                max_size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                size: Size::new(Val::Px(31.), Val::Px(31.)),
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                overflow: Overflow::Hidden,
                ..default()
            },
            focus_policy: FocusPolicy::Block,
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    color: UiColor(Color::rgb(250., 250., 250.)),
                    style: Style {
                        size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(30.), Val::Px(30.)),
                            align_self: AlignSelf::FlexEnd,
                            position_type: PositionType::Absolute,
                            ..default()
                        },
                        image: UiImage(asset_server.load("menu/right_arrow.png")),
                        ..default()
                    });
                });
        })
        .insert(Menu);
}

#[derive(Component)]
struct Menu;

#[derive(Default)]
struct MenuState {
    expanded: bool,
}

fn mark_menu_expand(
    mut q_menu: Query<&Interaction, (Changed<Interaction>, With<Menu>)>,
    mut menu_state: ResMut<MenuState>,
) {
    for interaction in &mut q_menu {
        match *interaction {
            Interaction::Clicked => menu_state.expanded = !menu_state.expanded,
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

fn expand_retract_menu(
    mut q_menu: Query<&mut Style, With<Menu>>,
    //mut windows: ResMut<Windows>,
    menu_state: Res<MenuState>,
) {
    let window = windows.get_primary_mut().unwrap();
    for style in q_menu.iter_mut() {
        if menu_state.expanded {
            println!("{:?}", style.size);
            style.size.width = Val::Px(style.size.width - window.width() * 0.01);
            style.size.height = Val::Px(style.size.height - window.height() * 0.01);
        } else {
            println!("{:?}", style.size);
            style.size.width.add_assign(style.size.width);
            style.size.height.add_assign(style.size.add_assign(0.01));
        }
    }
}
