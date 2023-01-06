mod cleanup;
mod gravity;
mod obstacles;
mod physics;
mod player;

use bevy::prelude::*;
use cleanup::CleanerPlugin;
use gravity::GravityPlugin;
use obstacles::ObstaclesPlugin;
use physics::PhysicsPlugin;
use player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(GravityPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(ObstaclesPlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugin(CleanerPlugin)
        .add_startup_system(make_camera)
        .run();
}

fn make_camera(mut cmd: Commands) {
    cmd.spawn(Camera2dBundle::default());
}
