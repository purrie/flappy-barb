use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};

const SKY_COLOR: Color = Color::Hsla {
    hue: 200.0,
    saturation: 0.4,
    lightness: 0.7,
    alpha: 1.0,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        let play_update = SystemSet::on_update(GameState::Playing)
            .with_system(game_over.label("game_over").after("collision"));

        let menu_update =
            SystemSet::on_update(GameState::MainMenu).with_system(start_game_shortcut);

        let end_update = SystemSet::on_update(GameState::End).with_system(start_game_shortcut);

        app.add_state(GameState::MainMenu)
            .add_event::<GameOverEvent>()
            .add_startup_system(make_camera)
            .add_system_set(menu_update)
            .add_system_set(play_update)
            .add_system_set(end_update);
    }
}

#[derive(Default)]
pub struct GameOverEvent;

#[derive(PartialEq, Hash, Debug, Eq, Clone)]
pub enum GameState {
    Playing,
    End,
    MainMenu,
}

fn make_camera(mut cmd: Commands) {
    cmd.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(SKY_COLOR),
        },
        ..default()
    });
}

fn game_over(go: EventReader<GameOverEvent>, mut end: ResMut<State<GameState>>) {
    if go.is_empty() == false {
        if let Err(e) = end.set(GameState::End) {
            println!("Error: {e}");
        }
        go.clear();
    }
}

fn start_game_shortcut(input: Res<Input<KeyCode>>, mut state: ResMut<State<GameState>>) {
    if input.just_pressed(KeyCode::Return) {
        if let Err(e) = state.set(GameState::Playing) {
            println!("Error: {e}");
        }
    }
}
