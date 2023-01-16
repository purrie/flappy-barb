use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

use crate::{
    cleanup::Dead,
    obstacles::{Obstacle, ObstacleKind},
    player::{AttackState, Player},
};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>()
            .add_event::<ProjectileCollisionEvent>()
            .add_system(gravity.label("gravity").before("movement"))
            .add_system(face_movement_direction.after("gravity"))
            .add_system(
                move_bodies
                    .label("movement")
                    .before("collision")
                    .before("projectiles"),
            )
            .add_system(collision_detection.label("collision"))
            .add_system(projectile_collision.label("projectiles"));
    }
}

#[derive(Component)]
pub struct Collider;

#[derive(Component)]
pub struct Projectile;

#[derive(Component, Default)]
pub struct Movement {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct FaceMovementDirection {
    /// The direction sprite faces naturally
    pub neutral: Vec2,
}

#[derive(Component, Deref, DerefMut)]
pub struct Gravity {
    pub strength: Vec2,
}

impl Default for Gravity {
    fn default() -> Self {
        Self {
            strength: Vec2 { x: 1., y: 2. },
        }
    }
}

impl Gravity {
    pub const MAX_GRAVITY: f32 = -500.;
}

pub struct CollisionEvent {
    pub collision: Collision,
    pub player_state: AttackState,
    pub player: Entity,
    pub player_pos: Vec3,
    pub obstacle: Entity,
    pub obstacle_pos: Vec3,
    pub obstacle_kind: ObstacleKind,
}

pub struct ProjectileCollisionEvent {
    pub projectile_pos: Vec3,
    pub hit: Entity,
    pub hit_pos: Vec3,
    pub hit_kind: ObstacleKind,
}

fn collision_detection(
    mut sender: EventWriter<CollisionEvent>,
    player: Query<(&Sprite, &Transform, &Player, Entity)>,
    obstacles: Query<(&Sprite, &Transform, Entity, &Obstacle), (With<Collider>, Without<Dead>)>,
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
            Some((c, x.2, x.1, x.3))
        })
        .for_each(|x| {
            let ev = CollisionEvent {
                collision: x.0,
                player_state: pl.2.attack_state.clone(),
                player: pl.3,
                player_pos: p_pos,
                obstacle: x.1,
                obstacle_pos: x.2.translation,
                obstacle_kind: x.3.kind.clone(),
            };
            sender.send(ev);
        });
}

fn projectile_collision(
    mut sender: EventWriter<ProjectileCollisionEvent>,
    projectiles: Query<(&Sprite, &Transform), With<Projectile>>,
    obstacles: Query<(&Sprite, &Transform, Entity, &Obstacle), (With<Collider>, Without<Dead>)>,
) {
    projectiles.for_each(|(pro_sprite, pro_transform)| {
        let pro_pos = pro_transform.translation;
        let pro_size = pro_sprite.custom_size.unwrap();
        obstacles.for_each(|(obs_sprite, obs_transform, obs_entity, obstacle)| {
            let obs_pos = obs_transform.translation;
            let obs_size = obs_sprite.custom_size.unwrap();
            if let Some(_) = collide(obs_pos, obs_size, pro_pos, pro_size) {
                sender.send(ProjectileCollisionEvent {
                    projectile_pos: pro_pos,
                    hit: obs_entity.clone(),
                    hit_pos: obs_pos,
                    hit_kind: obstacle.kind.clone(),
                })
            }
        })
    })
}

fn move_bodies(time: Res<Time>, mut bodies: Query<(&Movement, &mut Transform)>) {
    bodies.for_each_mut(|mut o| {
        let (x, y) = (o.0.x * time.delta_seconds(), o.0.y * time.delta_seconds());
        o.1.translation.x += x;
        o.1.translation.y += y;
    })
}

fn gravity(time: Res<Time>, mut affected: Query<(&mut Movement, &Gravity)>) {
    affected.for_each_mut(|mut o| {
        let speed = Vec2 {
            x: o.1.x * time.delta_seconds(),
            y: o.1.y * time.delta_seconds(),
        };
        o.0.x = o.0.x * (1.0 - speed.x);
        o.0.y = o.0.y * (1.0 - speed.y) + (Gravity::MAX_GRAVITY * speed.y);
    });
}

fn face_movement_direction(mut bodies: Query<(&mut Transform, &Movement, &FaceMovementDirection)>) {
    bodies.for_each_mut(|mut o| {
        let dir = Vec2 { x: o.1.x, y: o.1.y }.normalize();
        let neutral = o.2.neutral;
        let angle = neutral.angle_between(dir);
        o.0.rotation = Quat::from_euler(EulerRot::XYZ, 0., 0., angle);
    })
}
