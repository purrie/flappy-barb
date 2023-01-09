use std::time::Duration;

use bevy::prelude::*;

use crate::{
    cleanup::Dead,
    game::GameState,
    physics::{Collider, CollisionEvent, FaceMovementDirection, Gravity, Movement},
    player::AttackState,
    ui::ScoreEvent,
};

pub struct ObstaclesPlugin;

impl Plugin for ObstaclesPlugin {
    fn build(&self, app: &mut App) {
        let start = SystemSet::on_enter(GameState::Playing).with_system(setup_obstacle_spawn_timer);
        let update = SystemSet::on_update(GameState::Playing)
            .with_system(spawn_birds)
            .with_system(bird_animation)
            .with_system(spawn_tree_obstacles)
            .with_system(remove_obstacle.before("cleanup"))
            .with_system(kill_obstacles.before("cleanup").after("collision"));

        app.add_startup_system(load_birds)
            .add_system_set(start)
            .add_system_set(update);
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
            Movement { x: -500., y: 0. },
        );
        cmd.spawn(sprite);
    }
}

fn bird_animation(
    mut cmd: Commands,
    sprites: Res<ObstacleAssets>,
    birds: Query<(Entity, &Transform), (With<Bird>, With<Obstacle>)>,
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
            cmd.entity(x.0).remove::<Obstacle>().insert(Dead);
            if x.2.defeated == false && x.2.kind == ObstacleKind::Bird {
                ev.send(ScoreEvent::ResetCombo)
            }
        });
}

fn kill_obstacles(
    mut cmd: Commands,
    mut ev: EventReader<CollisionEvent>,
    sprites: Res<ObstacleAssets>,
    mut score: EventWriter<ScoreEvent>,
) {
    ev.iter()
        .filter(|x| x.player_state != AttackState::NotAttacking)
        .for_each(|o| {
            cmd.entity(o.obstacle).remove::<Obstacle>().insert(Dead);
            let x = (o.obstacle_pos.x - o.player_pos.x) * (rand::random::<f32>() + 1.);
            let y = (o.obstacle_pos.y - o.player_pos.y) * (rand::random::<f32>() + 1.);
            let force = Vec2 { x, y }.normalize() * 1000.;
            let mut corpse = cmd.spawn(Obstacle {
                defeated: true,
                kind: o.obstacle_kind.clone(),
            });
            match o.obstacle_kind {
                ObstacleKind::Tree => {
                    corpse.insert((
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
                                translation: o.obstacle_pos,
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        Movement {
                            x: -300.,
                            y: -200.,
                        },
                    ));
                }
                ObstacleKind::Bird => {
                    corpse.insert((
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
                                translation: o.obstacle_pos,
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        Movement {
                            x: force.x,
                            y: force.y,
                        },
                        Gravity::default(),
                        FaceMovementDirection {
                            neutral: Vec2 { x: 0., y: -1. },
                        },
                    ));
                    score.send(ScoreEvent::Add);
                }
            };
        });
}
