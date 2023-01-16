use bevy::prelude::*;

use crate::{
    cleanup::Dead,
    game::GameState,
    physics::{Gravity, Movement},
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        let start = SystemSet::on_enter(GameState::Playing).with_system(make_player_sprite);
        let update = SystemSet::on_update(GameState::Playing)
            .with_system(jump_system.after("gravity").before("movement"))
            .with_system(attack_state.before("collision"))
            .with_system(animate_player);
        let end = SystemSet::on_exit(GameState::Playing).with_system(player_dead);

        let cleanup =
            SystemSet::on_exit(GameState::End).with_system(clean_player.before("cleanup"));

        app.add_system_set(start)
            .add_system_set(update)
            .add_system_set(end)
            .add_system_set(cleanup);
    }
}

pub const PLAYER_JUMP_STRENGTH: f32 = 500.;

#[derive(Default, Clone, PartialEq)]
pub enum AttackState {
    #[default]
    NotAttacking,
    Swinging,
    SwingEnd,
}

#[derive(Component, Default)]
pub struct Player {
    pub attack_state: AttackState,
}

#[derive(Component)]
pub struct PlayerCorpse;

fn jump_system(input: Res<Input<KeyCode>>, mut player: Query<&mut Movement, With<Player>>) {
    if input.just_pressed(KeyCode::Space) {
        for mut mov in player.iter_mut() {
            mov.y = PLAYER_JUMP_STRENGTH;
        }
    }
}

fn attack_state(mut player: Query<(&mut Player, &Movement)>) {
    let mut pl = player.get_single_mut().unwrap();
    let threshhold = PLAYER_JUMP_STRENGTH / 2.;
    pl.0.attack_state = match pl.1.y {
        x if x > threshhold => AttackState::Swinging,
        x if x > 0. => AttackState::SwingEnd,
        _ => AttackState::NotAttacking,
    }
}

fn animate_player(mut player: Query<(&mut Sprite, &Player)>) {
    let mut pl = player.get_single_mut().unwrap();
    pl.0.color = match pl.1.attack_state {
        AttackState::NotAttacking => Color::Rgba {
            red: 1.,
            green: 1.,
            blue: 1.,
            alpha: 1.,
        },
        AttackState::Swinging => Color::Rgba {
            red: 1.,
            green: 1.,
            blue: 0.5,
            alpha: 1.,
        },
        AttackState::SwingEnd => Color::Rgba {
            red: 1.,
            green: 0.5,
            blue: 0.5,
            alpha: 1.,
        },
    }
}

fn make_player_sprite(
    mut commands: Commands,
    _asset_server: Res<AssetServer>,
    camera: Query<&OrthographicProjection>,
) {
    // let img = asset_server.load("sprites/testchar2.png");
    let camera = camera.single();
    commands.spawn((
        SpriteBundle {
            // texture: img,
            sprite: Sprite {
                custom_size: Some(Vec2 { x: 128., y: 128. }),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3 {
                    x: camera.left + 256.0,
                    y: (camera.top + camera.bottom) / 2.0,
                    ..default()
                },
                ..default()
            },
            ..default()
        },
        Movement {
            y: PLAYER_JUMP_STRENGTH,
            ..Default::default()
        },
        Gravity::default(),
        Player::default(),
    ));
}

fn player_dead(
    mut player: Query<(&mut Sprite, &mut Movement, Entity), With<Player>>,
    mut cmd: Commands,
) {
    let mut player = player.get_single_mut().unwrap();
    player.0.color = Color::GREEN;
    player.1.y = PLAYER_JUMP_STRENGTH;

    cmd.entity(player.2).remove::<Player>().insert(PlayerCorpse);
}

fn clean_player(mut cmd: Commands, player: Query<Entity, With<PlayerCorpse>>) {
    player.for_each(|x| {
        cmd.entity(x).insert(Dead::default());
    })
}
