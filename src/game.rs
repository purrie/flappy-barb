use bevy::{
    core_pipeline::clear_color::ClearColorConfig, prelude::*, render::camera::ScalingMode,
    sprite::Anchor,
};

use crate::ui::{Score, ScoreEvent};

const SKY_COLOR: Color = Color::Hsla {
    hue: 200.0,
    saturation: 0.4,
    lightness: 0.7,
    alpha: 1.0,
};

pub const VIEW_BOX: Rect = Rect {
    min: Vec2 {
        x: -860.0,
        y: -540.0,
    },
    max: Vec2 { x: 860.0, y: 540.0 },
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        let play_start = SystemSet::on_enter(GameState::Playing).with_system(reset_elapsed_time);
        let play_update = SystemSet::on_update(GameState::Playing)
            .with_system(advance_elapsed_time)
            .with_system(game_over.label("game_over").after("collision"));

        let menu_update =
            SystemSet::on_update(GameState::MainMenu).with_system(start_game_shortcut);

        let end_update = SystemSet::on_update(GameState::End).with_system(start_game_shortcut);

        app.add_state(GameState::MainMenu)
            .add_event::<GameOverEvent>()
            .insert_resource(ScreenShake { shake: 0.0 })
            .insert_resource(ElapsedTime { time: 0.0 })
            .add_startup_system(make_camera)
            .add_startup_system(make_background)
            .add_system(fade_out)
            .add_system(screen_shake)
            .add_system(side_scroll)
            .add_system_set(menu_update)
            .add_system_set(play_start)
            .add_system_set(play_update)
            .add_system_set(end_update);
    }
}

#[derive(Default)]
pub struct GameOverEvent;

#[derive(PartialEq, Hash, Debug, Eq, Clone)]
pub enum GameState {
    Playing,
    End,
    MainMenu,
}

#[derive(Resource)]
struct ScreenShake {
    shake: f32,
}

#[derive(Component)]
pub struct FadeOut {
    pub speed: f32,
}

#[derive(Component)]
struct Scroll {
    speed: f32,
}

#[derive(Resource)]
pub struct ElapsedTime {
    pub time: f32,
}

fn make_camera(mut cmd: Commands) {
    cmd.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(SKY_COLOR),
        },
        projection: OrthographicProjection {
            top: VIEW_BOX.max.y,
            bottom: VIEW_BOX.min.y,
            right: VIEW_BOX.max.x,
            left: VIEW_BOX.min.x,
            scaling_mode: ScalingMode::None,
            ..default()
        },
        transform: Transform::from_translation(Vec3 {
            z: 5.0,
            ..default()
        }),
        ..default()
    });
}

fn screen_shake(
    mut camera: Query<&mut Transform, With<Camera2d>>,
    mut event: EventReader<ScoreEvent>,
    mut shake: ResMut<ScreenShake>,
    score: Res<Score>,
    time: Res<Time>,
) {
    event.iter().for_each(|ev| {
        shake.shake = match ev {
            ScoreEvent::Add => 1.0,
            ScoreEvent::ResetCombo => 0.0,
        };
    });
    // shaking describes the angle the camera is rotated each frame on shake
    let shaking = 0.05 * (rand::random::<f32>() * 2.0 - 1.0);
    // scale describes how fast the camera rotates towards the angle
    let scale = time.delta_seconds() * score.current_combo.min(100) as f32;

    let mut camera = camera.single_mut();
    // lineral interpolation towards the angle
    let shaking = camera.rotation.to_euler(EulerRot::XYZ).2 * (1.0 - scale) + shaking * scale;
    // then scaling the shaking by the on/off scalar
    let shaking = shaking * shake.shake;

    // applying the shaking
    camera.rotation = Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, shaking);

    // slowly turning the on/off scalar to off position to ease out of the shaking
    shake.shake = shake.shake * (1.0 - time.delta_seconds() * 10.0);
}

fn reset_elapsed_time(mut elapsed: ResMut<ElapsedTime>) {
    elapsed.time = 0.0;
}
fn advance_elapsed_time(mut elapsed: ResMut<ElapsedTime>, time: Res<Time>) {
    elapsed.time += time.delta_seconds();
}

fn make_background(mut cmd: Commands, asset_server: Res<AssetServer>) {
    let bg1 = asset_server.load("sprites/grass.png");
    let bg2 = asset_server.load("sprites/hills.png");
    let bg3 = asset_server.load("sprites/hill.png");
    let sun = asset_server.load("sprites/sun.png");
    let size = Vec2 {
        x: VIEW_BOX.width(),
        y: 400.0,
    };
    let size2 = Vec2 {
        x: VIEW_BOX.width(),
        y: 300.0,
    };
    let size3 = Vec2 { x: 300.0, y: 300.0 };

    cmd.spawn((
        SpriteBundle {
            texture: bg1.clone(),
            sprite: Sprite {
                custom_size: Some(size),
                anchor: Anchor::BottomRight,
                ..default()
            },
            transform: Transform::from_translation(Vec3 {
                x: VIEW_BOX.max.x,
                y: VIEW_BOX.min.y - size.y * 0.1,
                z: -1.0,
            }),
            ..default()
        },
        Scroll { speed: 200.0 },
    ));
    cmd.spawn((
        SpriteBundle {
            texture: bg1,
            sprite: Sprite {
                custom_size: Some(size),
                anchor: Anchor::BottomRight,
                ..default()
            },
            transform: Transform::from_translation(Vec3 {
                x: VIEW_BOX.max.x + size.x,
                y: VIEW_BOX.min.y - size.y * 0.1,
                z: -1.0,
            }),
            ..default()
        },
        Scroll { speed: 200.0 },
    ));
    cmd.spawn((
        SpriteBundle {
            texture: bg2.clone(),
            sprite: Sprite {
                custom_size: Some(size2),
                anchor: Anchor::BottomRight,
                ..default()
            },
            transform: Transform::from_translation(Vec3 {
                x: VIEW_BOX.max.x,
                y: VIEW_BOX.min.y,
                z: -2.0,
            }),
            ..default()
        },
        Scroll { speed: 100.0 },
    ));
    cmd.spawn((
        SpriteBundle {
            texture: bg2,
            sprite: Sprite {
                custom_size: Some(size2),
                anchor: Anchor::BottomRight,
                ..default()
            },
            transform: Transform::from_translation(Vec3 {
                x: VIEW_BOX.max.x + size2.x,
                y: VIEW_BOX.min.y,
                z: -2.0,
            }),
            ..default()
        },
        Scroll { speed: 100.0 },
    ));
    cmd.spawn((
        SpriteBundle {
            texture: bg3,
            sprite: Sprite {
                custom_size: Some(size3),
                anchor: Anchor::BottomRight,
                ..default()
            },
            transform: Transform::from_translation(Vec3 {
                x: VIEW_BOX.max.x + size3.x,
                y: VIEW_BOX.min.y + 200.0,
                z: -3.0,
            }),
            ..default()
        },
        Scroll { speed: 50.0 },
    ));
    cmd.spawn(SpriteBundle {
        texture: sun,
        sprite: Sprite {
            custom_size: Some(Vec2 { x: 256.0, y: 256.0 }),
            anchor: Anchor::TopRight,
            ..default()
        },
        transform: Transform::from_translation(Vec3 {
            x: VIEW_BOX.max.x,
            y: VIEW_BOX.max.y,
            z: -4.0,
        }),
        ..default()
    });
}

fn side_scroll(mut background: Query<(&mut Transform, &Scroll)>, time: Res<Time>) {
    background.for_each_mut(|(mut tr, scr)| {
        let scroll = scr.speed * time.delta_seconds();
        let mut newx = tr.translation.x - scroll;
        if newx <= VIEW_BOX.min.x {
            newx += VIEW_BOX.width() * 2.0;
        }
        tr.translation.x = newx;
    })
}

fn game_over(go: EventReader<GameOverEvent>, mut end: ResMut<State<GameState>>) {
    if go.is_empty() == false {
        if let Err(e) = end.set(GameState::End) {
            println!("Error: {e}");
        }
        go.clear();
    }
}

fn start_game_shortcut(input: Res<Input<KeyCode>>, mut state: ResMut<State<GameState>>) {
    if input.just_pressed(KeyCode::Return) {
        if let Err(e) = state.set(GameState::Playing) {
            println!("Error: {e}");
        }
    }
}

fn fade_out(mut fade: Query<(&mut Sprite, &FadeOut)>, time: Res<Time>) {
    fade.for_each_mut(|(mut sprite, fade)| {
        let a = sprite.color.a();
        sprite.color.set_a(a - fade.speed * time.delta_seconds());
    })
}
