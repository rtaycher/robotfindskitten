#![feature(convert)]
use std::collections::HashMap;
extern crate rand;
use rand::{thread_rng, Rng, ThreadRng};
use std::io::{self, Write, Read};
use std::thread::sleep;
use std::time::Duration;
use std::mem::swap;
extern crate wio;

use wio::console::{Input, ScreenBuffer, CharInfo};

static NKI_FILE_CONTENTS: &'static str = include_str!("vanilla.nki");

static ASCII_LOWERCASE_MAP: &'static [u8] = &[b' ', b'!', b'"', b'#', b'$', b'%', b'&', b'\'',
                                              b'(', b')', b'*', b'+', b',', b'-', b'.', b'/',
                                              b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7',
                                              b'8', b'9', b':', b';', b'<', b'=', b'>', b'?',
                                              b'@', b'a', b'b', b'c', b'd', b'e', b'f', b'g',
                                              b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o',
                                              b'p', b'q', b'r', b's', b't', b'u', b'v', b'w',
                                              b'x', b'y', b'z', b'[', b'\\', b']', b'^', b'_',
                                              b'`', b'a', b'b', b'c', b'd', b'e', b'f', b'g',
                                              b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o',
                                              b'p', b'q', b'r', b's', b't', b'u', b'v', b'w',
                                              b'x', b'y', b'z', b'{', b'|', b'}', b'~'];

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i16,
    y: i16,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum GItem {
    Robot,
    Kitten(u8),
    NonKittenItem(String, u8),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum UsefulInput {
    Up,
    Down,
    Left,
    Right,
    Escape,
}
pub use UsefulInput::*;
static VERSION_STRING: &'static str = "robotfindskitten v1";
static INSTRUCTION_STRING: &'static str = "robotfindskitten v1
By the illustrious Leonard \
                                           Richardson (C) 1997, 2000 
Written originally for the \
                                           Nerth Pork robotfindskitten contest 

In this game, \
                                           you are robot (#). 
Your job is to find kitten. 
This \
                                           task is complicated by the existence of various things \
                                           which are not kitten. 
Robot must touch items to \
                                           determine if they are kitten or not. 
The game ends \
                                           when robotfindskitten. 
Alternatively, you may end the \
                                           game by hitting the Esc key. 
See the documentation \
                                           for more information. 
Press any key to start.";

pub use GItem::*;

pub struct Board {
    board_size: Point,
    robot_location: Point,
    board_locations: HashMap<Point, GItem>,
    rng: ThreadRng,
    message: String,
    game_over: bool,
}



#[cfg(target_os = "windows")]
pub struct TextGraphicsContext {
    frontbuf: ScreenBuffer,
    backbuf: ScreenBuffer,
}

impl TextGraphicsContext {
    fn new() -> TextGraphicsContext {
        TextGraphicsContext {
            frontbuf: ScreenBuffer::new().unwrap(),
            backbuf: ScreenBuffer::new().unwrap(),
        }
    }
}

impl Board {
    fn new(mut phrases: Vec<&str>) -> Board {
        let mut b = Board {
            board_size: Point { x: 80, y: 30 },
            board_locations: HashMap::new(),
            rng: thread_rng(),
            message: "".to_string(),
            robot_location: Point { x: 0, y: 0 },
            game_over: false,
        };
        let new_location = b.new_location();
        b.robot_location = new_location;
        let mut ascii_lower: Vec<u8> = ASCII_LOWERCASE_MAP.to_vec();
        {
            let slice: &mut [u8] = ascii_lower.as_mut_slice();
            b.rng.shuffle(slice);
        }

        for _ in 0..10 {
            let new_location = b.new_location();
            b.board_locations.insert(new_location,
                                     NonKittenItem(phrases.pop().unwrap().to_string(),
                                                   ascii_lower.pop().unwrap()));
        }
        b
    }
    fn new_location(&mut self) -> Point {
        let x = self.rng.gen_range(0, self.board_size.x);
        let y = self.rng.gen_range(0, self.board_size.y);
        let mut p = Point { x: x, y: y };
        while self.is_occupied(p) {
            p = Point {
                x: self.rng.gen_range(0, self.board_size.x),
                y: self.rng.gen_range(0, self.board_size.y),
            };
        }
        p
    }
    fn print_board(&self) {
        println!("{:}", VERSION_STRING);
        println!("");
        println!("===========================================================");
        for y in 0..self.board_size.y {
            for x in 0..self.board_size.x {
                match self.board_locations.get(&Point { x: x, y: y }) {
                    Some(&Kitten(ch)) => {
                        io::stdout().write(&[ch]).expect("print should work");
                    }
                    Some(&NonKittenItem(_, ch)) => {
                        io::stdout().write(&[ch]).expect("print should work");
                    }                    
                    _ => {
                        io::stdout().write(&[b' ']).expect("print should work");
                    }
                }


                if (Point { x: x, y: y }) == self.robot_location {
                    io::stdout().write(b"#").expect("print should work");
                }

            }
            io::stdout().write(b"\n").expect("print should work");
        }

    }

    #[cfg(target_os = "windows")]
    fn draw_board(&self, ctx: &mut TextGraphicsContext) {

        let info = ctx.backbuf.info().unwrap();
        let (max_x, max_y) = info.size();
        let mut buf: Vec<_> = (0..(max_x * max_y))
                                  .map(|_| CharInfo::new(0u16, 0u16))
                                  .collect();


        for (i, ch) in VERSION_STRING.chars().enumerate() {
            buf[i] = CharInfo::new(ch as u16, 10u16);
        }

        for (i, ch) in self.message.chars().enumerate() {
            buf[(max_x as usize + i) as usize] = CharInfo::new(ch as u16, 10u16);
        }


        for x in 0..max_x - 1 {
            let y = 2;
            buf[(y * max_x + x) as usize] = CharInfo::new(b'=' as u16, 10u16);
        }

        for y in 0..max_y - 1 {
            for x in 0..max_x - 1 {
                match self.board_locations.get(&Point { x: x, y: y }) {
                    Some(&Kitten(ch)) => {
                        buf[(y * (3 + max_x) + x) as usize] = CharInfo::new(ch as u16, 10u16);
                    }
                    Some(&NonKittenItem(_, ch)) => {
                        buf[(y * (3 + max_x) + x) as usize] = CharInfo::new(ch as u16, 10u16);
                    }                    
                    _ => {
                        // buf[y*x+x] = CharInfo::new(' ', 0u16);
                    }
                }


                if (Point { x: x, y: y }) == self.robot_location {
                    buf[(y * (3 + max_x) + x) as usize] = CharInfo::new(b'#' as u16, 10u16);
                }

            }
        }
        ctx.backbuf.write_output(&buf, (max_x, max_y), (0, 0)).unwrap();
        swap(&mut ctx.backbuf, &mut ctx.frontbuf);
        ctx.frontbuf.set_active().unwrap();
    }

    fn is_out_of_bounds(&self, p: Point) -> bool {
        p.x < 0 || p.y < 0 || p.x >= self.board_size.x || p.y >= self.board_size.y
    }
    fn is_occupied(&self, p: Point) -> bool {
        p == self.robot_location || self.board_locations.contains_key(&p)
    }
    fn attempt_move(&mut self, d: UsefulInput) {
        let mut new_robot_location = self.robot_location.clone();

        match d {
            Up => new_robot_location.y -= 1,
            Down => new_robot_location.y += 1,
            Left => new_robot_location.x -= 1,
            Right => new_robot_location.x += 1,
            Escape => panic!("Escape should never be passed to this function"),
        }
        if self.is_out_of_bounds(new_robot_location) {
            return;
        }
        match self.board_locations.get(&new_robot_location) {
            Some(&Kitten(ch)) => {
                io::stdout().write(&[ch]).expect("print should work");
                self.message = "Game won".to_string();
                self.game_over = true;
            }
            Some(&NonKittenItem(ref s, _)) => {
                self.message = s.clone();
            }
            _ => self.robot_location = new_robot_location,
        }

    }
}

fn get_input(stdin: &ScreenBuffer) -> Vec<UsefulInput> {
    let mut res: Vec<UsefulInput> = Vec::new();
    if stdin.available_input().unwrap() > 0 {
        let input: Vec<_> = stdin.read_input().unwrap();
        let mut last_input_code = 0;
        let mut input = input.iter()
                             .flat_map(|y| {
                                 match *y {
                                     Input::Key(z) => Some(z.wVirtualKeyCode),
                                     _ => None,
                                 }
                             })
                             .collect::<Vec<_>>();
        input.dedup();
        for i in input {
            if i == last_input_code {
                // skip duplicates
                last_input_code = 0;
                continue;
            }
            if i == 0x25 {
                res.push(Left);
            }
            if i == 0x26 {
                res.push(Up);
            }
            if i == 0x27 {
                res.push(Right);
            }
            if i == 0x28 {
                res.push(Down);
            }
            if i == 0x1B {
                res.push(Escape);
            }
        }
    }
    res
}


fn main() {
    let phrases: Vec<&str> = NKI_FILE_CONTENTS.lines().collect();
    let mut b = Board::new(phrases);
    // draw_instruction_string();
    let mut ctx = TextGraphicsContext::new();
    loop {

        let stdin = ScreenBuffer::from_stdin().unwrap();
        b.draw_board(&mut ctx);
        for inp in get_input(&stdin) {

            if inp == Escape || b.game_over {
                return;
            }
            b.attempt_move(inp);
            b.draw_board(&mut ctx);
        }

        if b.game_over {
            break;
        }

        sleep(Duration::new(0, 300));

    }
}
    