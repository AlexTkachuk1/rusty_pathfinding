use bevy::prelude::*;

pub struct WindowSize {
    pub w: f32,
    pub h: f32,
}

impl Resource for WindowSize {}

pub struct GameTexture {
    pub hex_tile: Handle<Image>,
}

impl Resource for GameTexture {}

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
            if mouse_position.1 < self.upper_corner.1 
            && mouse_position.1 > self.lower_corner.1 {

                let max_x_for_point_y = self.pos_x
                    + self.half_size * (1. - ((mouse_position.1 - self.upper_right_corner.1)
                    / (self.upper_corner.1 - self.upper_right_corner.1)));
                let min_x_for_point_y = self.pos_x
                    - self.half_size * (1. - ((mouse_position.1 - self.upper_right_corner.1)
                    / (self.upper_corner.1 - self.upper_right_corner.1)));

                if mouse_position.1 > self.upper_right_corner.1
                    && mouse_position.0 < max_x_for_point_y
                    && mouse_position.0 > min_x_for_point_y
                {
                    return true;
                }
                else if mouse_position.1 > self.upper_right_corner.1 {
                    return false;
                }

                let max_x_for_point_y = self.pos_x
                    + self.half_size * (1. - ((self.lower_right_corner.1 - mouse_position.1)
                    / (self.lower_right_corner.1 - self.lower_corner.1)));
                let min_x_for_point_y = self.pos_x
                    - self.half_size * (1. - ((self.lower_right_corner.1 - mouse_position.1)
                    / (self.lower_right_corner.1 - self.lower_corner.1)));

                if mouse_position.1 < self.lower_left_corner.1
                    && mouse_position.0 < max_x_for_point_y
                    && mouse_position.0 > min_x_for_point_y
                {
                    return true;
                }
                else if mouse_position.1 < self.lower_left_corner.1 {
                    return false;
                }

                return true;
            }
        }

        false
    }
}

pub struct Grid {
    pub cells: Vec<Cell>,
}

impl Resource for Grid {}
