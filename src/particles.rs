use std::time::Duration;

use bevy::prelude::*;

use crate::physics::Movement;

const PARTICLE_COUNT: u32 = 500;

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_particles)
            .add_system(particle_opacity)
            .add_system(particle_lifetime.before(particle_expire))
            .add_system(particle_expire)
            .add_system(emit_particles);
    }
}

pub enum EmissionDirection {
    Local(Vec2),
    Global(Vec2),
}

#[derive(Component)]
pub struct ParticleEmitter {
    emit_rate: u8,
    interval: Timer,
    color: Color,
    size: Vec2,
    direction: EmissionDirection,
    speed: f32,
}

impl Default for ParticleEmitter {
    fn default() -> Self {
        Self {
            emit_rate: 1,
            interval: Timer::new(Duration::new(0, 0), TimerMode::Once),
            color: Color::WHITE,
            size: Vec2 { x: 8.0, y: 8.0 },
            direction: EmissionDirection::Global(Vec2::Y),
            speed: 500.0,
        }
    }
}

impl ParticleEmitter {
    pub fn new(rate: u8, interval: Duration, mode: TimerMode) -> Self {
        Self {
            emit_rate: rate,
            interval: Timer::new(interval, mode),
            ..default()
        }
    }
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
    pub fn with_direction(mut self, direction: EmissionDirection) -> Self {
        self.direction = direction;
        self
    }
}

#[derive(Component)]
struct InactiveParticle;
#[derive(Component)]
struct ActiveParticle {
    lifetime: f32,
    alive_for: f32,
}

impl Default for ActiveParticle {
    fn default() -> Self {
        Self {
            lifetime: 0.5,
            alive_for: Default::default(),
        }
    }
}

fn spawn_particles(mut cmd: Commands) {
    (0..PARTICLE_COUNT).for_each(|_| {
        cmd.spawn((
            SpriteBundle {
                visibility: Visibility { is_visible: false },
                ..default()
            },
            InactiveParticle,
        ));
    })
}

fn particle_opacity(mut particles: Query<(&mut Sprite, &ActiveParticle)>) {
    particles.for_each_mut(|(mut sprite, particle)| {
        let progress = {
            let half_life = particle.lifetime / 2.0;
            let lifetime = (particle.alive_for - half_life).max(0.0);
            lifetime / half_life
        };
        sprite.color.set_a(1.0 - progress);
    })
}

fn particle_expire(mut cmd: Commands, particles: Query<(Entity, &ActiveParticle)>) {
    particles.for_each(|(entity, particle)| {
        if particle.alive_for > particle.lifetime {
            cmd.entity(entity)
                .remove::<ActiveParticle>()
                .remove::<Movement>()
                .insert(InactiveParticle)
                .insert(Visibility { is_visible: false });
        }
    })
}

fn particle_lifetime(time: Res<Time>, mut particles: Query<&mut ActiveParticle>) {
    particles.for_each_mut(|mut particle| {
        particle.alive_for += time.delta_seconds();
    })
}

fn emit_particles(
    mut cmd: Commands,
    time: Res<Time>,
    mut emiters: Query<(&mut ParticleEmitter, &Transform), Without<InactiveParticle>>,
    mut particles: Query<(Entity, &InactiveParticle, &mut Sprite, &mut Transform)>,
) {
    let mut particles = particles.iter_mut();
    emiters.for_each_mut(|(mut emiter, transform)| {
        if emiter.interval.tick(time.delta()).just_finished() {
            (0..emiter.emit_rate).for_each(|_| {
                let Some(mut p) = particles.next() else {
                    return;
                };
                let mut cmd = cmd.entity(p.0);
                cmd.remove::<InactiveParticle>()
                    .insert(ActiveParticle::default());

                p.2.color = emiter.color.clone();
                p.2.custom_size = Some(emiter.size.clone());

                let (x, y) = {
                    let dir = match &emiter.direction {
                        EmissionDirection::Local(d) => {
                            let t = transform.rotation
                                * Vec3 {
                                    x: d.x,
                                    y: d.y,
                                    z: 0.0,
                                };
                            Vec2 { x: t.x, y: t.y }.normalize()
                        }

                        EmissionDirection::Global(d) => d.normalize(),
                    };
                    let random_angle = rand::random::<f32>() - 0.5;
                    let dir = Vec2::from_angle(random_angle).rotate(dir) * emiter.speed;
                    (dir.x, dir.y)
                };
                cmd.insert(Movement { x, y });
                cmd.insert(Visibility { is_visible: true });
                p.3.translation = transform.translation;
            })
        }
    })
}
