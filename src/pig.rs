
use bevy::prelude::*;
use rand::Rng;

use crate::Player;
use crate::Money;
use crate::idle_movement::*;

pub struct PigPlugin;

impl Plugin for PigPlugin {
    fn build(&self, app: &mut App) {
	app
	    .add_systems(Update, (spawn_pig, pig_lifetime))
	    .add_systems(Startup, spawn_pig_parent)
	    .add_plugins(IdleMovementPlugin)
	    .register_type::<Pig>();
    }
    
}

#[derive(Component, Default, Reflect)]
pub struct PigParent;

fn spawn_pig_parent(
    mut commands: Commands
){
    commands.spawn((
	SpatialBundle::default(),
	PigParent,
	Name::new("Pig Parent")
    ));
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Pig {
    pub lifetime: Timer,
}

fn spawn_pig(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    mut money: ResMut<Money>,
    player: Query<&Transform, With<Player>>,
    parent: Query<Entity, With<PigParent>>,
){
    if !input.just_pressed(KeyCode::Space) {
	return;
    }

    let player_transform = player.single();
    let parent = parent.single();

    if money.0 >= 10.0 {
	money.0 -= 10.0;
	info!("Spent 10 bucks on a pig, {:?} remaining", money.0);

	let texture = asset_server.load("pig.png");
	let pig_direction_x = rand::thread_rng().gen_range(1..=10) >= 5;
	let pig_direction_y = rand::thread_rng().gen_range(1..=10) >= 5;
	commands.entity(parent).with_children(|commands| {
	    commands.spawn((
		SpriteBundle {
		    texture,
		    transform: *player_transform,
		    ..default()
		},
		Pig {
		    lifetime: Timer::from_seconds(6.0, TimerMode::Once),
		},
		IdleMovement {
		    change_dir_x: Timer::from_seconds(2.0, TimerMode::Repeating),
		    change_dir_y: Timer::from_seconds(3.0, TimerMode::Repeating),
		    speed: Vec2::new(10.0, 2.0),
		    right: pig_direction_x,
		    down: pig_direction_y,
		},
		Name::new("Pig")
	    ));
	});
	
    }
}

fn pig_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut pigs: Query<(Entity, &mut Pig)>,
    parent: Query<Entity, With<PigParent>>,
    mut money: ResMut<Money>,
){
    let parent = parent.single();
    for (pig_entity, mut pig) in &mut pigs {
	
	pig.lifetime.tick(time.delta());
	
	if pig.lifetime.finished() {    
	    money.0 += 15.0;

	    commands.entity(parent).remove_children(&[pig_entity]);
	    commands.entity(pig_entity).despawn();

	    info!("Pig sold for 15 buckaneers! You've got ${:?}", money.0);
	}
    }
}
