use bevy::prelude::*;

pub struct CleanerPlugin;

impl Plugin for CleanerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(clean_dead.label("cleanup"));
    }
}

#[derive(Component)]
pub struct Dead;

fn clean_dead(mut cmd: Commands, ded: Query<Entity, With<Dead>>) {
    ded.for_each(|x| cmd.entity(x).despawn());
}
