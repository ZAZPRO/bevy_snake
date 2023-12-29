use bevy::render::color::Color;

pub const GRID_CELL: f32 = 60.0;
pub const GRID_SIZE: u32 = 13;
pub const GRID_CENTER: u32 = GRID_SIZE / 2;

pub const WINDOW_SIZE: f32 = GRID_CELL * GRID_SIZE as f32;
pub const LEFT_WINDOW_BORDER: f32 = -WINDOW_SIZE / 2.;
pub const TOP_WINDOW_BORDER: f32 = WINDOW_SIZE / 2.;

pub const BACKGROUND_COLOR: Color = Color::rgb(0.24, 0.25, 0.24);

pub const FOOD_COLOR: Color = Color::rgb(0.9, 0.1, 0.1);
pub const HEAD_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
pub const TAIL_COLOR: Color = Color::rgb(0.15, 0.79, 0.58);

pub const GAME_SPEED: f32 = 0.5;
