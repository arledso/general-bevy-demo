use crate::{kinematics::*, physics::*, config::*};
use bevy::prelude::*;

//plugin
pub struct MovementPlugin;

impl Plugin for MovementPlugin {

    fn build(&self, app: &mut App) {
        app

        .insert_resource(JumpDuration(Timer::from_seconds(JUMP_DURATION, TimerMode::Once)))

        .add_system_to_stage(CoreStage::PreUpdate, walk_on_input)
        .add_system_to_stage(CoreStage::PreUpdate, jump_on_input);
    }
}

//resources
#[derive(Resource)]
struct JumpDuration(Timer);

//components
#[derive(Component)]
pub struct Controllable;

//systems

fn walk_on_input(
        mut query: Query<(&mut Acceleration, &mut Friction, &Velocity), With<Controllable>>,
        time: Res<Time>,
        input: Res<Input<KeyCode>>, 
    ) 
    {
    for (mut acceleration, mut friction, velocity) in &mut query {
        let mut acceleration_mod: f32 = (velocity.0.x/WALK_VELOCITY - 1.0).abs();
        friction.1 = true;

        if input.pressed(KeyCode::D) && velocity.0.x <= WALK_VELOCITY {
            
            if velocity.0.x > 0.0 { //if moving in right direction
                friction.1 = false;
            }
            
            else {acceleration_mod = 1.0;}

            acceleration.0.x += WALK_ACCELERATION * time.delta_seconds() * acceleration_mod;

            println!("walking: right");
        }

        if input.pressed(KeyCode::A) && velocity.0.x >= -WALK_VELOCITY {
            
            if velocity.0.x < 0.0 { // if moving in right dir
                friction.1 = false;
            }
            
            else {acceleration_mod = 1.0;}
            
            acceleration.0.x -= WALK_ACCELERATION * time.delta_seconds() * acceleration_mod;

            println!("walking: left");
        }
    }
}

fn jump_on_input(
        mut query: Query<&mut Velocity, With<Controllable>>, 
        mut jump_duration: ResMut<JumpDuration>,
        input: Res<Input<KeyCode>>,
        time: Res<Time>,
    )
    {
        for mut velocity in &mut query {
            
            if velocity.0.y <= 0.0 { //reset duration
                jump_duration.0.reset();
            }

            if jump_duration.0.tick(time.delta()).finished() { //timer wont properly reset after trigger
                println!("Timer finished");
                continue;
            }  

            if input.pressed(KeyCode::Space) {
                velocity.0.y = JUMP_VELOCITY;
            }

            if input.just_released(KeyCode::Space) {
                jump_duration.0.set_elapsed(std::time::Duration::from_secs(u64::MAX));
            }
        }
}