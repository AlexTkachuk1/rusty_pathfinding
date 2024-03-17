use bevy::{
<<<<<<< HEAD
    prelude::*,
    render::render_resource::encase::rts_array::Length,
    window::{PrimaryWindow, WindowPosition, WindowTheme},
};
use components::{AnimationIndices, AnimationTimer};
use constants::{
    CHARACTER_ATLAS, HALF_SIZE, HEX_HEIGHT, HEX_TILE, HEX_TILE_SCALE, HEX_TILE_SIZE, QUARTER_SIZE,
};
use my_resourses::{Cell, GameTexture, Grid, WindowSize};

use crate::constants::CHARACTER_SIZE;

mod components;
mod constants;
mod my_resourses;
=======
    ecs::query, prelude::*, sprite::collide_aabb::collide, utils::hashbrown::HashSet, window::{WindowPosition, WindowTheme}
};
use components::{Enemy, Explosion, ExplosionTimer, ExplosionToSpawn, FromEnemy, FromPlayer, Laser, Movable, Player, SpriteSize, Velocity};
use enemy::EnemyPlugin;
use player::PlayerPlugin;

mod components;
mod enemy;
mod player;

// region: --- Assets Constants
const PLAYER_SPRITE: &str = "../assets/hero.png";
const PLAYER_SIZE: (f32, f32) = (1024f32, 1024f32);

const ENEMY_SPRITE: &str = "../assets/enemy.png";
const ENEMY_SIZE: (f32, f32) = (103f32, 84f32);

const PLAYER_LASER_SPRITE: &str = "../assets/laser.png";
const PLAYER_LASER_SIZE: (f32, f32) = (9f32, 54f32);

const ENEMY_LASER_SPRITE: &str = "../assets/enemyLaser.png";
const ENEMY_LASER_SIZE: (f32, f32) = (9f32, 37f32);

const EXPLOSION_SHEET: &str = "../assets/explosion.png";
const EXPLOSION_LEN: usize = 16;
// endregion: --- Assets Constants

// region: --- Game Constants
const TIME_STEP: f32 = 1f32 / 60f32;
const BASE_SPEED: f32 = 200f32;

const PLAYER_RESPAWN_DELAY: f64 = 2.;
const ENEMY_MAX: u32 = 2;

const FORMATION_MEMBERS_MAX: u32 = 2;
// endregion: --- Game Constants

// region: --- Resourses
pub struct WindowSize {
    pub w: f32,
    pub h: f32,
}

impl Resource for WindowSize {}

struct GameTexture {
    player: Handle<Image>,
    player_laser: Handle<Image>,
    enemy: Handle<Image>,
    enemy_laser: Handle<Image>,
    explosion: Handle<TextureAtlas>,
}

impl Resource for GameTexture {}

struct EnemyCount (u32);

impl Resource for EnemyCount {}

struct PlayerState {
    on: bool, // alive
    last_shot: f64, // -1 if not shot
}

impl Resource for PlayerState {}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            on: false,
            last_shot: -1.,
        }
    }
}

impl PlayerState {
    pub fn shot(&mut self, time: f64) {
        self.on = false;
        self.last_shot = time;
    }

    pub fn spawned(&mut self) {
        self.on = true;
        self.last_shot = -1.;
    }
}
// endregion: --- Resourses
>>>>>>> ba0493394022f23f80e46965118b005b401b1cff

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.02, 0.02, 0.02)))
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Welcom to Rusty".into(),
                position: WindowPosition::At(IVec2::new(300, 200)),
                window_theme: Some(WindowTheme::Dark),
                enabled_buttons: bevy::window::EnabledButtons {
                    maximize: false,
                    ..Default::default()
                },
                visible: true,
                resizable: false,
                ..default()
            }),
            ..default()
        }),))
<<<<<<< HEAD
        .add_systems(Startup, setup_system)
        .add_systems(
            PostStartup,
            (init_grid_system, hex_tile_spawn_system).chain(),
        )
        .add_systems(Update, animate_sprite)
        .add_systems(Update, mouse_button_input_system)
=======
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .add_systems(Startup, setup_system)
        .add_systems(Update, movable_system)
        .add_systems(Update, player_laser_hit_enemy_system)
        .add_systems(Update, enemy_laser_hit_player_system)
        .add_systems(Update, explosion_to_spawn_system)
        .add_systems(Update, explosion_animation_system)
>>>>>>> ba0493394022f23f80e46965118b005b401b1cff
        .run();
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
<<<<<<< HEAD
=======
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
>>>>>>> ba0493394022f23f80e46965118b005b401b1cff
    mut windows: Query<&mut Window>,
) {
    commands.spawn(Camera2dBundle::default());

    let window = windows.single_mut();
    let (win_w, win_h) = (window.width(), window.height());
    let win_size = WindowSize { w: win_w, h: win_h };
    commands.insert_resource(win_size);

    let texture_handle = asset_server.load(EXPLOSION_SHEET);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(64f32, 64f32),
        4,
        4,
        Some(Vec2::new(0f32, 0f32)), 
        Some(Vec2::new(0f32, 0f32))
    );
    let explosion = texture_atlases.add(texture_atlas);


    let game_textures = GameTexture {
<<<<<<< HEAD
        hex_tile: asset_server.load(HEX_TILE),
=======
        player: asset_server.load(PLAYER_SPRITE),
        player_laser: asset_server.load(PLAYER_LASER_SPRITE),
        enemy: asset_server.load(ENEMY_SPRITE),
        enemy_laser: asset_server.load(ENEMY_LASER_SPRITE),
        explosion,
>>>>>>> ba0493394022f23f80e46965118b005b401b1cff
    };
    commands.insert_resource(game_textures);
    commands.insert_resource(EnemyCount(0));
}

fn movable_system(
    mut commands: Commands,
    win_size: Res<WindowSize>,
    mut query: Query<(Entity, &Velocity, &mut Transform, &Movable)>,
) {
    for (entity, velocity, mut transform, movable) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;

        if movable.auto_despown {
            const MARGIN: f32 = 200f32;
            if translation.y > win_size.h / 2f32 + MARGIN
                || translation.y < -win_size.h / 2f32 - MARGIN
                || translation.x > win_size.w / 2f32 + MARGIN
                || translation.x < -win_size.w / 2f32 - MARGIN
            {
                commands.entity(entity).despawn();
            }
        }
    }
}

fn player_laser_hit_enemy_system(
    mut commands: Commands,
    mut enemy_count: ResMut<EnemyCount>,
    laser_query: Query<(Entity, &Transform, &SpriteSize), (With<Laser>, With<FromPlayer>)>,
    enemy_query: Query<(Entity, &Transform, &SpriteSize), With<Enemy>>,
) {
    let mut despawned_entities: HashSet<Entity> = HashSet::new();

    for (laser_entity, laser_tf, laser_size) in laser_query.iter() {
        if despawned_entities.contains(&laser_entity) {
            continue;
        };

        let laser_scale = Vec2::from(laser_tf.scale.xy());

        for (enemy_entity, enemy_tf, enemy_size) in enemy_query.iter() {
            if despawned_entities.contains(&enemy_entity)
            || despawned_entities.contains(&laser_entity) {
                continue;
            }

            let enemy_scale = Vec2::from(enemy_tf.scale.xy());

            let collision = collide(
                laser_tf.translation,
                laser_size.0 * laser_scale,
                enemy_tf.translation,
                enemy_size.0 * enemy_scale,
            );

            if let Some(_) = collision {
                commands.entity(enemy_entity).despawn();
                despawned_entities.insert(enemy_entity);
                enemy_count.0 -= 1;

                commands.entity(laser_entity).despawn();
                despawned_entities.insert(laser_entity);

                commands.spawn(ExplosionToSpawn(enemy_tf.translation.clone()));
            }
        }
    }
}

fn enemy_laser_hit_player_system(
    mut commands: Commands,
    mut player_state: ResMut<PlayerState>,
    time: Res<Time>,
    laser_query: Query<(Entity, &Transform, &SpriteSize), (With<Laser>, With<FromEnemy>)>,
    player_query: Query<(Entity, &Transform, &SpriteSize), With<Player>>,
) {
    if let Ok((player_entity, player_tf, player_size)) = player_query.get_single() {
        let player_scale = Vec2::from(player_tf.scale.xy());

        for (laser_entity, laser_tf, laser_size) in laser_query.iter() {
            let laser_scale = Vec2::from(laser_tf.scale.xy());

            let collision = collide(
                laser_tf.translation,
                laser_scale * laser_size.0,
                player_tf.translation,
                player_scale * player_size.0);

            if let Some(_) = collision {
                commands.entity(player_entity).despawn();
                player_state.shot(time.elapsed_seconds_f64());

                commands.entity(laser_entity).despawn();

                commands.spawn(ExplosionToSpawn(player_tf.translation.clone()));

                break;
            }
        }
    };
}

fn explosion_to_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTexture>,
    query: Query<(Entity, &ExplosionToSpawn)>
) {
    for (explosion_spawn_entity, explosion_to_spawn) in query.iter() {
        commands.spawn(SpriteSheetBundle {
            texture_atlas: game_textures.explosion.clone(),
            transform: Transform {
                translation: explosion_to_spawn.0,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Explosion)
        .insert(ExplosionTimer::default());

        commands.entity(explosion_spawn_entity).despawn();
    }
}

fn explosion_animation_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ExplosionTimer, &mut TextureAtlasSprite), With<Explosion>>,
) {
    for (entity, mut timer, mut sprite) in query.iter_mut() {
        timer.0.tick(time.delta());

        if timer.0.finished() {
            sprite.index += 1;

            if sprite.index >= EXPLOSION_LEN {
                commands.entity(entity).despawn();
            }
        }
    }
}

fn hex_tile_spawn_system(mut commands: Commands, game_textures: Res<GameTexture>, grid: Res<Grid>) {
    for cell in grid.cells.iter() {
        commands.spawn(SpriteBundle {
            texture: game_textures.hex_tile.clone(),
            transform: Transform {
                scale: Vec3::new(HEX_TILE_SCALE, HEX_TILE_SCALE, 1.0),
                translation: Vec3::new(cell.pos_x, cell.pos_y, 0.),
                ..Default::default()
            },
            ..default()
        });
    }
}

fn character_spawn_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load(CHARACTER_ATLAS);
    let layout = TextureAtlasLayout::from_grid(Vec2::new(48.0, 36.0), 6, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices { first: 0, last: 5 };

    commands.spawn((
        SpriteSheetBundle {
            texture,
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
            transform: Transform::from_scale(Vec3::splat(1.5)),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}

fn mouse_button_input_system(
    buttons: Res<ButtonInput<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    win_size: Res<WindowSize>,
    grid: Res<Grid>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(position) = q_windows.single().cursor_position() {
            let filtered_collection: Vec<&Cell> = grid
                .cells
                .iter()
                .filter(|&cell| cell.is_point_inside_cell((position.x - win_size.w / 2., (win_size.h - position.y) - win_size.h / 2.)))
                .collect();

            if filtered_collection.length() == 1 {
                let target_cell_option = filtered_collection.first();

                if let Some(target_cell) = target_cell_option {
                    let texture = asset_server.load(CHARACTER_ATLAS);
                    let layout =
                        TextureAtlasLayout::from_grid(Vec2::new(CHARACTER_SIZE.0, CHARACTER_SIZE.1), 6, 1, None, None);
                    let texture_atlas_layout = texture_atlas_layouts.add(layout);
                    let animation_indices = AnimationIndices { first: 0, last: 5 };

                    commands.spawn((
                        SpriteSheetBundle {
                            texture,
                            atlas: TextureAtlas {
                                layout: texture_atlas_layout,
                                index: animation_indices.first,
                            },
                            transform: Transform {
                                scale: Vec3::splat(1.4),
                                translation: Vec3::new(
                                    target_cell.pos_x,
                                    target_cell.pos_y,
                                    1.),
                                ..Default::default()
                            },
                            ..default()
                        },
                        animation_indices,
                        AnimationTimer(Timer::from_seconds(0.15, TimerMode::Repeating)),
                    ));
                } else {
                    println!("Target cell not found");
                }
            }
        }
    }
}

fn init_grid_system(mut commands: Commands, win_size: Res<WindowSize>) {
    let mut cells = Vec::<Cell>::new();
    let left_pos = -win_size.w / 2f32;
    let top_pos = win_size.h / 2f32;

    for x in 0..18 {
        for y in 0..12 {
            let pos_x = if y % 2 == 0 {
                left_pos + (HEX_TILE_SIZE.0 / 2f32 * HEX_TILE_SCALE) + (HEX_TILE_SIZE.0 * HEX_TILE_SCALE * x as f32)
            } else {
                left_pos
                    + (HEX_TILE_SIZE.0 / 2f32 * HEX_TILE_SCALE)
                    + (HEX_TILE_SIZE.0 * HEX_TILE_SCALE * x as f32)
                    + HEX_TILE_SIZE.0 * HEX_TILE_SCALE * 0.5
            };

            let pos_y = top_pos
                - (HEX_TILE_SIZE.1 / 2f32 * HEX_TILE_SCALE)
                - (HEX_TILE_SIZE.1 * HEX_TILE_SCALE * 0.5 * y as f32)
                - (18f32 * y as f32);

            let cell: Cell = Cell {
                x: x as f32,
                y: y as f32,
                pos_x,
                pos_y,
                is_walkable: true,
                half_size: HALF_SIZE,
                upper_corner: (pos_x, pos_y + HEX_HEIGHT / 2.),
                lower_corner: (pos_x, pos_y - HEX_HEIGHT / 2.),
                upper_right_corner: (pos_x + HALF_SIZE, pos_y + QUARTER_SIZE),
                upper_left_corner: (pos_x - HALF_SIZE, pos_y + QUARTER_SIZE),
                lower_right_corner: (pos_x + HALF_SIZE, pos_y - QUARTER_SIZE),
                lower_left_corner: (pos_x - HALF_SIZE, pos_y - QUARTER_SIZE),
            };
            cells.push(cell);
        }
    }

    let grid = Grid { cells: cells };

    commands.insert_resource(grid);
}
