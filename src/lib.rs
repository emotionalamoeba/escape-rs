#[macro_use]
extern crate seed;
extern crate lazy_static;

use seed::prelude::*;
use seed::storage::Storage;
use serde::{Deserialize, Serialize};
use std::char;

use std::cell::RefCell;

use crate::{ entity::{Room} };
use crate::{ primitives::{RoomMap} };
/*use crate::{
    entity::{Room},
    primitives::{RoomMap},
*/

mod entity;
mod primitives;
mod interpreter;

const ENTER_KEY: u32 = 13;
const BACKSPACE_KEY: u32 = 0x8;

// Model

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
struct Exit {
    direction: primitives::direction::Direction,
    goesTo: String,
}

fn build_rooms(roomMap: &mut RoomMap) {
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
            entity::exit::Exit { direction: primitives::direction::Direction::North, goesTo: String::from("Room2") },
        ]
    };
    
    let mut room2 = Room {
        name: String::from("Room2"),
        description: String::from("Welcome to the other room."),
        exits: vec![ 
            entity::exit::Exit { direction: primitives::direction::Direction::South, goesTo: String::from("The Crater"), },
        ]
    };
    
    roomMap.insert(String::from("The Crater"), Box::new(startRoom));
    roomMap.insert(String::from("Room2"), Box::new(room2));
}

struct Model {
    rooms: RoomMap,

    pub val: i32,
    pub currentRoom: String,
    local_storage: Storage,
    
    edit_text: String,
    response_text: String,
    cursor_position: i32,
}

impl Model {
    pub fn get_curr_room(&self) -> &Room {
        if let Some(room) = self.rooms.get(&self.currentRoom) {
            room
        } else {
            panic!(format!(
                "ERROR: {} is not a valid room (The world should be fixed).",
                self.currentRoom
            ))
        }
    }

    fn sync_storage(&self) {
        // todo: Every item that adds, deletes, or changes a today re-serializes and stores
        // todo the whole model. Effective, but probably quite slow!
        seed::storage::store_data(&self.local_storage, "seed-todo-data", &self.currentRoom);
    }
}

impl Default for Model {
    fn default() -> Self {
        let local_storage = seed::storage::get_storage().unwrap();
        
        let mut model = Model {
            rooms: RoomMap::new(),
            
            val: 0,
            local_storage,
            currentRoom: String::new(),
            edit_text: String::new(),
            response_text: String::new(),
            cursor_position: 0,
        };
        
        build_rooms(&mut model.rooms);
        model.currentRoom = String::from("The Crater");
        
        return model;
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

fn moveInDirection(model: &mut Model, direction: primitives::direction::Direction) {

    for index in 0..model.get_curr_room().exits.len() {
        let exit = &model.get_curr_room().exits[index];
        if exit.get_direction() == direction {
            model.currentRoom = exit.goes_to();
        }
    }
}

fn processAction(model: &mut Model, action: &primitives::action::Action) {
    
    model.response_text = String::from("");
        
    match action.direction {
        primitives::direction::Direction::North => { moveInDirection(model, action.direction) },
        primitives::direction::Direction::South => { moveInDirection(model, action.direction) },
        primitives::direction::Direction::East => { moveInDirection(model, action.direction) },
        primitives::direction::Direction::West => { moveInDirection(model, action.direction) },
    }
}

fn processKeyPress(model: &mut Model, event: web_sys::KeyboardEvent) {

    if event.key_code() == ENTER_KEY {
        model.response_text = String::from("");
        
        let result = interpreter::processCommandIntoAction(model.edit_text.to_string());
        
        match result {
            Some(x) => { processAction(model, &x) },
            None => { model.response_text = String::from("I did not understand!") }
        }
    
        model.edit_text = String::from("");
    }
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    //model.sync_storage(); // Doing it here will miss the most recent update...
    
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
    
    let roomName = &model.get_curr_room().name;
    let roomDescription = &model.get_curr_room().description;
    
    vec![
        div![ &banner_style, get_banner_text(model) ],
        
        h1![ &text_style, format!("{}", roomName) ],
        
        div![ &text_style, format!("{}", roomDescription) ],
        
  //      div![ &text_style, style!{ St::FontSize => unit!(1, em) }, format!("{}", model.currentRoom.describeExits()) ],
        
        div![ &text_style, format!("{}", model.response_text) ],
                
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