use std::os::windows::process;

use bevy::{
    prelude::*,
    render::render_resource::encase::rts_array::Length,
    window::{PrimaryWindow, WindowPosition, WindowTheme},
};
use components::{AnimationIndices, AnimationTimer, Hero};
use constants::{
    CHARACTER_ATLAS, CHARACTER_SIZE, HALF_SIZE, HEX_HEIGHT, HEX_TILE, HEX_TILE_SCALE,
    HEX_TILE_SIZE, QUARTER_SIZE,
};
use my_resourses::{
    Cell, CurrentHeroPosition, GameTexture, Grid, HeroPath, Node, PathCondition, PathConditions,
    Position, SimplifiedCell, TargetHeroPosition, WindowSize,
};
use rand::{thread_rng, Rng};

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
            (
                init_grid_system,
                hex_tile_spawn_system,
                character_spawn_system,
            )
                .chain(),
        )
        .add_systems(Update, animate_sprite)
        .add_systems(Update, mouse_button_input_system)
        .add_systems(Update, pathfinding_system.run_if(pathfinding_criteria))
        .add_systems(FixedUpdate, movement_system.run_if(movement_criteria))
        .insert_resource(Time::<Fixed>::from_seconds(0.02))
        .run();
}

fn pathfinding_criteria(path_condition: Res<PathCondition>) -> bool {
    path_condition.path_state == PathConditions::Search
}

fn movement_criteria(path_condition: Res<PathCondition>) -> bool {
    path_condition.path_state == PathConditions::Performance
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

    let path_condition = PathCondition {
        path_state: PathConditions::Sleep,
    };

    commands.insert_resource(path_condition);
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
    grid: Res<Grid>,
) {
    let mut rng: rand::prelude::ThreadRng = thread_rng();
    let rnd_number = rng.gen_range(0..grid.cells.length()) as f32;
    let rnd_cell_options = grid.cells.get(rnd_number as usize);

    match rnd_cell_options {
        Some(rnd_cell) => {
            let current_hero_position = CurrentHeroPosition {
                cell: SimplifiedCell {
                    x: rnd_cell.x,
                    y: rnd_cell.y,
                    pos_x: rnd_cell.pos_x,
                    pos_y: rnd_cell.pos_y,
                },
            };

            let hero_path = HeroPath {
                cells: vec![],
                current_target_cell: Position {
                    x: current_hero_position.cell.pos_x as i32,
                    y: current_hero_position.cell.pos_y as i32,
                },
            };

            commands.insert_resource(hero_path);

            let hero = Hero {
                id: 1.,
                active: true,
                speed: 1.,
            };

            let texture = asset_server.load(CHARACTER_ATLAS);
            let layout = TextureAtlasLayout::from_grid(Vec2::new(48.0, 36.0), 6, 1, None, None);
            let texture_atlas_layout = texture_atlas_layouts.add(layout);
            let animation_indices = AnimationIndices { first: 0, last: 5 };

            commands
                .spawn((
                    SpriteSheetBundle {
                        texture,
                        atlas: TextureAtlas {
                            layout: texture_atlas_layout,
                            index: animation_indices.first,
                        },
                        transform: Transform {
                            scale: Vec3::splat(1.4),
                            translation: Vec3::new(
                                current_hero_position.cell.pos_x,
                                current_hero_position.cell.pos_y,
                                1.,
                            ),
                            ..Default::default()
                        },
                        ..default()
                    },
                    animation_indices,
                    AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
                ))
                .insert(hero);

            let target_hero_position = TargetHeroPosition {
                cell: SimplifiedCell {
                    x: rnd_cell.x,
                    y: rnd_cell.y,
                    pos_x: rnd_cell.pos_x,
                    pos_y: rnd_cell.pos_y,
                },
            };

            commands.insert_resource(current_hero_position);
            commands.insert_resource(target_hero_position);
        }
        _ => {}
    };
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
    // mut commands: Commands,
    // asset_server: Res<AssetServer>,
    // mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    win_size: Res<WindowSize>,
    mut target_cell: ResMut<TargetHeroPosition>,
    mut path_condition: ResMut<PathCondition>,
    grid: Res<Grid>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(position) = q_windows.single().cursor_position() {
            let filtered_collection: Vec<&Cell> = grid
                .cells
                .iter()
                .filter(|&cell| {
                    cell.is_point_inside_cell((
                        position.x - win_size.w / 2.,
                        (win_size.h - position.y) - win_size.h / 2.,
                    ))
                })
                .collect();

            if filtered_collection.length() == 1 {
                let target_cell_option = filtered_collection.first();

                if let Some(new_target_cell) = target_cell_option {
                    if path_condition.path_state == PathConditions::Sleep {
                        target_cell.cell.x = new_target_cell.x;
                        target_cell.cell.y = new_target_cell.y;
                        target_cell.cell.pos_x = new_target_cell.pos_x;
                        target_cell.cell.pos_y = new_target_cell.pos_y;

                        path_condition.path_state = PathConditions::Search;
                    }

                    // let texture = asset_server.load(CHARACTER_ATLAS);
                    // let layout = TextureAtlasLayout::from_grid(
                    //     Vec2::new(CHARACTER_SIZE.0, CHARACTER_SIZE.1),
                    //     6,
                    //     1,
                    //     None,
                    //     None,
                    // );
                    // let texture_atlas_layout = texture_atlas_layouts.add(layout);
                    // let animation_indices = AnimationIndices { first: 0, last: 5 };

                    // commands.spawn((
                    //     SpriteSheetBundle {
                    //         texture,
                    //         atlas: TextureAtlas {
                    //             layout: texture_atlas_layout,
                    //             index: animation_indices.first,
                    //         },
                    //         transform: Transform {
                    //             scale: Vec3::splat(1.4),
                    //             translation: Vec3::new(new_target_cell.pos_x, new_target_cell.pos_y, 1.),
                    //             ..Default::default()
                    //         },
                    //         ..default()
                    //     },
                    //     animation_indices,
                    //     AnimationTimer(Timer::from_seconds(0.15, TimerMode::Repeating)),
                    // ));
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
                left_pos
                    + (HEX_TILE_SIZE.0 / 2f32 * HEX_TILE_SCALE)
                    + (HEX_TILE_SIZE.0 * HEX_TILE_SCALE * x as f32)
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

fn movement_system(
    time: Res<Time>,
    mut sprite: Query<(&mut Hero, &mut Transform, &mut Sprite)>,
    mut hero_path: ResMut<HeroPath>,
    mut path_condition: ResMut<PathCondition>,
    mut current_cell: ResMut<CurrentHeroPosition>,
    target_cell: Res<TargetHeroPosition>,
) {
    for (hero, mut transform, mut sprite) in &mut sprite {
        let current_distance_x = hero_path.current_target_cell.x as f32 - current_cell.cell.pos_x;
        let current_distance_y = hero_path.current_target_cell.y as f32 - current_cell.cell.pos_y;
        let real_distance_x = (hero_path.current_target_cell.x as f32 - transform.translation.x).abs();
        let real_distance_y = (hero_path.current_target_cell.y as f32 - transform.translation.y).abs();

        if hero.active
            && (real_distance_x.clone() > 1. || real_distance_y.clone() > 1.)
        {
            if real_distance_x.clone() > 1. {
                transform.translation.x += current_distance_x.clone() / 20.;
                sprite.flip_x = current_distance_x.clone() < 0.;
            }

            if real_distance_y.clone() > 1. {
                transform.translation.y += current_distance_y.clone() / 20.;
            }
        } else {
            if hero_path.cells.length() > 0 {
                hero_path.cells.pop();
                if let Some(position) = hero_path.cells.last() {
                    hero_path.current_target_cell = position.clone();

                    current_cell.cell.pos_x = transform.translation.x;
                    current_cell.cell.pos_y = transform.translation.y;
                }
            } else {
                current_cell.cell.x = target_cell.cell.x;
                current_cell.cell.y = target_cell.cell.y;
                current_cell.cell.pos_x = target_cell.cell.pos_x;
                current_cell.cell.pos_y = target_cell.cell.pos_y;

                path_condition.path_state = PathConditions::Sleep;

                if sprite.flip_x {
                    sprite.flip_x = false;
                }
            }
        }
    }
}

fn pathfinding_system(
    target_cell: Res<TargetHeroPosition>,
    current_cell: Res<CurrentHeroPosition>,
    grid: Res<Grid>,
    mut path_condition: ResMut<PathCondition>,
    mut hero_path: ResMut<HeroPath>,
) {
    let start_position = Position {
        x: current_cell.cell.x as i32,
        y: current_cell.cell.y as i32,
    };

    let destination = Position {
        x: target_cell.cell.x as i32,
        y: target_cell.cell.y as i32,
    };

    let mut openlist: std::collections::HashMap<Position, Node> = std::collections::HashMap::new();
    let mut closedlist: std::collections::HashMap<Position, Node> =
        std::collections::HashMap::new();

    openlist.insert(
        start_position.clone(),
        Node {
            position: start_position.clone(),
            g: 0,
            h: 0,
            f: 0,
            parent: None,
        },
    );

    while !openlist.is_empty() {
        let mut current_node = None;
        let mut lowest_f = -1;
        let mut lowest_g = -1;

        for (position, node) in openlist.iter() {
            if lowest_f == -1 || node.f < lowest_f || (node.f == lowest_f && node.g < lowest_g) {
                // println!("{} {}", node.position.x.clone(), node.position.y.clone());
                lowest_f = node.f;
                lowest_g = node.g;
                current_node = Some(node);
            }
        }

        if let None = current_node {
            break;
        }

        let current_node = current_node.unwrap().clone();

        let mut current_position = current_node.position.clone();
        openlist.remove(&current_position);
        // Add n to the CLOSED list
        let g = current_node.g + 1;
        let h = current_position.distance(&destination);
        let f = g + h;
        closedlist.insert(
            current_position.clone(),
            Node {
                position: current_position.clone(),
                g: g,
                h: h,
                f: f,
                parent: current_node.parent,
            },
        );
        // IF n is the same as the goal, we have a solution. Backtrack to find the path.
        if current_position == destination {
            let mut nodelist: Vec<Position> = vec![];

            while let Some(node) = closedlist.get(&current_position) {
                let cell_position = current_position.clone().get_world_position(grid.cells.clone());
                nodelist.push(cell_position);
            
                if let Some(parent) = &node.parent {
                    current_position = parent.clone();
                } else {
                    break;
                }
            }
            

            if let Some(target_cell) = nodelist.last().cloned() {
                hero_path.cells = nodelist;
                hero_path.current_target_cell = target_cell.clone();

                path_condition.path_state = PathConditions::Performance;
            }

            break;
        }
        let mut neighbors: Vec<Position> = vec![];

        let correction_factor: i32 = if (current_position.y + 1i32) % 2i32 == 0i32 {
            -1i32
        } else {
            1i32
        };

        let neighbors_cords = vec![
            (current_position.x - 1i32, current_position.y),
            (current_position.x + 1i32, current_position.y),
            (current_position.x, current_position.y - 1i32),
            (current_position.x, current_position.y + 1i32),
            (
                current_position.x + correction_factor,
                current_position.y + 1i32,
            ),
            (
                current_position.x + correction_factor,
                current_position.y + 1i32,
            ),
        ];

        for neighbor in neighbors_cords.iter() {
            if neighbor.0 >= 0i32 && neighbor.1 >= 0i32 && neighbor.0 < 18i32 && neighbor.1 < 12i32
            {
                neighbors.push(Position {
                    x: neighbor.0,
                    y: neighbor.1,
                });
            }
        }

        for neighbor in neighbors {
            // This checks if the space is enterable. Change to whatever
            // determines "enterability" in your code.
            // if tiletypes.get(&neighbor).unwrap().clone() == TileType::Wall {
            //     continue;
            // }
            let h = neighbor.distance(&destination);
            let g = current_node.g + 1;
            let f = g + h;
            if openlist.contains_key(&neighbor) {
                if g > openlist.get(&neighbor).unwrap().g {
                    continue;
                }
            }
            if closedlist.contains_key(&neighbor) {
                if g > closedlist.get(&neighbor).unwrap().g {
                    continue;
                }
            }
            openlist.remove(&neighbor);
            closedlist.remove(&neighbor);
            openlist.insert(
                neighbor.clone(),
                Node {
                    position: neighbor.clone(),
                    g: g,
                    h: h,
                    f: f,
                    parent: Some(current_position.clone()),
                },
            );
        }
    }
}
