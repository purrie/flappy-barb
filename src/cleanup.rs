use bevy::prelude::*;

pub struct CleanerPlugin;

impl Plugin for CleanerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(clean_dead.label("cleanup"));
    }
}

#[derive(Component, Default)]
pub struct Dead {
    pub timer: f32,
}

fn clean_dead(mut cmd: Commands, mut ded: Query<(Entity, &mut Dead)>, time: Res<Time>) {
    ded.for_each_mut(|mut x| {
        x.1.timer -= time.delta_seconds();
        if x.1.timer <= 0.0 {
            cmd.entity(x.0).despawn_recursive()
        }
    });
}
