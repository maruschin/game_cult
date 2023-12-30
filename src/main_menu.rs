use crate::prelude::*;
use bevy::ui::node_bundles::NodeBundle;

#[derive(Default, Resource)]
struct MenuState;

#[derive(Component)]
pub struct Button;

pub struct MainMenuPlugin;

fn add_button(
    commands: &mut Commands<'_, '_>,
    asset_server: &mut ResMut<AssetServer>,
    text: String,
) -> Entity {
    let parent = commands
        .spawn((
            Button,
            NodeBundle {
                style: Style {
                    width: Val::Percent(19.8),
                    height: Val::Percent(6.),
                    border: UiRect::all(Val::Percent(0.5)),
                    margin: UiRect::all(Val::Percent(0.1)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: Color::rgba(0.15, 0.15, 0.15, 0.9).into(),
                ..default()
            },
        ))
        .id();
    let child = commands
        .spawn(
            TextBundle::from_section(
                text,
                TextStyle {
                    font: asset_server.load("fonts/droid_sans/DroidSans.ttf"),
                    font_size: 40.,
                    ..default()
                },
            )
            .with_style(Style {
                margin: UiRect::all(Val::Px(5.)),
                ..default()
            }),
        )
        .id();
    commands.entity(parent).push_children(&[child]);
    parent
}

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MenuState>().add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands, mut asset_server: ResMut<AssetServer>) {
    let parent = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .id();

    let childrens = &[
        add_button(&mut commands, &mut asset_server, "New Game".to_string()),
        add_button(&mut commands, &mut asset_server, "Load Game".to_string()),
        add_button(&mut commands, &mut asset_server, "Save Game".to_string()),
        add_button(&mut commands, &mut asset_server, "Settings".to_string()),
        add_button(&mut commands, &mut asset_server, "Exit".to_string()),
    ];

    commands.entity(parent).push_children(childrens);
}
