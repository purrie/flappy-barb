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
    let asset_folder = if cfg!(feature = "local_unix_assets") {
        let home = match std::env::var("HOME") {
            Ok(o) => o,
            Err(_) => panic!("Couldn't obtain HOME directory"),
        };
        format!("{}/.local/share/flappy-barb/", home)
    } else if cfg!(feature = "unix_assets") {
        String::from("/usr/share/flappy-barb/")
    } else {
        String::from("assets/")
    };
    let window = if cfg!(target_arch = "wasm32") {
        WindowDescriptor {
            width: 1280.0,
            height: 720.0,
            ..default()
        }
    } else {
        WindowDescriptor {
            title: String::from("Flappy Barb"),
            width: 1280.0,
            height: 720.0,
            ..default()
        }
    };
    let mut app = App::new();
    app
        // Engine Plugins
        .add_plugin(CorePlugin::default())
        .add_plugin(TimePlugin::default())
        .add_plugin(TransformPlugin::default())
        .add_plugin(HierarchyPlugin::default())
        .add_plugin(InputPlugin::default())
        .add_plugin(WindowPlugin {
            window,
            ..default()
        })
        .add_plugin(AssetPlugin {
            asset_folder,
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
        .add_plugin(ParticlePlugin);

    app.run();
}
