use bevy::prelude::*;

pub struct IdleMovementPlugin;

impl Plugin for IdleMovementPlugin {
    fn build(&self, app: &mut App) {
	app.add_systems(Update, idle_movement_logic);
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct IdleMovement {
    pub speed: Vec2,
    pub change_dir_x: Timer,
    pub change_dir_y: Timer,
    pub right: bool,
    pub down: bool,
}

fn idle_movement_logic(
    time: Res<Time>,
    mut things: Query<(&mut Transform, &mut IdleMovement)> ,
){
    for (mut transform, mut idle_movement)  in &mut things {
	let movement_amount = idle_movement.speed * time.delta_seconds();

	if idle_movement.right {
	    transform.translation.x += movement_amount.x;
	} else {
	    transform.translation.x -= movement_amount.x;
	}

	if idle_movement.down {
	    transform.translation.y += movement_amount.y;
	} else {
	    transform.translation.y -= movement_amount.y;
	}

	idle_movement.change_dir_x.tick(time.delta());
	idle_movement.change_dir_y.tick(time.delta());

	if idle_movement.change_dir_x.just_finished() {
	    idle_movement.right = !idle_movement.right;
	}
	if idle_movement.change_dir_y.just_finished() {
	    idle_movement.down = !idle_movement.down;
	}
    }
}

