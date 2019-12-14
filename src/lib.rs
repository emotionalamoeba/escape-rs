#[macro_use]
extern crate seed;
use seed::prelude::*;
use seed::storage::Storage;
use serde::{Deserialize, Serialize};
use std::char;

const ENTER_KEY: u32 = 13;
const BACKSPACE_KEY: u32 = 0x8;

// Model

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
enum Direction {
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

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
struct Exit {
    direction: Direction,
    goesTo: Room,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
struct Room {
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
                directionList = format!("{}", self.exits[index].direction.as_str().to_string());
            }
            else if (index == self.exits.len() - 1) {
                directionList = format!("{} and {}", directionList, self.exits[index].direction.as_str().to_string());
            }
            else {
                directionList = format!("{}, {}", directionList, self.exits[index].direction.as_str().to_string());
            }
        }

        return format!(
            "Exits are {}", directionList
        );
    }
}

fn build_level() -> Room {

    let mut roomNextDoor = Room {
        name: String::from("Room Next Door"),
        description: String::from("Nothing to see here."),
        exits: Vec::new()
    };
    
    let mut roomToTheEast = Room {
        name: String::from("Room To The East"),
        description: String::from("Nothing to see here."),
        exits: Vec::new()
    };
    
    let mut roomToTheWest = Room {
        name: String::from("Room To The West"),
        description: String::from("Nothing to see here."),
        exits: Vec::new()
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
        // exits: Vec::new()
        exits: vec![ 
            Exit { direction: Direction::North, goesTo: roomNextDoor },
            Exit { direction: Direction::East, goesTo: roomToTheEast },
            Exit { direction: Direction::West, goesTo: roomToTheWest }
        ]
    };
    
    
    return startRoom;
}

struct Model {
    pub val: i32,
    pub currentRoom: Room,
    local_storage: Storage,
    
    edit_text: String,
    cursor_position: i32,
}

impl Model {
    fn sync_storage(&self) {
        // todo: Every item that adds, deletes, or changes a today re-serializes and stores
        // todo the whole model. Effective, but probably quite slow!
        seed::storage::store_data(&self.local_storage, "seed-todo-data", &self.currentRoom);
    }
}

impl Default for Model {
    fn default() -> Self {
        let local_storage = seed::storage::get_storage().unwrap();
        
        Self {
            val: 0,
            currentRoom: build_level(),
            local_storage,
            
            edit_text: String::new(),
            cursor_position: 0,
        }
    }
}


// Update

#[derive(Clone)]
enum Msg {
    Increment,
    EditChange(String),
    
    UpdateCoords(web_sys::MouseEvent),
    KeyPressed(web_sys::KeyboardEvent)
}

fn processCommand(model: &mut Model, command: String) {
    if command.trim().eq_ignore_ascii_case("north") {
        for index in 0..model.currentRoom.exits.len() {
            let exit = &model.currentRoom.exits[index];
            if exit.direction == Direction::North {
                let nextRoom = &exit.goesTo;
                
                model.currentRoom = nextRoom;
            }
        }
    }
}

fn processKeyPress(model: &mut Model, event: web_sys::KeyboardEvent) {

    if event.key_code() == ENTER_KEY {
        processCommand(model, model.edit_text.to_string());
        model.edit_text = String::from("");
    }
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    model.sync_storage(); // Doing it here will miss the most recent update...
    
    match msg {
        Msg::Increment => model.val += 1,
        Msg::EditChange(edit_text) => model.edit_text = edit_text,
        Msg::KeyPressed(ev) => processKeyPress(model, ev),
        Msg::UpdateCoords(ev) => {}
    }
}

fn get_banner_text(model: &Model) -> String {
    let score = 0;
    let level = 1;
    let moves = 0;
    let health = 100;
    let carry = 0;
    
    return format!("Score = {}/500.....L = {}.....Moves = {}.....Health = {}.....Carry = {}", 
        score,
        level,
        moves,
        health,
        carry);
}

// View

fn view(model: &Model) -> impl View<Msg> {
    
    let text_style = style!{ 
        St::UserSelect => "none";
        "font-family" => "dos"; 
        St::Color => "#aaaaaa";
    };
    
    let banner_style = style!{
        St::UserSelect => "none";
        St::WhiteSpace => "nowrap";
        St::Overflow => "hidden";
        St::Position => "absolute";
        St::Top => unit!(0, px);
        St::Left => unit!(0, px);
        St::Height => unit!(1, em);
        St::Width => "100%";
        St::BackgroundColor => "#aa0000";
        St::Color => "#aaaaaa";
        "font-family" => "dos"; 
    };
    
    let input_style = style!{
        St::MaxWidth => unit!(0, px);
        St::MaxHeight => unit!(0, px);
        St::Border => "none";
        
        St::Position => "absolute";
        St::Bottom => unit!(1, em);
    };
    
    vec![
        div![ &banner_style, get_banner_text(model) ],
        
        h1![ &text_style, format!("{}", model.currentRoom.name) ],
        
        div![ &text_style, format!("{}", model.currentRoom.description) ],
        
        div![ &text_style, style!{ St::FontSize => unit!(1, em) }, format!("{}", model.currentRoom.describeExits()) ],
        
        div![ &text_style, format!(">{}", model.edit_text) ],
        
        div![ &text_style ],
                    
        input![
            &input_style,
            attrs! {At::Class => "edit", At::Value => model.edit_text},
            attrs! {At::OnBlur => "this.focus()"},
            attrs! {At::AutoFocus => "autofocus"},
         //   simple_ev(Ev::Blur, Msg::EditSubmit(posit)),
            input_ev(Ev::Input, Msg::EditChange),
            keyboard_ev(Ev::KeyDown, Msg::KeyPressed),
        ],
        
        /*
        /*button![
            simple_ev(Ev::Click, Msg::Increment),
            format!("Hello, World × {}", model.val)
        ]*/*/
    ]
}

fn window_events(model: &Model) -> Vec<Listener<Msg>> {
    let mut result = Vec::new();
    //if model.watching {
        result.push(mouse_ev(Ev::MouseMove, Msg::UpdateCoords));
        //result.push(keyboard_ev(Ev::KeyDown, Msg::KeyPressed));
        //result.push(input_ev(Ev::KeyDown, Msg::KeyPressed));
    //}
    result
}

#[wasm_bindgen(start)]
pub fn render() {
    seed::App::builder(update, view)
        .window_events(window_events)
        .build_and_start();
}