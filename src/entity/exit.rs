use serde::{Deserialize, Serialize};

use crate::primitives::{ direction::{Direction} };
use crate::{ entity::room::{Room} };

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Exit {
    pub direction: Direction,
    pub goesTo: String,
}

impl Exit {
    pub fn get_direction(&self) -> Direction {
        return self.direction;
    }
    
    pub fn goes_to(&self) -> String {
        return format!("{}", self.goesTo);
    }
}