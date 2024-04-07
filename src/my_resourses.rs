use std::hash::Hash;

use bevy::{prelude::*};

pub struct WindowSize {
    pub w: f32,
    pub h: f32,
}

impl Resource for WindowSize {}

pub struct GameTexture {
    pub hex_tile: Handle<Image>,
}

impl Resource for GameTexture {}

#[derive(Clone)]
pub struct Cell {
    pub x: f32,
    pub y: f32,
    pub pos_x: f32,
    pub pos_y: f32,
    pub is_walkable: bool,
    pub half_size: f32,
    pub upper_right_corner: (f32, f32),
    pub upper_left_corner: (f32, f32),
    pub lower_right_corner: (f32, f32),
    pub lower_left_corner: (f32, f32),
    pub upper_corner: (f32, f32),
    pub lower_corner: (f32, f32),
}

impl Cell {
    pub fn is_point_inside_cell(&self, mouse_position: (f32, f32)) -> bool {
        if mouse_position.0 > self.upper_left_corner.0
            && mouse_position.0 < self.upper_right_corner.0
        {
            if mouse_position.1 < self.upper_corner.1 && mouse_position.1 > self.lower_corner.1 {
                let max_x_for_point_y = self.pos_x
                    + self.half_size
                        * (1.
                            - ((mouse_position.1 - self.upper_right_corner.1)
                                / (self.upper_corner.1 - self.upper_right_corner.1)));
                let min_x_for_point_y = self.pos_x
                    - self.half_size
                        * (1.
                            - ((mouse_position.1 - self.upper_right_corner.1)
                                / (self.upper_corner.1 - self.upper_right_corner.1)));

                if mouse_position.1 > self.upper_right_corner.1
                    && mouse_position.0 < max_x_for_point_y
                    && mouse_position.0 > min_x_for_point_y
                {
                    return true;
                } else if mouse_position.1 > self.upper_right_corner.1 {
                    return false;
                }

                let max_x_for_point_y = self.pos_x
                    + self.half_size
                        * (1.
                            - ((self.lower_right_corner.1 - mouse_position.1)
                                / (self.lower_right_corner.1 - self.lower_corner.1)));
                let min_x_for_point_y = self.pos_x
                    - self.half_size
                        * (1.
                            - ((self.lower_right_corner.1 - mouse_position.1)
                                / (self.lower_right_corner.1 - self.lower_corner.1)));

                if mouse_position.1 < self.lower_left_corner.1
                    && mouse_position.0 < max_x_for_point_y
                    && mouse_position.0 > min_x_for_point_y
                {
                    return true;
                } else if mouse_position.1 < self.lower_left_corner.1 {
                    return false;
                }

                return true;
            }
        }

        false
    }
}


pub struct SimplifiedCell {
    pub x: f32,
    pub y: f32,
    pub pos_x: f32,
    pub pos_y: f32,
}

impl Clone for SimplifiedCell {
    fn clone(&self) -> Self {
        Self {
            x: self.x, 
            y: self.y,
            pos_x: self.pos_x,
            pos_y: self.pos_y,
        }
    }
}

impl SimplifiedCell {}

pub struct Grid {
    pub cells: Vec<Cell>,
}

impl Resource for Grid {}

pub struct CurrentHeroPosition {
    pub cell: SimplifiedCell,
}

impl Resource for CurrentHeroPosition {}

pub struct TargetHeroPosition {
    pub cell: SimplifiedCell,
}

impl Resource for TargetHeroPosition {}

pub struct HeroPath {
    pub cells: Vec<Position>,
    pub current_target_cell: Position,
}

impl Resource for HeroPath {}

pub struct PathCondition {
    pub path_state: PathConditions
}

impl Resource for PathCondition {}

#[derive(PartialEq)]
pub enum PathConditions{
    Search,
    Performance,
    Sleep,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Node {
    pub position: Position,
    pub g: i32,
    pub h: i32,
    pub f: i32,
    pub parent: Option<Position>,
}

pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn distance(&self, destination: &Position) -> i32 {
        let distance_x = (self.x - destination.x).abs();
        let distance_y = (self.y - destination.y).abs();

        distance_x + distance_y
    }

    pub fn get_world_position(&self, cells: Vec<Cell>) -> Position {
        let mut result = Position { 
            x: 0i32, 
            y: 0i32, 
        };

        match cells.iter().find(|cell| cell.x == self.x as f32 && cell.y == self.y as f32) {
            Some(cell) => {
                result.x = cell.pos_x as i32;
                result.y = cell.pos_y as i32;
            },
            None => panic!("Совпадение не найдено!"),
        }

        result
    }
}

impl Clone for Position {
    fn clone(&self) -> Self {
        Self {
            x: self.x, 
            y: self.y,
        }
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Position {}

impl Hash for Position {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}