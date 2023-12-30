use bevy::reflect::Reflect;

#[derive(Default, Reflect, PartialEq, Clone, Copy, Debug)]
pub enum Direction {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match &self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}
