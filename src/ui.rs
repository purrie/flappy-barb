use bevy::prelude::*;

use crate::{cleanup::Dead, game::GameState};

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        let start_menu = SystemSet::on_enter(GameState::MainMenu).with_system(spawn_main_manu);
        let update_menu = SystemSet::on_update(GameState::MainMenu).with_system(main_menu);
        let exit_menu = SystemSet::on_exit(GameState::MainMenu).with_system(clean_ui);

        let start_game = SystemSet::on_enter(GameState::Playing).with_system(spawn_scoreboard);
        let update_game = SystemSet::on_update(GameState::Playing).with_system(score_event);
        let exit_game = SystemSet::on_exit(GameState::Playing).with_system(clean_ui);

        let start_end = SystemSet::on_enter(GameState::End).with_system(spawn_end_score);
        let update_end = SystemSet::on_update(GameState::End).with_system(end_screen_menu);
        let exit_end = SystemSet::on_exit(GameState::End).with_system(clean_ui);

        app.add_event::<ScoreEvent>()
            .insert_resource(Score::default())
            .add_startup_system(load_font)
            .add_system_set(start_menu)
            .add_system_set(update_menu)
            .add_system_set(exit_menu)
            .add_system_set(start_game)
            .add_system_set(update_game)
            .add_system_set(exit_game)
            .add_system_set(start_end)
            .add_system_set(update_end)
            .add_system_set(exit_end);
    }
}

#[derive(PartialEq)]
pub enum ScoreEvent {
    Add,
    ResetCombo,
}

#[derive(Component)]
pub struct UI;

#[derive(Default, Resource)]
pub struct UiAssets {
    pub font: Handle<Font>,
}

#[derive(Component)]
pub struct ScoreBoard;

#[derive(Default, Resource)]
pub struct Score {
    pub score: i32,
    pub current_combo: i32,
    pub max_combo: i32,
}

impl Score {
    pub fn add_to_score(&mut self) {
        let combo_bonus = self.current_combo / 10;
        self.score += 1 + combo_bonus;
        self.current_combo += 1;
        if self.current_combo > self.max_combo {
            self.max_combo = self.current_combo;
        }
    }
    pub fn reset_combo(&mut self) {
        self.current_combo = 0;
    }
}

fn load_font(mut cmd: Commands, asset_server: Res<AssetServer>) {
    let ui = UiAssets {
        font: asset_server.load("fonts/pixhobo.ttf"),
    };
    cmd.insert_resource(ui);
}

fn score_event(
    mut score: ResMut<Score>,
    mut ev: EventReader<ScoreEvent>,
    mut board: Query<&mut Text, With<ScoreBoard>>,
) {
    ev.iter().for_each(|e| match e {
        ScoreEvent::Add => score.add_to_score(),
        ScoreEvent::ResetCombo => score.reset_combo(),
    });

    let Ok(mut board) = board.get_single_mut() else {
        return;
    };
    board.sections[0].value = format!("Score: {}\nCombo: {}", score.score, score.current_combo);
}

fn spawn_scoreboard(mut cmd: Commands, ui: Res<UiAssets>) {
    let board = TextBundle::from_section(
        "Score: 0\nCombo: 0",
        TextStyle {
            font: ui.font.clone(),
            font_size: 50.,
            color: Color::WHITE,
        },
    )
    .with_style(Style {
        position_type: PositionType::Absolute,
        position: UiRect {
            left: Val::Px(10.),
            top: Val::Px(10.),
            ..Default::default()
        },
        ..Default::default()
    });

    cmd.spawn((board, ScoreBoard, UI));
    cmd.insert_resource(Score::default());
}

fn spawn_main_manu(mut cmd: Commands, ui: Res<UiAssets>) {
    cmd.spawn((
        ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(350.0), Val::Px(85.0)),
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::WHITE.into(),
            ..default()
        },
        UI,
    ))
    .with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "Start Game",
            TextStyle {
                font: ui.font.clone(),
                font_size: 50.0,
                color: Color::BLACK,
            },
        ));
    });
}

fn main_menu(
    mut state: ResMut<State<GameState>>,
    mut butt: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
) {
    butt.for_each_mut(|(int, mut col)| match *int {
        Interaction::Clicked => {
            *col = Color::DARK_GRAY.into();
            state.set(GameState::Playing).unwrap();
        }
        Interaction::Hovered => *col = Color::GRAY.into(),
        Interaction::None => *col = Color::WHITE.into(),
    })
}

fn clean_ui(mut cmd: Commands, butt: Query<Entity, With<UI>>) {
    butt.for_each(|x| {
        cmd.entity(x).insert(Dead::default());
    });
}

fn spawn_end_score(mut cmd: Commands, ui: Res<UiAssets>, score: Res<Score>) {
    cmd.spawn((
        UI,
        NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(50.0), Val::Percent(50.0)),
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Percent(25.0),
                    left: Val::Percent(25.0),
                    ..default()
                },
                padding: UiRect {
                    top: Val::Percent(5.0),
                    bottom: Val::Percent(5.0),
                    ..default()
                },
                // justify_content: JustifyContent::SpaceAround,
                // align_items: AlignItems::Center,
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: Color::WHITE.into(),
            ..default()
        },
    ))
    .with_children(|parent| {
        parent.spawn(
            TextBundle::from_section(
                "Final Score",
                TextStyle {
                    font: ui.font.clone(),
                    color: Color::BLACK,
                    font_size: 40.0,
                },
            )
            .with_style(Style {
                align_self: AlignSelf::Center,
                flex_basis: Val::Px(40.0),
                flex_grow: 0.0,
                ..default()
            }),
        );
        parent.spawn(
            TextBundle::from_section(
                format! {"{}", score.score},
                TextStyle {
                    font: ui.font.clone(),
                    color: Color::BLACK,
                    font_size: 40.0,
                },
            )
            .with_style(Style {
                align_self: AlignSelf::Center,
                ..default()
            }),
        );
        parent.spawn(NodeBundle {
            style: Style {
                flex_grow: 20.0,
                ..default()
            },
            ..default()
        });
        parent.spawn(
            TextBundle::from_section(
                "Top Combo",
                TextStyle {
                    font: ui.font.clone(),
                    color: Color::BLACK,
                    font_size: 40.0,
                },
            )
            .with_style(Style {
                align_self: AlignSelf::Center,
                ..default()
            }),
        );
        parent.spawn(
            TextBundle::from_section(
                format!("{}", score.max_combo),
                TextStyle {
                    font: ui.font.clone(),
                    color: Color::BLACK,
                    font_size: 40.0,
                },
            )
            .with_style(Style {
                align_self: AlignSelf::Center,
                ..default()
            }),
        );
        parent.spawn(NodeBundle {
            style: Style {
                flex_grow: 1000.0,
                ..default()
            },
            ..default()
        });
        parent
            .spawn(ButtonBundle {
                background_color: Color::GRAY.into(),
                style: Style {
                    align_self: AlignSelf::Center,
                    padding: UiRect {
                        top: Val::Px(5.0),
                        bottom: Val::Px(5.0),
                        left: Val::Px(5.0),
                        right: Val::Px(5.0),
                    },
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Restart",
                    TextStyle {
                        color: Color::WHITE,
                        font: ui.font.clone(),
                        font_size: 30.0,
                    },
                ));
            });
    });
}

fn end_screen_menu(
    mut state: ResMut<State<GameState>>,
    mut butt: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
) {
    butt.for_each_mut(|mut b| match b.0 {
        Interaction::Clicked => {
            *b.1 = Color::DARK_GRAY.into();
            state.set(GameState::Playing).unwrap();
        }
        Interaction::Hovered => *b.1 = Color::DARK_GRAY.into(),
        Interaction::None => *b.1 = Color::GRAY.into(),
    })
}
