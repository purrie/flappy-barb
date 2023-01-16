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
            .with_system(remove_obstacle.before("cleanup"))
            .with_system(projectiles.after("projectiles"))
            .with_system(obstacle_collision.before("game_over").after("collision"));

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
}

#[derive(Component)]
struct Bird;

#[derive(Component)]
struct Tree;

#[derive(Resource, Deref, DerefMut)]
struct BirdSpawnTimer {
    timer: Timer,
}

#[derive(Resource, Deref, DerefMut)]
struct TreeSpawnTimer {
    timer: Timer,
}

#[derive(Resource, Default)]
struct ObstacleAssets {
    bird_fly_1: Handle<Image>,
    bird_fly_2: Handle<Image>,
    bird_dead: Handle<Image>,
    tree_normal: Handle<Image>,
    tree_dead: Handle<Image>,
}

impl ObstacleAssets {
    pub const BIRD_SPRITE_SIZE_X: f32 = 128.;
    pub const BIRD_SPRITE_SIZE_Y: f32 = 128.;

    pub const TREE_SPRITE_SIZE_X: f32 = 256.;
    pub const TREE_SPRITE_SIZE_Y: f32 = 256.;
}

fn load_birds(mut cmd: Commands, asset_server: Res<AssetServer>) {
    let bs = ObstacleAssets {
        bird_fly_1: asset_server.load("sprites/bird-fly-1.png"),
        bird_fly_2: asset_server.load("sprites/bird-fly-2.png"),
        bird_dead: asset_server.load("sprites/bird-dead.png"),
        tree_normal: asset_server.load("sprites/tree-full.png"),
        tree_dead: asset_server.load("sprites/tree-cut.png"),
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
            cmd.entity(x.0).remove::<Obstacle>().insert(Dead::default());
            if x.2.defeated == false && x.2.kind == ObstacleKind::Bird {
                ev.send(ScoreEvent::ResetCombo)
            }
        });
}

fn obstacle_collision(
    mut cmd: Commands,
    mut ev: EventReader<CollisionEvent>,
    sprites: Res<ObstacleAssets>,
    mut score: EventWriter<ScoreEvent>,
    mut game_over: EventWriter<GameOverEvent>,
) {
    ev.iter().for_each(|o| {
        if o.player_state == AttackState::NotAttacking {
            if o.player_pos.distance(o.obstacle_pos) < 100.0 {
                game_over.send_default();
            }
            return;
        }
        cmd.entity(o.obstacle)
            .remove::<Obstacle>()
            .insert(Dead::default());

        let x = (o.obstacle_pos.x - o.player_pos.x) * (rand::random::<f32>() + 1.);
        let y = (o.obstacle_pos.y - o.player_pos.y) * (rand::random::<f32>() + 1.);
        let force = Vec2 { x, y }.normalize() * 1000.;
        let hit_location = (o.obstacle_pos + o.player_pos) / 2.0;

        match o.obstacle_kind {
            ObstacleKind::Tree => {
                spawn_tree_corpse(&mut cmd, &sprites, o.obstacle_pos);
                spawn_hit(&mut cmd, Color::GREEN, hit_location, force);
            }
            ObstacleKind::Bird => {
                spawn_bird_corpse(&mut cmd, &sprites, o.obstacle_pos, force);
                spawn_hit(&mut cmd, Color::RED, hit_location, force);
                score.send(ScoreEvent::Add);
            }
        };
    });
}

fn projectiles(
    mut cmd: Commands,
    mut ev: EventReader<ProjectileCollisionEvent>,
    sprites: Res<ObstacleAssets>,
    mut score: EventWriter<ScoreEvent>,
) {
    ev.iter().for_each(|e| {
        if e.hit_pos.distance(e.projectile_pos) > 100.0 {
            return;
        }
        cmd.entity(e.hit)
            .remove::<Obstacle>()
            .insert(Dead::default());

        let x = (e.hit_pos.x - e.projectile_pos.x) * (rand::random::<f32>() + 1.0);
        let y = (e.hit_pos.y - e.projectile_pos.y) * (rand::random::<f32>() + 1.0);
        let force = Vec2 { x, y }.normalize() * 1000.0;
        let hit_location = (e.hit_pos + e.projectile_pos) / 2.0;

        match e.hit_kind {
            ObstacleKind::Tree => {
                spawn_tree_corpse(&mut cmd, &sprites, e.hit_pos);
                spawn_hit(&mut cmd, Color::GREEN, hit_location, force);
            }
            ObstacleKind::Bird => {
                spawn_bird_corpse(&mut cmd, &sprites, e.hit_pos, force);
                spawn_hit(&mut cmd, Color::RED, hit_location, force);
                score.send(ScoreEvent::Add);
            }
        }
    });
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

fn cleanup_obstacles(mut cmd: Commands, obs: Query<Entity, With<Obstacle>>) {
    obs.for_each(|x| {
        cmd.entity(x).insert(Dead::default());
    });
}
