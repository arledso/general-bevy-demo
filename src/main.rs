//try to render something | check
//make it move | check
//detect input and move according to input | check
//detect collisions
//try to kill stuff (??)
//render stuff with vectors
mod kinematics;
mod movement;
mod physics;
mod config;

use config::{FRICTION_COEFFICIENT, GRAVITY_ACCELERATION};
use kinematics::*;
use movement::*;
use bevy::prelude::*;
use physics::*;

fn main() {
    App::new()

    .add_plugins(DefaultPlugins)
    .add_plugin(KinematicsPlugin)
    .add_plugin(MovementPlugin)
    .add_plugin(PhysicsPlugin)

    .add_startup_system(setup)

    .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                        color: Color::rgb(0.2, 0.2, 0.2),
                        custom_size: Some(Vec2::new(50.0, 100.0)),
                        ..default()
                    },
            transform: Transform {
                        translation: Vec3::new(0.0,0.0,0.0),
                        ..default()
                    },
            ..default()
        },

        Velocity(Vec3::new(0.0,0.0,0.0)),
        Acceleration(Vec3::new(0.0,0.0,0.0)),

        Friction(FRICTION_COEFFICIENT, true),
        Gravity(GRAVITY_ACCELERATION),
        
        Controllable,

    ));

    commands.spawn((
        Camera2dBundle {
            transform: Transform {
                            translation: Vec3::new(0.0,0.0,0.0),
                            ..default()
                        },
            ..default()
        } ,
    ));
}