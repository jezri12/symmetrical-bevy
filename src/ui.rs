use bevy::prelude::*;

use crate::Money;

pub struct GameUI;

impl Plugin for GameUI {
    fn build(&self, app: &mut App) {
	app
	    .add_systems(Startup, spawn_game_ui)
	    .add_systems(Update, update_money_ui);
    }
}

#[derive(Component)]
struct MoneyText;

fn spawn_game_ui(
    mut commands: Commands,
){
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(10.0),
                    align_items: AlignItems::Center,
		    justify_content: JustifyContent::Center,
                    padding: UiRect::all(Val::Px(10.0)),
		    margin: UiRect::top(Val::Percent(2.3)),
                    ..default()
                },
                background_color: Color::rgba_u8(44, 59, 85, 156).into(),
                ..default()
            },
            Name::new("UI Root"),
        ))
        .with_children(|commands| {
            commands.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Money!",
                        TextStyle {
                            font_size: 32.0,
                            ..default()
                        },
                    ),
                    ..default()
                },
                MoneyText,
            ));
        });
}

fn update_money_ui(
    mut texts: Query<&mut Text, With<MoneyText>>,
    money: Res<Money>
){
    for mut text in &mut texts {
	text.sections[0].value = format!("Money: {}", money.0);
    }
}
