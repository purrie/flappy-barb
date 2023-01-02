pub mod gravity;
pub mod obstacles;
pub mod player;

use bevy::prelude::*;
use gravity::GravityPlugin;
use obstacles::ObstaclesPlugin;
use player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(GravityPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(ObstaclesPlugin)
        .add_startup_system(make_camera)
        .run();
}

fn make_camera(mut cmd: Commands) {
    cmd.spawn(Camera2dBundle::default());
}
