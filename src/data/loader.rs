use crate::entity::{ room::{Room} };
use crate::entity::{ exit::{Exit} };
use crate::primitives::{ direction::{Direction} };
use crate::primitives::{ aliases::{RoomMap} };

pub fn build_rooms(roomMap: &mut RoomMap) {

const crashedShipName: &str = "The Crashed Ship";

let mut crashedShip = Room {
    name: crashedShipName.to_string(),
    description: String::from("   You are inside the debris that was your home for six months.  The impact \n
                            of the crash was such that it destroyed most every useful item on the craft.\n
                            You cannot see your hand in front of your face for the dark.  A string is\n
                            swinging against your ear.\n"),
    exits: vec![
        Exit { direction: Direction::South, goesTo: String::from("The Crater") }
    ]
};

    let mut startRoom = Room {
        name: String::from("The Crater"),
        description: String::from("   The last thing you remember is whistling through space and the phenonomal\n\
                                   pain you experienced after your (well, the central prisons') ship spun out of\n\
                                   control.  Astoundingly the safety measures on the prison craft worked better\n\
                                    than you could have hoped for, aside from a few cuts, bruises and a headache\n\
                                    worse than anyone in the universe could have had.\n\
                                       You look dazedly around.  A crater stretches for thirty metres around and\n\
                                    the craft that got you to this state is lying a short distance to the north\n\
                                    in a condition that puts your headache to shame.  It is buried several metres\n\
                                    under the level of the surrounding ground and is twisted and bent like a\n\
                                    screwed up piece of paper.  You yourself are crumpled up and resting next to\n\
                                    the craft.  \n\
                                       So, here you are."),
        exits: vec![ 
            Exit { direction: Direction::North, goesTo: crashedShipName.to_string() },
        ]
    };
    
    roomMap.insert(String::from("The Crater"), Box::new(startRoom));
    roomMap.insert(crashedShipName.to_string(), Box::new(crashedShip));
}