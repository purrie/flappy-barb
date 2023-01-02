use bevy::prelude::*;

#[derive(Component)]
pub struct VerticalMove {
    pub speed: f32,
}

impl VerticalMove {
    pub const JUMP_STRENGTH: f32 = 500.;
    pub const GRAVITY_STRENGTH: f32 = 2.;
    pub const MAX_GRAVITY: f32 = -500.;
}

pub struct GravityPlugin;

impl Plugin for GravityPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(gravity_system);
    }
}

fn gravity_system(time: Res<Time>, mut affected: Query<(&mut VerticalMove, &mut Transform)>) {
    for (mut mov, mut trans) in &mut affected {
        if mov.speed > VerticalMove::MAX_GRAVITY {
            let speed = VerticalMove::GRAVITY_STRENGTH * time.delta_seconds();
            mov.speed = mov.speed * (1.0 - speed) + (VerticalMove::MAX_GRAVITY * speed);
        }

        trans.translation.y += mov.speed * time.delta_seconds();
    }
}
