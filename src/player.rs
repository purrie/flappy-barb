use bevy::prelude::*;

use crate::physics::{Gravity, Movement};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(make_player_sprite)
            .add_system(player_on_side)
            .add_system(jump_system.after("gravity").before("movement"))
            .add_system(attack_state.before("collision"))
            .add_system(animate_player);
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

fn make_player_sprite(mut commands: Commands, _asset_server: Res<AssetServer>) {
    // let img = asset_server.load("sprites/testchar2.png");
    commands.spawn((
        SpriteBundle {
            // texture: img,
            sprite: Sprite {
                custom_size: Some(Vec2 { x: 128., y: 128. }),
                ..Default::default()
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

fn player_on_side(
    mut player: Query<&mut Transform, With<Player>>,
    camera_view: Query<&OrthographicProjection>,
) {
    let mut player = player.get_single_mut().unwrap();
    let cam = camera_view.get_single().unwrap();
    player.translation.x = cam.left + 256.;
}
