<<<<<<< HEAD
use bevy::prelude::*;
=======
use bevy::{prelude::*};
>>>>>>> ba0493394022f23f80e46965118b005b401b1cff

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

<<<<<<< HEAD
#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);
=======
#[derive(Component)]
pub struct Movable {
    pub auto_despown: bool,
}

#[derive(Component)]
pub struct Laser;

#[derive(Component)]
pub struct SpriteSize(pub Vec2);

impl From<(f32, f32)> for SpriteSize {
    fn from(val: (f32, f32)) -> Self {
        SpriteSize(Vec2::new(val.0, val.1))
    }
}
// region: --- Player Component
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct FromPlayer;
// endregion: --- Player Component

// region: --- Enemy Component
#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct FromEnemy;
// endregion: --- Enemy Component

// region: --- Explosion Component
#[derive(Component)]
pub struct Explosion;

#[derive(Component)]
pub struct ExplosionToSpawn(pub Vec3);

#[derive(Component)]
pub struct ExplosionTimer(pub Timer);

impl Default for ExplosionTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.05, TimerMode::Repeating))
    }
}
// endregion: --- Explosion Component
>>>>>>> ba0493394022f23f80e46965118b005b401b1cff
