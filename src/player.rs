use std::time::Duration;

use bevy::prelude::*;

use crate::{
    cleanup::Dead,
    game::{GameOverEvent, GameState, VIEW_BOX},
    particles::ParticleEmitter,
    physics::{Gravity, Movement, PlayerCollider},
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        let start = SystemSet::on_enter(GameState::Playing).with_system(make_player_sprite);
        let update = SystemSet::on_update(GameState::Playing)
            .with_system(jump_system.after("gravity").before("movement"))
            .with_system(attack_state.before("collision"))
            .with_system(player_out_of_bounds.after("movement"))
            .with_system(animate_player);
        let end = SystemSet::on_exit(GameState::Playing).with_system(player_dead);

        let cleanup =
            SystemSet::on_exit(GameState::End).with_system(clean_player.before("cleanup"));

        app.add_startup_system(load_assets)
            .add_system_set(start)
            .add_system_set(update)
            .add_system_set(end)
            .add_system_set(cleanup);
    }
}

pub const PLAYER_JUMP_STRENGTH: f32 = 500.;
const PLAYER_SIZE_X: f32 = 169.0;
const PLAYER_SIZE_Y: f32 = 169.0;

#[derive(Default, Clone, PartialEq)]
pub enum AttackState {
    #[default]
    NotAttacking,
    Swinging,
    SwingEnd,
}

#[derive(Resource)]
struct PlayerAssets {
    state_normal: Handle<Image>,
    state_swing: Handle<Image>,
    state_swing_end: Handle<Image>,
    state_dead: Handle<Image>,
    attack_sounds: Vec<Handle<AudioSource>>,
    death_sounds: Vec<Handle<AudioSource>>,
}

#[derive(Component, Default)]
pub struct Player {
    pub attack_state: AttackState,
}

#[derive(Component)]
pub struct PlayerCorpse;

fn load_assets(mut cmd: Commands, asset_server: Res<AssetServer>) {
    let mut attack_sounds = Vec::new();
    for i in 1..=19 {
        attack_sounds.push(asset_server.load(format!("audio/barb-attack-{}.ogg", i)));
    }
    let mut death_sounds = Vec::new();
    for i in 1..=8 {
        death_sounds.push(asset_server.load(format!("audio/barb-death-{}.ogg", i)));
    }
    let ass = PlayerAssets {
        state_normal: asset_server.load("sprites/barbarian-falling.png"),
        state_swing: asset_server.load("sprites/barbarian-midswing.png"),
        state_swing_end: asset_server.load("sprites/barbarian-chop.png"),
        state_dead: asset_server.load("sprites/barbarian-dead.png"),
        attack_sounds,
        death_sounds,
    };
    cmd.insert_resource(ass);
}

fn jump_system(
    input: Res<Input<KeyCode>>,
    mut player: Query<&mut Movement, With<Player>>,
    audio: Res<Audio>,
    assets: Res<PlayerAssets>,
) {
    if input.just_pressed(KeyCode::Space) {
        if let Ok(mut player) = player.get_single_mut() {
            player.y = PLAYER_JUMP_STRENGTH;
            play_attack_sound(&audio, &assets);
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

fn animate_player(mut cmd: Commands, player: Query<(Entity, &Player)>, assets: Res<PlayerAssets>) {
    let pl = player.single();
    let sprite = match pl.1.attack_state {
        AttackState::NotAttacking => assets.state_normal.clone(),
        AttackState::Swinging => assets.state_swing.clone(),
        AttackState::SwingEnd => assets.state_swing_end.clone(),
    };
    cmd.entity(pl.0).insert(sprite);
}

fn make_player_sprite(
    mut commands: Commands,
    _asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    assets: Res<PlayerAssets>,
) {
    commands.spawn((
        SpriteBundle {
            // texture: img,
            sprite: Sprite {
                custom_size: Some(Vec2 {
                    x: PLAYER_SIZE_X,
                    y: PLAYER_SIZE_Y,
                }),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3 {
                    x: VIEW_BOX.min.x + 256.0,
                    y: VIEW_BOX.min.y + VIEW_BOX.height() / 2.0,
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
        PlayerCollider {
            collision_size: Vec2 {
                x: PLAYER_SIZE_X * 0.9,
                y: PLAYER_SIZE_Y,
            },
        },
        Gravity::default(),
        Player::default(),
    ));
    play_attack_sound(&audio, &assets);
}

fn player_dead(
    mut player: Query<(&mut Movement, Entity), With<Player>>,
    mut cmd: Commands,
    assets: Res<PlayerAssets>,
    audio: Res<Audio>,
) {
    let (mut movement, entity) = player.single_mut();
    movement.y = PLAYER_JUMP_STRENGTH;

    cmd.entity(entity)
        .remove::<Player>()
        .insert(PlayerCorpse)
        .insert(
            ParticleEmitter::new(1, Duration::from_secs_f32(0.01), TimerMode::Repeating)
                .with_color(Color::RED),
        )
        .insert(assets.state_dead.clone());
    play_death_sound(&audio, &assets);
}

fn clean_player(mut cmd: Commands, player: Query<Entity, With<PlayerCorpse>>) {
    player.for_each(|x| {
        cmd.entity(x).insert(Dead::default());
    })
}

fn player_out_of_bounds(
    mut event: EventWriter<GameOverEvent>,
    player: Query<(&Transform, &Sprite), With<Player>>,
) {
    let player = player.single();
    let pos = player.0.translation.y;
    let size = player.1.custom_size.unwrap().y / 2.0;
    let bottom = pos - size;
    let top = pos + size;
    if bottom < VIEW_BOX.min.y || top > VIEW_BOX.max.y {
        event.send_default();
    }
}

fn play_attack_sound(audio: &Res<Audio>, assets: &PlayerAssets) {
    let sound = assets
        .attack_sounds
        .get(rand::random::<usize>() % assets.attack_sounds.len())
        .unwrap()
        .clone();
    audio.play(sound);
}

fn play_death_sound(audio: &Res<Audio>, assets: &PlayerAssets) {
    let sound = assets
        .death_sounds
        .get(rand::random::<usize>() % assets.death_sounds.len())
        .unwrap()
        .clone();
    audio.play(sound);
}
