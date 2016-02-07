#![feature(convert)]
use std::collections::HashMap;
extern crate rand;
use rand::{thread_rng, Rng, ThreadRng};

use std::thread::sleep;
use std::time::Duration;
use std::mem::swap;
extern crate wio;

use wio::console::{Input, ScreenBuffer, CharInfo};

static HEART_CH: char = '♥';
static FOREGROUND_RED_WINDOWS: u16 = 4;
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
    Kitten(u8, u16),
    NonKittenItem(String, u8, u16),
}

pub use GItem::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum UsefulInput {
    Up,
    Down,
    Left,
    Right,
    Escape,
    Other,
}

pub use UsefulInput::*;

pub struct Board {
    board_size: Point,
    robot_location: Point,
    board_locations: HashMap<Point, GItem>,
    rng: ThreadRng,
    message: String,
    game_over: bool,
    kitten_color: u16,
}
static VERSION_STRING: &'static str = "robotfindskitten v0.9";
static INSTRUCTION_STRING: &'static str = "robotfindskitten v0.9 
This version was written by \
                                           Roman Taycher (C) 2015 <rtaycher1987@gmail.com>

\
                                           Written originally for the Nerth Pork robotfindskitten \
                                           contest 
by the illustrious Leonard Richardson (C) 1997, \
                                           2000

In this game, you are robot (#). 
Your job is to \
                                           find kitten. 
This task is complicated by the \
                                           existence of various things which are not kitten. 
\
                                           Robot must touch items to determine if they are kitten \
                                           or not. 
The game ends when robotfindskitten. 
\
                                           Alternatively, you may end the game by hitting the Esc \
                                           key. 
See the documentation for more information. 
\
                                           Press any key to start.";



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
            kitten_color: 0,
        };
        let new_location = b.new_location();
        b.robot_location = new_location;
        let mut ascii_lower: Vec<u8> = ASCII_LOWERCASE_MAP.to_vec();
        {
            let slice: &mut [u8] = ascii_lower.as_mut_slice();
            b.rng.shuffle(slice);
        }

        for _ in 0..21 {
            let new_location = b.new_location();
            let color: u16 = b.rng.gen_range(0, 0xf);
            b.board_locations.insert(new_location,
                                     NonKittenItem(phrases.pop().unwrap().into(),
                                                   ascii_lower.pop().unwrap(),
                                                   color));
        }

        let new_location = b.new_location();
        let color: u16 = b.rng.gen_range(0, 0xf);

        b.kitten_color = color;
        b.board_locations.insert(new_location, Kitten(ascii_lower.pop().unwrap(), color));
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

    #[cfg(target_os = "windows")]
    fn draw_board(&self, ctx: &mut TextGraphicsContext) {

        let info = ctx.backbuf.info().unwrap();
        let (max_x, max_y) = info.size();
        let mut buf: Vec<_> = (0..(max_x * max_y))
                                  .map(|_| CharInfo::new(0u16, 0u16))
                                  .collect();


        for (i, ch) in VERSION_STRING.chars().enumerate() {
            buf[i] = CharInfo::new(ch as u16, 0x0fu16);
        }

        for (i, ch) in self.message.chars().enumerate() {
            if ch == HEART_CH {
                buf[(max_x as usize + i) as usize] = CharInfo::new(ch as u16,
                                                                   FOREGROUND_RED_WINDOWS);
            } else if self.game_over && (!(ch == ' ' || ch == '#')) {
                buf[(max_x as usize + i) as usize] = CharInfo::new(ch as u16, self.kitten_color);
            } else {
                buf[(max_x as usize + i) as usize] = CharInfo::new(ch as u16, 0x0fu16);
            }
        }


        for x in 0..max_x - 1 {
            let y = 2;
            buf[(y * max_x + x) as usize] = CharInfo::new(b'-' as u16, 0x0fu16);
        }

        for y in 0..max_y - 1 {
            for x in 0..max_x - 1 {
                match self.board_locations.get(&Point { x: x, y: y }) {
                    Some(&Kitten(ch, color)) => {
                        buf[((3 + y) * max_x + x) as usize] = CharInfo::new(ch as u16, color);
                    }
                    Some(&NonKittenItem(_, ch, color)) => {
                        buf[((3 + y) * max_x + x) as usize] = CharInfo::new(ch as u16, color);
                    }                    
                    _ => {}
                }


                if (Point { x: x, y: y }) == self.robot_location {
                    buf[((3 + y) * max_x + x) as usize] = CharInfo::new(b'#' as u16, 0x0fu16);
                }

            }
        }
        ctx.backbuf.write_output(&buf, (max_x, max_y), (0, 0)).unwrap();
        swap(&mut ctx.backbuf, &mut ctx.frontbuf);
        ctx.frontbuf.set_active().unwrap();
    }

    fn draw_success(&mut self, ctx: &mut TextGraphicsContext, item_ch: u8) {
        let info = ctx.backbuf.info().unwrap();
        let (max_x, _) = info.size();
        let middle_x = max_x / 2 - 4;
        let prefix = (0..middle_x).map(|_| " ").collect::<String>();
        let ch = item_ch as char;

        self.message = format!("{}{}      {}", prefix, '#', ch);
        self.draw_board(ctx);
        sleep(Duration::new(1, 0));

        self.message = format!("{} {}    {} ", prefix, '#', ch);
        self.draw_board(ctx);
        sleep(Duration::new(1, 0));

        self.message = format!("{}  {}  {}  ", prefix, '#', ch);
        self.draw_board(ctx);
        sleep(Duration::new(1, 0));

        self.message = format!("{}   {}{}   ", prefix, '#', ch);
        self.draw_board(ctx);
        sleep(Duration::new(1, 0));

        self.message = format!("{}    {}    ", prefix, HEART_CH);
        self.draw_board(ctx);

        sleep(Duration::new(3, 0));

    }
    fn draw_text(&self, ctx: &mut TextGraphicsContext, text: &str) {
        let info = ctx.backbuf.info().unwrap();
        let (max_x, max_y) = info.size();
        let mut buf: Vec<_> = (0..(max_x * max_y))
                                  .map(|_| CharInfo::new(0u16, 0u16))
                                  .collect();

        for (y, line) in text.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                buf[((max_x as usize) * y + x) as usize] = CharInfo::new(ch as u16, 0x0fu16);
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
    fn attempt_move(&mut self, ctx: &mut TextGraphicsContext, d: UsefulInput) {
        let mut new_robot_location = self.robot_location.clone();
        let mut kitten_ch = None;
        match d {
            Up => new_robot_location.y -= 1,
            Down => new_robot_location.y += 1,
            Left => new_robot_location.x -= 1,
            Right => new_robot_location.x += 1,
            _ => panic!("Escape/Other should never be passed to this function"),
        }
        if self.is_out_of_bounds(new_robot_location) {
            return;
        }

        match self.board_locations.get(&new_robot_location) {
            Some(&Kitten(ch, _)) => {
                self.message = "Game won".into();
                self.game_over = true;
                kitten_ch = Some(ch);
            }
            Some(&NonKittenItem(ref s, _, _)) => {
                self.message = s.clone();
            }
            _ => self.robot_location = new_robot_location,
        }
        if let Some(ch) = kitten_ch {
            self.draw_success(ctx, ch);
        }
    }
}

fn get_input(stdin: &ScreenBuffer) -> Vec<UsefulInput> {
    let mut res: Vec<UsefulInput> = Vec::new();
    if stdin.available_input().unwrap() > 0 {
        let input: Vec<_> = stdin.read_input().unwrap();

        let input = input.iter()
                         .flat_map(|y| {
                             match *y {
                                 Input::Key(z) => Some(z.wVirtualKeyCode),
                                 _ => None,
                             }
                         })
                         .collect::<Vec<_>>();
        if input.len() == 0 {
            return res;
        }
        let i = input[0];

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
        } else {
            res.push(Other)
        }

    }

    res
}


fn main() {
    let phrases: Vec<&str> = NKI_FILE_CONTENTS.lines().collect();
    let mut b = Board::new(phrases);


    let mut ctx = TextGraphicsContext::new();
    let stdin = ScreenBuffer::from_stdin().unwrap();

    b.draw_text(&mut ctx, INSTRUCTION_STRING);
    loop {
        if let Some(f_inp) = get_input(&stdin).first() {
            if *f_inp == Escape {
                return;
            } else {
                break;
            }
        } else {
            sleep(Duration::new(2, 0));
        }
    }

    loop {
        b.draw_board(&mut ctx);
        for inp in get_input(&stdin) {
            if b.game_over {
                return;
            }
            if inp == Escape {
                return;
            }
            if inp == Other {
                continue;
            }
            b.attempt_move(&mut ctx, inp);
            b.draw_board(&mut ctx);
        }

        if b.game_over {
            break;
        }
        sleep(Duration::new(0, 22_000_000));
    }
}
    