use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

use crate::{
    cleanup::Dead,
    obstacles::Obstacle,
    player::{AttackState, Player},
};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>()
            .add_system(collision_detection);
    }
}

pub struct CollisionEvent {
    pub collision: Collision,
    pub player_state: AttackState,
    pub player: Entity,
    pub obstacle: Entity,
}

fn collision_detection(
    mut sender: EventWriter<CollisionEvent>,
    player: Query<(&Sprite, &Transform, &Player, Entity)>,
    obstacles: Query<(&Sprite, &Transform, Entity), (With<Obstacle>, Without<Dead>)>,
) {
    let Ok(pl) = player.get_single() else {
        return;
    };
    let p_pos = pl.1.translation;
    let p_size = pl.0.custom_size.unwrap();

    obstacles
        .iter()
        .filter_map(|x| {
            let o_pos = x.1.translation;
            let o_size = x.0.custom_size.unwrap();
            let Some(c) = collide(o_pos, o_size, p_pos, p_size) else {
            return None;
        };
            Some((c, x.2))
        })
        .for_each(|x| {
            let ev = CollisionEvent {
                collision: x.0,
                player_state: pl.2.attack_state.clone(),
                player: pl.3,
                obstacle: x.1,
            };
            sender.send(ev);
        });
}
