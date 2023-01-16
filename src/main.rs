mod cleanup;
mod game;
mod obstacles;
mod particles;
mod physics;
mod player;
mod ui;

use bevy::{
    audio::AudioPlugin, core_pipeline::CorePipelinePlugin, input::InputPlugin, prelude::*,
    render::RenderPlugin, sprite::SpritePlugin, text::TextPlugin, time::TimePlugin, ui::UiPlugin,
    winit::WinitPlugin,
};
use cleanup::CleanerPlugin;
use game::GamePlugin;
use obstacles::ObstaclesPlugin;
use particles::ParticlePlugin;
use physics::PhysicsPlugin;
use player::PlayerPlugin;
use ui::GameUiPlugin;

fn main() {
    App::new()
        // Engine Plugins
        .add_plugin(CorePlugin::default())
        .add_plugin(TimePlugin::default())
        .add_plugin(TransformPlugin::default())
        .add_plugin(HierarchyPlugin::default())
        .add_plugin(InputPlugin::default())
        .add_plugin(WindowPlugin {
            window: WindowDescriptor {
                title: String::from("Flappy Barb"),
                width: 1280.0,
                height: 720.0,
                ..default()
            },
            ..default()
        })
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
        .add_plugin(GamePlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(ObstaclesPlugin)
        .add_plugin(GameUiPlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugin(CleanerPlugin)
        .add_plugin(ParticlePlugin)
        .run();
}
