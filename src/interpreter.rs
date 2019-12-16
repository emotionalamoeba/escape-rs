use crate::primitives::direction;
use crate::primitives::action;

use regex::Regex;
use lazy_static::lazy_static;

pub fn processCommandIntoAction(command: String) -> Option<action::Action> {

    lazy_static! {
        static ref NORTH_COMMAND: Regex = Regex::new(r"(?i)north").unwrap();
        static ref SOUTH_COMMAND: Regex = Regex::new(r"(?i)south").unwrap();
        static ref EAST_COMMAND: Regex = Regex::new(r"(?i)east").unwrap();
        static ref WEST_COMMAND: Regex = Regex::new(r"(?i)west").unwrap();
    }
    
    if (NORTH_COMMAND.is_match(&command)) {
       
        let action = action::Action {
            verb: action::Verb::Move,
            direction: direction::Direction::North,
        };
        
        return Some(action);
    }
    
    if (SOUTH_COMMAND.is_match(&command)) {
       
        let action = action::Action {
            verb: action::Verb::Move,
            direction: direction::Direction::South,
        };
        
        return Some(action);
    }
    
    if (EAST_COMMAND.is_match(&command)) {
       
        let action = action::Action {
            verb: action::Verb::Move,
            direction: direction::Direction::East,
        };
        
        return Some(action);
    }
    
    if (WEST_COMMAND.is_match(&command)) {
       
        let action = action::Action {
            verb: action::Verb::Move,
            direction: direction::Direction::West,
        };
        
        return Some(action);
    }
    
    None
}