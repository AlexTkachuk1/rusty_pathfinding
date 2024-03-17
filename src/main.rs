use bevy::{
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
        .add_systems(Startup, setup_system)
        .add_systems(
            PostStartup,
            (init_grid_system, hex_tile_spawn_system).chain(),
        )
        .add_systems(Update, animate_sprite)
        .add_systems(Update, mouse_button_input_system)
        .run();
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: Query<&mut Window>,
) {
    commands.spawn(Camera2dBundle::default());

    let window = windows.single_mut();
    let (win_w, win_h) = (window.width(), window.height());
    let win_size = WindowSize { w: win_w, h: win_h };
    commands.insert_resource(win_size);

    let game_textures = GameTexture {
        hex_tile: asset_server.load(HEX_TILE),
    };

    commands.insert_resource(game_textures);
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
