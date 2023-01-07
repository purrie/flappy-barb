mod cleanup;
mod obstacles;
mod physics;
mod player;

use bevy::{
    audio::AudioPlugin, core_pipeline::CorePipelinePlugin, input::InputPlugin, prelude::*,
    render::RenderPlugin, sprite::SpritePlugin, text::TextPlugin, time::TimePlugin, ui::UiPlugin,
    winit::WinitPlugin,
};
use cleanup::CleanerPlugin;
use obstacles::ObstaclesPlugin;
use physics::PhysicsPlugin;
use player::PlayerPlugin;

fn main() {
    App::new()
        // Engine Plugins
        .add_plugin(CorePlugin::default())
        .add_plugin(TimePlugin::default())
        .add_plugin(TransformPlugin::default())
        .add_plugin(HierarchyPlugin::default())
        .add_plugin(InputPlugin::default())
        .add_plugin(WindowPlugin::default())
        .add_plugin(AssetPlugin {
            ..Default::default()
        })
        .add_plugin(WinitPlugin::default())
        .add_plugin(RenderPlugin::default())
        .add_plugin(ImagePlugin::default())
        .add_plugin(CorePipelinePlugin::default())
        .add_plugin(SpritePlugin::default())
        .add_plugin(TextPlugin::default())
        .add_plugin(UiPlugin::default())
        .add_plugin(AudioPlugin::default())
        // Game Plugins
        .add_startup_system(make_camera)
        .add_plugin(PlayerPlugin)
        .add_plugin(ObstaclesPlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugin(CleanerPlugin)
        .run();
}

fn make_camera(mut cmd: Commands) {
    cmd.spawn(Camera2dBundle::default());
}
