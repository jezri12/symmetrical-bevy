
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy::render::camera::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use ui::GameUI;

use crate::pig::PigPlugin;

mod pig;
mod idle_movement;
mod ui;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Farming Sim!".into(),
                        resolution: (640.0, 480.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_plugins(
	    WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape))
	)
        .insert_resource(Money(100.0))
        .add_plugins((PigPlugin, GameUI))
        .register_type::<Player>()
        .add_systems(Startup, setup)
        .add_systems(Update, character_movement)
        .run();
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Player {
    pub speed: f32
}

#[derive(Resource)]
pub struct Money(pub f32);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin {
	min_width: 256.0,
	min_height: 144.0
    };

    camera.camera_2d.clear_color = ClearColorConfig::Custom(Color::OLIVE);

    commands.spawn(camera);

    let texture = asset_server.load("character.png");

    commands.spawn((
        SpriteBundle {
            texture,
            ..default()
        },
        Player { speed: 100.0 },
	Name::new("Player"),
    ));
}

fn character_movement(
    mut characters: Query<(&mut Transform, &Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut characters {
        let movement_amount: f32 = player.speed * time.delta_seconds();

        if input.pressed(KeyCode::W) {
            transform.translation.y += movement_amount;
        }
        if input.pressed(KeyCode::S) {
            transform.translation.y -= movement_amount;
        }
        if input.pressed(KeyCode::D) {
            transform.translation.x += movement_amount;
        }
        if input.pressed(KeyCode::A) {
            transform.translation.x -= movement_amount;  
        }
    }
}


