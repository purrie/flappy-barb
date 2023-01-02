use bevy::prelude::*;

use crate::gravity::VerticalMove;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(make_player_sprite)
            .add_system(player_on_side)
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

fn player_on_side(mut player: Query<&mut Transform, With<Player>>, camera_view: Query<&OrthographicProjection>) {
    let mut player = player.get_single_mut().unwrap();
    let cam = camera_view.get_single().unwrap();
    player.translation.x = cam.left + 256.;
}
