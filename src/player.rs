use bevy::prelude::*;

use crate::gravity::VerticalMove;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(make_player_sprite)
            .add_system(jump_system);
    }
}

#[derive(Component)]
pub struct Player;

fn jump_system(input: Res<Input<KeyCode>>, mut player: Query<&mut VerticalMove, With<Player>>) {
    if input.just_pressed(KeyCode::Space) {
        for mut mov in player.iter_mut() {
            mov.speed = VerticalMove::JUMP_STRENGTH;
        }
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
        VerticalMove { speed: 1. },
        Player,
    ));
}
