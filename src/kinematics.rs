use bevy::prelude::*;

//plugin
pub struct KinematicsPlugin;

impl Plugin for KinematicsPlugin {

    fn build(&self, app: &mut App) {
        app

        .add_system_to_stage(CoreStage::PostUpdate, apply_acceleration)
        .add_system_to_stage(CoreStage::PostUpdate, apply_velocity.after(apply_acceleration))
        .add_system_to_stage(CoreStage::PostUpdate, reset_acceleration.after(apply_acceleration));
    }
}

//components
#[derive(Component)]
pub struct Velocity(pub Vec3);

#[derive(Component)]
pub struct Acceleration(pub Vec3);

//systems

//update parameters
fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation += velocity.0 * time.delta_seconds();
    }
}

fn apply_acceleration(mut query: Query<(&mut Velocity, &Acceleration)>, time: Res<Time>) {
    for (mut velocity, acceleration) in &mut query {
        velocity.0 += acceleration.0 * time.delta_seconds();
    }
}

fn reset_acceleration(mut query: Query<&mut Acceleration>) {
    for mut acceleration in &mut query {
        acceleration.0 = Vec3::new(0.0,0.0,0.0);
    }
}