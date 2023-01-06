use std::time::Duration;

use bevy::prelude::*;

use crate::{cleanup::Dead, physics::CollisionEvent, player::AttackState};

pub struct ObstaclesPlugin;

impl Plugin for ObstaclesPlugin {
    fn build(&self, app: &mut App) {
        let birb_time = BirdSpawnTimer {
            timer: Timer::new(Duration::new(2, 0), TimerMode::Repeating),
        };

        app.insert_resource(birb_time)
            .insert_resource(BirdSprites::default())
            .add_startup_system(load_birds)
            .add_system(spawn_birds)
            .add_system(move_obstacles)
            .add_system(bird_animation)
            .add_system(remove_obstacle.before("cleanup"))
            .add_system(kill_obstacles.before("cleanup"));
    }
}

#[derive(Component)]
pub struct Obstacle;

#[derive(Component)]
struct Bird;

#[derive(Component)]
struct HorizontalMove {
    speed: f32,
}

#[derive(Resource, Deref, DerefMut)]
struct BirdSpawnTimer {
    timer: Timer,
}

#[derive(Resource, Default)]
struct BirdSprites {
    first: Handle<Image>,
    second: Handle<Image>,
}

fn load_birds(mut bs: ResMut<BirdSprites>, asset_server: Res<AssetServer>) {
    bs.first = asset_server.load("sprites/bird-fly-1.png");
    bs.second = asset_server.load("sprites/bird-fly-2.png");
}

fn spawn_birds(
    mut cmd: Commands,
    sprites: Res<BirdSprites>,
    mut timer: ResMut<BirdSpawnTimer>,
    time: Res<Time>,
    camera: Query<&OrthographicProjection>,
) {
    if timer.tick(time.delta()).just_finished() {
        let op = camera.get_single().unwrap();
        let rand_height: f32 = rand::random::<f32>();
        let height = (rand_height * 0.6) + 0.2;
        let height = op.top * (1. - height) + op.bottom * height;
        let img = sprites.first.clone();
        let sprite = (
            SpriteBundle {
                texture: img,
                sprite: Sprite {
                    custom_size: Some(Vec2 { x: 128., y: 128. }),
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
            Obstacle,
            Bird,
            HorizontalMove { speed: 500. },
        );
        cmd.spawn(sprite);
    }
}

fn move_obstacles(
    time: Res<Time>,
    mut q: Query<(&mut HorizontalMove, &mut Transform), With<Obstacle>>,
) {
    q.iter_mut()
        .for_each(|mut x| x.1.translation.x -= x.0.speed * time.delta_seconds());
}

fn bird_animation(
    mut cmd: Commands,
    sprites: Res<BirdSprites>,
    birds: Query<(Entity, &Transform), (With<Bird>, With<Obstacle>)>,
) {
    birds.for_each(|x| {
        let Some(mut cmd) = cmd.get_entity(x.0) else {
            return;
        };
        let tick = x.1.translation.x.abs() as i32 % 200;
        match tick >= 100 {
            true => {
                cmd.insert(sprites.first.clone());
            }
            false => {
                cmd.insert(sprites.second.clone());
            }
        }
    })
}

fn remove_obstacle(
    mut cmd: Commands,
    camera_view: Query<&OrthographicProjection>,
    obstacles: Query<(Entity, &Transform), With<Obstacle>>,
) {
    let op = camera_view.get_single().unwrap();
    obstacles
        .iter()
        .filter(|x| x.1.translation.x < (op.left - 64.))
        .for_each(|x| {
            cmd.entity(x.0).remove::<Obstacle>().insert(Dead);
        });
}

fn kill_obstacles(mut cmd: Commands, mut ev: EventReader<CollisionEvent>) {
    ev.iter()
        .filter(|x| x.player_state != AttackState::NotAttacking)
        .for_each(|x| {
            cmd.entity(x.obstacle).remove::<Obstacle>().insert(Dead);
        });
}
