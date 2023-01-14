use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};

use crate::{physics::CollisionEvent, player::AttackState};

const SKY_COLOR: Color = Color::Hsla {
    hue: 200.0,
    saturation: 0.4,
    lightness: 0.7,
    alpha: 1.0,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        let play_update =
            SystemSet::on_update(GameState::Playing).with_system(kill_player.after("collision"));

        let menu_update =
            SystemSet::on_update(GameState::MainMenu).with_system(start_game_shortcut);

        let end_update = SystemSet::on_update(GameState::End).with_system(start_game_shortcut);

        app.add_state(GameState::MainMenu)
            .add_startup_system(make_camera)
            .add_system_set(menu_update)
            .add_system_set(play_update)
            .add_system_set(end_update);
    }
}

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

fn kill_player(mut ev: EventReader<CollisionEvent>, mut end: ResMut<State<GameState>>) {
    ev.iter()
        .filter(|x| x.player_state == AttackState::NotAttacking)
        .filter(|x| {
            let dis = x.player_pos.distance(x.obstacle_pos);
            dis < 100.0
        })
        .for_each(|_| {
            if let Err(e) = end.set(GameState::End) {
                println!("Error: {e}");
            }
        });
}

fn start_game_shortcut(input: Res<Input<KeyCode>>, mut state: ResMut<State<GameState>>) {
    if input.just_pressed(KeyCode::Return) {
        if let Err(e) = state.set(GameState::Playing) {
            println!("Error: {e}");
        }
    }
}
