use bevy::prelude::*;

use crate::{physics::CollisionEvent, player::AttackState};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        let play_update =
            SystemSet::on_update(GameState::Playing).with_system(kill_player.after("collision"));

        app.add_state(GameState::MainMenu)
            .add_startup_system(make_camera)
            .add_system_set(play_update);
    }
}

#[derive(PartialEq, Hash, Debug, Eq, Clone)]
pub enum GameState {
    Playing,
    End,
    MainMenu,
}

fn make_camera(mut cmd: Commands) {
    cmd.spawn(Camera2dBundle::default());
}

fn kill_player(mut ev: EventReader<CollisionEvent>, mut end: ResMut<State<GameState>>) {
    ev.iter()
        .filter(|x| x.player_state == AttackState::NotAttacking)
        .filter(|x| {
            let dis = x.player_pos.distance(x.obstacle_pos);
            dis < 100.0
        })
        .for_each(|_| end.set(GameState::End).unwrap());
}
