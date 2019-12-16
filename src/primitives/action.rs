use crate::primitives::direction;

pub enum Verb {
    Move,
    Take,
    Drop
}

pub struct Article {
    name: String,
}

pub struct Action {
    pub verb: Verb,
    pub direction: direction::Direction,
}