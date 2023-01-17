use std::time::Duration;

use bevy::prelude::*;

use crate::{
    cleanup::Dead,
    game::{GameOverEvent, GameState},
    particles::{EmissionDirection, ParticleEmitter},
    physics::{
        Collider, CollisionEvent, FaceMovementDirection, Gravity, Movement, Projectile,
        ProjectileCollisionEvent,
    },
    player::AttackState,
    ui::ScoreEvent,
};

pub struct ObstaclesPlugin;

impl Plugin for ObstaclesPlugin {
    fn build(&self, app: &mut App) {
        let start = SystemSet::on_enter(GameState::Playing).with_system(setup_obstacle_spawn_timer);
        let update = SystemSet::on_update(GameState::Playing)
            .with_system(spawn_birds)
            .with_system(bird_animation.before("cleanup"))
            .with_system(spawn_tree_obstacles)
            .with_system(spawn_cloud_obstacles)
            .with_system(remove_obstacle.before("cleanup"))
            .with_system(projectiles.after("projectiles"))
            .with_system(
                obstacle_player_collision
                    .before("game_over")
                    .after("collision"),
            );

        let cleanup = SystemSet::on_exit(GameState::End).with_system(cleanup_obstacles);

        app.add_startup_system(load_birds)
            .add_system_set(start)
            .add_system_set(update)
            .add_system_set(cleanup);
    }
}

#[derive(Component, Default)]
pub struct Obstacle {
    pub defeated: bool,
    pub kind: ObstacleKind,
}

#[derive(Default, Clone, PartialEq)]
pub enum ObstacleKind {
    Tree,
    #[default]
    Bird,
    Cloud,
}

#[derive(Component)]
struct Bird;

#[derive(Component)]
struct Tree;

#[derive(Component)]
struct Cloud;

#[derive(Resource, Deref, DerefMut)]
struct BirdSpawnTimer {
    timer: Timer,
}

#[derive(Resource, Deref, DerefMut)]
struct TreeSpawnTimer {
    timer: Timer,
}

#[derive(Resource, Deref, DerefMut)]
struct CloudSpawnTimer {
    timer: Timer,
}

#[derive(Resource, Default)]
struct ObstacleAssets {
    bird_fly_1: Handle<Image>,
    bird_fly_2: Handle<Image>,
    bird_dead: Handle<Image>,
    tree_normal: Handle<Image>,
    tree_dead: Handle<Image>,
    cloud_normal: Handle<Image>,
    cloud_dead: Handle<Image>,
    bird_death_sounds: Vec<Handle<AudioSource>>,
    tree_death_sounds: Vec<Handle<AudioSource>>,
}

impl ObstacleAssets {
    pub const BIRD_SPRITE_SIZE_X: f32 = 128.;
    pub const BIRD_SPRITE_SIZE_Y: f32 = 128.;

    pub const TREE_SPRITE_SIZE_X: f32 = 256.;
    pub const TREE_SPRITE_SIZE_Y: f32 = 256.;

    pub const CLOUD_SPRITE_SIZE_X: f32 = 256.0;
    pub const CLOUD_SPRITE_SIZE_Y: f32 = 256.0;
}

fn load_birds(mut cmd: Commands, asset_server: Res<AssetServer>) {
    let mut bird_death_sounds = vec![];
    for i in 1..=11 {
        bird_death_sounds.push(asset_server.load(format!("audio/bird-death-{}.ogg", i)));
    }
    let mut tree_death_sounds = vec![];
    for i in 1..=2 {
        tree_death_sounds.push(asset_server.load(format!("audio/tree-death-{}.ogg", i)));
    }

    let bs = ObstacleAssets {
        bird_fly_1: asset_server.load("sprites/bird-fly-1.png"),
        bird_fly_2: asset_server.load("sprites/bird-fly-2.png"),
        bird_dead: asset_server.load("sprites/bird-dead.png"),
        tree_normal: asset_server.load("sprites/tree-full.png"),
        tree_dead: asset_server.load("sprites/tree-cut.png"),
        cloud_normal: asset_server.load("sprites/cloud.png"),
        cloud_dead: asset_server.load("sprites/cloud-cut.png"),
        bird_death_sounds,
        tree_death_sounds,
    };
    cmd.insert_resource(bs);
}

fn setup_obstacle_spawn_timer(mut cmd: Commands) {
    let birb_time = BirdSpawnTimer {
        timer: Timer::new(Duration::new(1, 0), TimerMode::Once),
    };
    cmd.insert_resource(birb_time);

    let tree_time = TreeSpawnTimer {
        timer: Timer::new(Duration::new(1, 0), TimerMode::Once),
    };
    cmd.insert_resource(tree_time);

    let cloud_time = CloudSpawnTimer {
        timer: Timer::new(Duration::new(1, 0), TimerMode::Once),
    };
    cmd.insert_resource(cloud_time);
}

fn spawn_birds(
    mut cmd: Commands,
    sprites: Res<ObstacleAssets>,
    mut timer: ResMut<BirdSpawnTimer>,
    time: Res<Time>,
    camera: Query<&OrthographicProjection>,
) {
    if timer.tick(time.delta()).just_finished() {
        let duration = Duration::new(1, rand::random::<u32>() % 1000000000);
        timer.set_duration(duration);
        timer.reset();
        let op = camera.get_single().unwrap();
        let rand_height: f32 = rand::random::<f32>();
        let height = (rand_height * 0.6) + 0.2;
        let height = op.top * (1. - height) + op.bottom * height;
        let img = sprites.bird_fly_1.clone();
        let random_speed = rand::random::<i32>() % 200 - 100;
        let sprite = (
            SpriteBundle {
                texture: img,
                sprite: Sprite {
                    custom_size: Some(Vec2 {
                        x: ObstacleAssets::BIRD_SPRITE_SIZE_X,
                        y: ObstacleAssets::BIRD_SPRITE_SIZE_Y,
                    }),
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3 {
                        x: op.right + 64.,
                        y: height,
                        z: 0.,
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            Obstacle {
                kind: ObstacleKind::Bird,
                ..default()
            },
            Bird,
            Collider,
            Movement {
                x: -500.0 + random_speed as f32,
                y: 0.,
            },
        );
        cmd.spawn(sprite);
    }
}

fn bird_animation(
    mut cmd: Commands,
    sprites: Res<ObstacleAssets>,
    birds: Query<(Entity, &Transform), (With<Bird>, With<Obstacle>, Without<Dead>)>,
) {
    birds.for_each(|x| {
        let Some(mut cmd) = cmd.get_entity(x.0) else {
            return;
        };
        let tick = x.1.translation.x.abs() as i32 % 200;
        match tick >= 100 {
            true => {
                cmd.insert(sprites.bird_fly_1.clone());
            }
            false => {
                cmd.insert(sprites.bird_fly_2.clone());
            }
        }
    })
}

fn spawn_tree_obstacles(
    mut cmd: Commands,
    sprites: Res<ObstacleAssets>,
    mut timer: ResMut<TreeSpawnTimer>,
    time: Res<Time>,
    camera: Query<&OrthographicProjection>,
) {
    if timer.tick(time.delta()).just_finished() {
        let duration = Duration::new(1, rand::random::<u32>());
        timer.set_duration(duration);
        timer.reset();
        let op = camera.get_single().unwrap();
        let height = op.bottom + ObstacleAssets::TREE_SPRITE_SIZE_Y / 2.;

        let img = sprites.tree_normal.clone();
        let sprite = (
            SpriteBundle {
                texture: img,
                sprite: Sprite {
                    custom_size: Some(Vec2 {
                        x: ObstacleAssets::TREE_SPRITE_SIZE_X,
                        y: ObstacleAssets::TREE_SPRITE_SIZE_Y,
                    }),
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3 {
                        x: op.right + 64.,
                        y: height,
                        z: 0.,
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            Obstacle {
                kind: ObstacleKind::Tree,
                ..default()
            },
            Tree,
            Collider,
            Movement { x: -300., y: 0. },
        );
        cmd.spawn(sprite);
    }
}

fn spawn_cloud_obstacles(
    mut cmd: Commands,
    assets: Res<ObstacleAssets>,
    camera: Query<&OrthographicProjection>,
    time: Res<Time>,
    mut timer: ResMut<CloudSpawnTimer>,
) {
    if timer.tick(time.delta()).just_finished() {
        let duration = Duration::new(3, rand::random::<u32>());
        timer.set_duration(duration);
        timer.reset();
        let camera = camera.single();
        let img = assets.cloud_normal.clone();
        let cloud = (
            SpriteBundle {
                texture: img,
                sprite: Sprite {
                    custom_size: Some(Vec2 {
                        x: ObstacleAssets::CLOUD_SPRITE_SIZE_X,
                        y: ObstacleAssets::CLOUD_SPRITE_SIZE_Y,
                    }),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3 {
                        x: camera.right + ObstacleAssets::CLOUD_SPRITE_SIZE_X,
                        y: camera.top
                            - ObstacleAssets::CLOUD_SPRITE_SIZE_Y
                                * (rand::random::<f32>() * 0.3 + 0.5),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            },
            Obstacle {
                kind: ObstacleKind::Cloud,
                ..default()
            },
            Cloud,
            Collider,
            Movement { x: -200.0, y: 0.0 },
        );
        cmd.spawn(cloud);
    }
}

fn remove_obstacle(
    mut cmd: Commands,
    camera_view: Query<&OrthographicProjection>,
    obstacles: Query<(Entity, &Transform, &Obstacle)>,
    mut ev: EventWriter<ScoreEvent>,
) {
    let op = camera_view.get_single().unwrap();
    obstacles
        .iter()
        .filter(|x| x.1.translation.x < (op.left - 128.) || x.1.translation.y < (op.bottom - 128.))
        .for_each(|x| {
            cmd.entity(x.0)
                .remove::<Obstacle>()
                .insert(Dead { timer: 1.0 });
            if x.2.defeated == false && x.2.kind == ObstacleKind::Bird {
                ev.send(ScoreEvent::ResetCombo)
            }
        });
}

fn obstacle_player_collision(
    mut cmd: Commands,
    mut ev: EventReader<CollisionEvent>,
    assets: Res<ObstacleAssets>,
    mut score: EventWriter<ScoreEvent>,
    mut game_over: EventWriter<GameOverEvent>,
    audio: Res<Audio>,
) {
    ev.iter().for_each(|o| {
        if o.player_state == AttackState::NotAttacking {
            if o.player_pos.distance(o.obstacle_pos) < 100.0 {
                game_over.send_default();
            }
            return;
        }
        obstacle_collision_handle(
            &mut cmd,
            &assets,
            &audio,
            &mut score,
            o.obstacle,
            &o.obstacle_kind,
            o.obstacle_pos,
            o.player_pos,
            true,
        );
    });
}

fn projectiles(
    mut cmd: Commands,
    mut ev: EventReader<ProjectileCollisionEvent>,
    assets: Res<ObstacleAssets>,
    mut score: EventWriter<ScoreEvent>,
    audio: Res<Audio>,
) {
    ev.iter().for_each(|e| {
        if e.hit_pos.distance(e.projectile_pos) > 100.0 {
            return;
        }
        obstacle_collision_handle(
            &mut cmd,
            &assets,
            &audio,
            &mut score,
            e.hit,
            &e.hit_kind,
            e.hit_pos,
            e.projectile_pos,
            false,
        );
    });
}

fn obstacle_collision_handle(
    cmd: &mut Commands,
    assets: &Res<ObstacleAssets>,
    audio: &Res<Audio>,
    score: &mut EventWriter<ScoreEvent>,
    obstacle: Entity,
    obstacle_kind: &ObstacleKind,
    obstacle_pos: Vec3,
    hit_pos: Vec3,
    is_player_collision: bool,
) {
    let x = (obstacle_pos.x - hit_pos.x) * (rand::random::<f32>() + 1.0);
    let y = (obstacle_pos.y - hit_pos.y) * (rand::random::<f32>() + 1.0);
    let force = Vec2 { x, y }.normalize() * 1000.0;
    let hit_location = (obstacle_pos + hit_pos) / 2.0;

    match obstacle_kind {
        ObstacleKind::Tree => {
            cmd.entity(obstacle)
                .remove::<Obstacle>()
                .insert(Dead::default());
            spawn_tree_corpse(cmd, &assets, obstacle_pos);
            spawn_hit(cmd, Color::GREEN, hit_location, force);
            play_tree_death_sound(&audio, &assets);
        }
        ObstacleKind::Bird => {
            cmd.entity(obstacle)
                .remove::<Obstacle>()
                .insert(Dead::default());
            spawn_bird_corpse(cmd, &assets, obstacle_pos, force);
            spawn_hit(cmd, Color::RED, hit_location, force);
            score.send(ScoreEvent::Add);
            play_bird_death_sound(&audio, &assets);
        }
        ObstacleKind::Cloud if is_player_collision => {
            cmd.entity(obstacle)
                .remove::<Obstacle>()
                .insert(Dead::default());
            spawn_cloud_corpse(cmd, &assets, obstacle_pos, force);
            spawn_hit(cmd, Color::WHITE, hit_location, force);
        }
        ObstacleKind::Cloud => {}
    }
}

fn play_bird_death_sound(audio: &Res<Audio>, assets: &Res<ObstacleAssets>) {
    let sound = assets
        .bird_death_sounds
        .get(rand::random::<usize>() % assets.bird_death_sounds.len())
        .unwrap()
        .clone();
    audio.play(sound);
}

fn play_tree_death_sound(audio: &Res<Audio>, assets: &Res<ObstacleAssets>) {
    let sound = assets
        .tree_death_sounds
        .get(rand::random::<usize>() % assets.tree_death_sounds.len())
        .unwrap()
        .clone();
    audio.play(sound);
}

fn spawn_hit(cmd: &mut Commands, color: Color, location: Vec3, force: Vec2) {
    cmd.spawn((
        ParticleEmitter::new(3, Duration::new(0, 500), TimerMode::Repeating)
            .with_color(color)
            .with_direction(EmissionDirection::Global(force)),
        Transform {
            translation: location,
            ..default()
        },
        Dead { timer: 0.1 },
    ));
}

fn spawn_tree_corpse(cmd: &mut Commands, sprites: &ObstacleAssets, location: Vec3) {
    cmd.spawn((
        Obstacle {
            defeated: true,
            kind: ObstacleKind::Tree,
        },
        SpriteBundle {
            texture: sprites.tree_dead.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2 {
                    x: ObstacleAssets::TREE_SPRITE_SIZE_X,
                    y: ObstacleAssets::TREE_SPRITE_SIZE_Y,
                }),
                ..Default::default()
            },
            transform: Transform {
                translation: location,
                ..Default::default()
            },
            ..Default::default()
        },
        Movement { x: -300., y: -200. },
    ));
}

fn spawn_bird_corpse(cmd: &mut Commands, sprites: &ObstacleAssets, location: Vec3, movement: Vec2) {
    cmd.spawn((
        Obstacle {
            defeated: true,
            kind: ObstacleKind::Bird,
        },
        SpriteBundle {
            texture: sprites.bird_dead.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2 {
                    x: ObstacleAssets::BIRD_SPRITE_SIZE_X,
                    y: ObstacleAssets::BIRD_SPRITE_SIZE_Y,
                }),
                ..Default::default()
            },
            transform: Transform {
                translation: location,
                ..Default::default()
            },
            ..Default::default()
        },
        Movement {
            x: movement.x,
            y: movement.y,
        },
        Projectile,
        Gravity::default(),
        FaceMovementDirection {
            neutral: Vec2 { x: 0., y: -1. },
        },
        ParticleEmitter::new(1, Duration::new(0, 50000), TimerMode::Repeating)
            .with_color(Color::RED)
            .with_direction(EmissionDirection::Local(Vec2::Y)),
    ));
}

fn spawn_cloud_corpse(
    cmd: &mut Commands,
    sprites: &ObstacleAssets,
    location: Vec3,
    movement: Vec2,
) {
    for i in 0..2 {
        let movement = Vec2 {
            x: if i == 0 { -movement.x } else { movement.x } * 0.5,
            y: movement.y * 0.5,
        };
        let i = i as f32;
        let location = Vec3 {
            x: location.x - (ObstacleAssets::CLOUD_SPRITE_SIZE_X / 4.0)
                + (ObstacleAssets::CLOUD_SPRITE_SIZE_X / 2.0 * i),
            y: location.y,
            z: location.z,
        };
        cmd.spawn((
            Obstacle {
                defeated: true,
                kind: ObstacleKind::Cloud,
            },
            SpriteBundle {
                texture: sprites.cloud_dead.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2 {
                        x: ObstacleAssets::CLOUD_SPRITE_SIZE_X * 0.5,
                        y: ObstacleAssets::CLOUD_SPRITE_SIZE_Y,
                    }),
                    rect: Some(Rect {
                        min: Vec2 {
                            x: 32.0 * i,
                            y: 0.0,
                        },
                        max: Vec2 {
                            x: 32.0 * (i + 1.0),
                            y: 64.0,
                        },
                    }),
                    ..default()
                },
                transform: Transform {
                    translation: location,
                    ..default()
                },
                ..default()
            },
            Movement {
                x: movement.x,
                y: movement.y,
            },
            ParticleEmitter::new(1, Duration::new(0, 50000), TimerMode::Repeating)
                .with_color(Color::WHITE)
                .with_direction(EmissionDirection::Global(movement * -1.0)),
            Dead { timer: 3.0 },
        ));
    }
}

fn cleanup_obstacles(mut cmd: Commands, obs: Query<Entity, With<Obstacle>>) {
    obs.for_each(|x| {
        cmd.entity(x).insert(Dead::default());
    });
}
