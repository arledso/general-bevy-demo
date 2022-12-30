use crate::kinematics::*;
use bevy::prelude::*;

//plugin

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {

    fn build(&self, app: &mut App) {
        app
        
        .add_system_to_stage(CoreStage::Update, apply_gravity)
        .add_system_to_stage(CoreStage::Update, apply_friction);
    }
}

//components
#[derive(Component)]
pub struct Friction(pub f32, pub bool);
#[derive(Component)]
pub struct Gravity(pub f32);


//systems

fn apply_gravity(mut query: Query<(&mut Acceleration, &Gravity)>, time: Res<Time>) {
    for (mut acceleration, gravity) in &mut query {
        acceleration.0.y -= gravity.0 * time.delta_seconds();
    }
}

fn apply_friction(mut query: Query<(&mut Velocity, &Friction)>, time: Res<Time>) {
    for (mut velocity, friction) in &mut query {
        if !friction.1 { continue;}
        velocity.0.x -= velocity.0.x * friction.0 * time.delta_seconds();
    }
}

//add collisions next
//try if position.in(object) then set position to propper position given vel, and set vel.y = 0.