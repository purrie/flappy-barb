use bevy::prelude::*;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ScoreEvent>()
            .insert_resource(Score::default())
            .add_startup_system(spawn_scoreboard)
            .add_system(score_event);
    }
}

pub enum ScoreEvent {
    Add,
    ResetCombo,
    ResetScore,
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
    pub fn reset_score(&mut self) {
        self.current_combo = 0;
        self.score = 0;
    }
}

fn score_event(
    mut score: ResMut<Score>,
    mut ev: EventReader<ScoreEvent>,
    mut board: Query<&mut Text, With<ScoreBoard>>,
) {
    ev.iter().for_each(|e| match e {
        ScoreEvent::Add => score.add_to_score(),
        ScoreEvent::ResetCombo => score.reset_combo(),
        ScoreEvent::ResetScore => score.reset_score(),
    });

    let Ok(mut board) = board.get_single_mut() else {
        return;
    };
    board.sections[0].value = format!("Score: {}\nCombo: {}", score.score, score.current_combo);
}

fn spawn_scoreboard(mut cmd: Commands, asset_server: Res<AssetServer>) {
    let board = TextBundle::from_section(
        "Score: 0\nCombo: 0",
        TextStyle {
            font: asset_server.load("fonts/pixhobo.ttf"),
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
}
