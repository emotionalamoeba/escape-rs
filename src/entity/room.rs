use serde::{Deserialize, Serialize};

use crate::{ entity::{Exit} };

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Room {
    pub name: String,
    pub description: String,
    pub exits: Vec<Exit>
}

impl Room {
    pub fn describeExits(&self) -> String {
        if self.exits.is_empty() {
            return String::from("There is no way out!")
        }
        
        let mut directionList = "".to_string();
        
        for index in 0..self.exits.len() {
            if (index == 0) {
                directionList = format!("{}", self.exits[index].get_direction().as_str().to_string());
            }
            else if (index == self.exits.len() - 1) {
                directionList = format!("{} and {}", directionList, self.exits[index].get_direction().as_str().to_string());
            }
            else {
                directionList = format!("{}, {}", directionList, self.exits[index].get_direction().as_str().to_string());
            }
        }

        return format!(
            "Exits are {}", directionList
        );
    }
}