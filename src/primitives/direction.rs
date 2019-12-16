use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub enum Direction {
    North,
    South,
    East,
    West
}


impl Direction {
    pub fn as_str(&self) -> &str {
        match self {
            &Direction::North => "North",
            &Direction::South => "South",
            &Direction::East => "East",
            &Direction::West => "West",
        }
    }
}