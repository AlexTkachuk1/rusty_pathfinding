use bevy::prelude::*;

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct Hero {
    pub id: f32,
    pub active: bool,
    pub speed: f32,
}

impl Default for Hero {
    fn default() -> Self {
        Self {
            id: 1.,
            active: false,
            speed: 1.
        }
    }
}
