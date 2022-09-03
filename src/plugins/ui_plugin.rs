use bevy::{prelude::*, ui::FocusPolicy};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(mark_hamburger_expanded)
            .add_system(expand_retract_hamburger);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(HamburgerState::default());
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
                        image: UiImage(asset_server.load("menu/hamburger.png")),
                        ..default()
                    });
                });
        })
        .insert(HamburgerMenu);
}

#[derive(Component)]
struct HamburgerMenu;

#[derive(Default)]
struct HamburgerState {
    expanded: bool,
}

fn mark_hamburger_expanded(
    mut q_hamburger: Query<&Interaction, (Changed<Interaction>, With<HamburgerMenu>)>,
    mut hamburger_state: ResMut<HamburgerState>,
) {
    for interaction in &mut q_hamburger {
        match *interaction {
            Interaction::Clicked => hamburger_state.expanded = !hamburger_state.expanded,
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

fn expand_retract_hamburger(
    mut q_hamburger: Query<&mut Style, With<HamburgerMenu>>,
    //mut windows: ResMut<Windows>,
    hamburger_state: Res<HamburgerState>,
) {
    //let window = windows.get_primary_mut().unwrap();
    for style in q_hamburger.iter_mut() {
        if hamburger_state.expanded {
            println!("{:?}", style.size);
            // style.size.width = Val::Px(style.size.width - window.width() * 0.01);
            // style.size.height = Val::Px(style.size.height - window.height() * 0.01);
        } else {
            println!("{:?}", style.size);
            // style.size.width.add_assign(style.size.width);
            // style.size.height.add_assign(style.size.add_assign(0.01));
        }
    }
}
