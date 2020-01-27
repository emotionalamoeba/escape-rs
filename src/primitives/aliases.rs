use std::collections::HashMap;

use crate::entity::{Room};

pub type RoomMap = HashMap<String, Box<Room>>;