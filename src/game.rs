use bevy::{
    core_pipeline::clear_color::ClearColorConfig, prelude::*, render::camera::ScalingMode,
    sprite::Anchor,
};

const SKY_COLOR: Color = Color::Hsla {
    hue: 200.0,
    saturation: 0.4,
    lightness: 0.7,
    alpha: 1.0,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        let play_update = SystemSet::on_update(GameState::Playing)
            .with_system(game_over.label("game_over").after("collision"));

        let menu_update =
            SystemSet::on_update(GameState::MainMenu).with_system(start_game_shortcut);

        let end_update = SystemSet::on_update(GameState::End).with_system(start_game_shortcut);

        app.add_state(GameState::MainMenu)
            .add_event::<GameOverEvent>()
            .add_startup_system(make_camera)
            .add_startup_system(make_background)
            .add_system(fade_out)
            .add_system(side_scroll)
            .add_system_set(menu_update)
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

#[derive(Component)]
pub struct FadeOut {
    pub speed: f32,
}

#[derive(Component)]
struct Scroll {
    speed: f32,
}

fn make_camera(mut cmd: Commands) {
    cmd.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(SKY_COLOR),
        },
        projection: OrthographicProjection {
            top: 1080.0,
            right: 1920.0,
            bottom: 0.0,
            left: 0.0,
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

fn make_background(mut cmd: Commands, asset_server: Res<AssetServer>) {
    let bg1 = asset_server.load("sprites/grass.png");
    let bg2 = asset_server.load("sprites/hills.png");
    let size = Vec2 {
        x: 1980.0,
        y: 300.0,
    };
    let size2 = Vec2 {
        x: 1980.0,
        y: 150.0,
    };

    cmd.spawn((
        SpriteBundle {
            texture: bg1.clone(),
            sprite: Sprite {
                custom_size: Some(size),
                anchor: Anchor::BottomRight,
                ..default()
            },
            transform: Transform::from_translation(Vec3 {
                x: 1980.0,
                z: -1.0,
                ..default()
            }),
            ..default()
        },
        Scroll { speed: 300.0 },
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
                x: 3960.0,
                z: -1.0,
                ..default()
            }),
            ..default()
        },
        Scroll { speed: 300.0 },
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
                x: 1980.0,
                y: 100.0,
                z: -2.0,
            }),
            ..default()
        },
        Scroll { speed: 200.0 },
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
                x: 3960.0,
                y: 100.0,
                z: -2.0,
            }),
            ..default()
        },
        Scroll { speed: 200.0 },
    ));
}

fn side_scroll(mut background: Query<(&mut Transform, &Scroll)>, time: Res<Time>) {
    background.for_each_mut(|(mut tr, scr)| {
        let scroll = scr.speed * time.delta_seconds();
        let mut newx = tr.translation.x - scroll;
        if newx < 0.0 {
            newx += 3960.0;
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
