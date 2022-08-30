use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                padding: UiRect::all(Val::Percent(0.5)),
                ..default()
            },
            color: Color::BLACK.into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        padding: UiRect::all(Val::Percent(1.)),
                        size: Size::new(Val::Px(400.), Val::Px(200.)),
                        align_self: AlignSelf::FlexEnd,
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(ImageBundle {
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
        });
}
