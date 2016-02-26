use std::collections::HashMap;
use rand::ThreadRng;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GItem {
    Kitten(u8, u16),
    NonKittenItem(String, u8, u16),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum UsefulInput {
    Up,
    Down,
    Left,
    Right,
    Escape,
    Other,
}

pub struct Board {
    pub board_size: Point,
    pub robot_location: Point,
    pub board_locations: HashMap<Point, GItem>,
    pub rng: ThreadRng,
    pub message: String,
    pub game_over: bool,
    pub kitten_color: u16,
}

pub static HEART_CH: char = '♥';

pub static VERSION_STRING: &'static str = "robotfindskitten v0.9";
pub static INSTRUCTION_STRING: &'static str = "robotfindskitten v0.9
This version was written by \
                                               Roman Taycher (C) 2015 <rtaycher1987@gmail.com>

\
                                               Written originally for the Nerth Pork \
                                               robotfindskitten contest
by the illustrious \
                                               Leonard Richardson (C) 1997, 2000

In this game, \
                                               you are robot (#).
Your job is to find kitten.
\
                                               This task is complicated by the existence of \
                                               various things which are not kitten.
Robot must \
                                               touch items to determine if they are kitten or \
                                               not.
The game ends when robotfindskitten.
\
                                               Alternatively, you may end the game by hitting the \
                                               Esc key.
See the documentation for more \
                                               information.
Press any key to start.";
