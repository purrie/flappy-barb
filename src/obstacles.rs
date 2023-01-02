use std::time::Duration;

use bevy::prelude::*;

pub struct ObstaclesPlugin;

impl Plugin for ObstaclesPlugin {
    fn build(&self, app: &mut App) {
        let birb_time = BirdSpawnTimer {
            timer: Timer::new(Duration::new(2, 0), TimerMode::Repeating),
        };

        app.insert_resource(birb_time)
            // .insert_resource(BirdSprites::default())
            // .add_startup_system(load_birds)
            .add_system(spawn_birds)
            .add_system(move_obstacles)
            .add_system(remove_obstacle);
    }
}

#[derive(Component)]
struct Obstacle;

#[derive(Component)]
struct HorizontalMove {
    speed: f32,
}

#[derive(Resource, Deref, DerefMut)]
struct BirdSpawnTimer {
    timer: Timer,
}

// #[derive(Resource, Default)]
// struct BirdSprites {
//     first: Handle<Image>,
// }

// fn load_birds(mut bs: ResMut<BirdSprites>, asset_server: Res<AssetServer>) {
//     bs.first = asset_server.load("sprites/testchar.png");
// }

fn spawn_birds(
    mut cmd: Commands,
    // sprites: Res<BirdSprites>,
    mut timer: ResMut<BirdSpawnTimer>,
    time: Res<Time>,
    camera: Query<&OrthographicProjection>,
) {
    if timer.tick(time.delta()).just_finished() {
        let op = camera.get_single().unwrap();
        // let img = sprites.first.clone();
        let sprite = (
            SpriteBundle {
                // texture: img,
                sprite: Sprite {
                    custom_size: Some(Vec2 { x: 64., y: 64. }),
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3 {
                        x: op.right + 64.,
                        y: (op.top + op.bottom) / 2.,
                        z: 0.,
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            Obstacle,
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

fn remove_obstacle(
    mut cmd: Commands,
    camera_view: Query<&OrthographicProjection>,
    obstacles: Query<(Entity, &Transform), With<Obstacle>>,
) {
    let op = camera_view.get_single().unwrap();
    obstacles
        .iter()
        .filter(|x| x.1.translation.x < (op.left - 64.))
        .for_each(|x| cmd.entity(x.0).despawn());
}
