use bevy::prelude::*;

use crate::{cleanup::Dead, game::GameState};

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        let start_menu = SystemSet::on_enter(GameState::MainMenu).with_system(spawn_main_manu);
        let update_menu = SystemSet::on_update(GameState::MainMenu).with_system(main_menu);

        let start_game = SystemSet::on_enter(GameState::Playing).with_system(spawn_scoreboard);
        let update_game = SystemSet::on_update(GameState::Playing).with_system(score_event);
        let exit_game = SystemSet::on_exit(GameState::Playing).with_system(despawn_scoreboard);

        app.add_event::<ScoreEvent>()
            .add_startup_system(load_font)
            .add_system_set(start_menu)
            .add_system_set(update_menu)
            .add_system_set(start_game)
            .add_system_set(update_game)
            .add_system_set(exit_game);
    }
}

pub enum ScoreEvent {
    Add,
    ResetCombo,
}

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
}

impl Score {
    pub fn add_to_score(&mut self) {
        let combo_bonus = self.current_combo / 10;
        self.score += 1 + combo_bonus;
        self.current_combo += 1;
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

    cmd.spawn((board, ScoreBoard));
    cmd.insert_resource(Score::default());
}

fn despawn_scoreboard(mut cmd: Commands, board: Query<Entity, With<Text>>) {
    board.for_each(|x| {
        cmd.entity(x).insert(Dead);
    });
}

fn spawn_main_manu(mut cmd: Commands, ui: Res<UiAssets>) {
    cmd.spawn(ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(350.0), Val::Px(85.0)),
            margin: UiRect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: Color::WHITE.into(),
        ..default()
    })
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
    mut cmd: Commands,
    mut state: ResMut<State<GameState>>,
    mut butt: Query<
        (Entity, &Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    butt.for_each_mut(|(ent, int, mut col)| match *int {
        Interaction::Clicked => {
            *col = Color::DARK_GRAY.into();
            state.set(GameState::Playing).unwrap();
            cmd.entity(ent).insert(Dead);
        }
        Interaction::Hovered => *col = Color::GRAY.into(),
        Interaction::None => *col = Color::WHITE.into(),
    })
}
