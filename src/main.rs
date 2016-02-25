#[macro_use]
extern crate log;

extern crate log4rs;
use std::io::prelude::*;
use std::io;
use std::thread::sleep;
use std::time::Duration;
use std::collections::HashMap;
use std::fs::File;

extern crate rand;
use rand::{thread_rng, Rng};

pub mod common;
use common::INSTRUCTION_STRING;
use common::Point;
use common::GItem::*;
use common::UsefulInput;
use common::UsefulInput::*;
pub use common::Board;


#[cfg(target_os = "linux")]
extern crate ncurses;
#[cfg(target_os = "linux")]
pub mod linux_console_gui;
#[cfg(target_os = "linux")]
use linux_console_gui::{TextGraphicsContext, get_input, draw_board, draw_text};

#[cfg(target_os = "windows")]
extern crate wio;
#[cfg(target_os = "windows")]
pub mod win_console_gui;
#[cfg(target_os = "windows")]
use win_console_gui::{TextGraphicsContext, get_input, draw_board, draw_text};

static HEART_CH: char = '♥';
static VANILLA_NKI_CONTENTS: &'static str = include_str!("vanilla.nki");
static DEFAULT_LOG_TOML: &'static str = include_str!("log.toml");

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

impl Board {
    fn new(mut phrases: Vec<&str>, ctx: &TextGraphicsContext) -> Board {
        let (x, y) = ctx.output_size();
        let mut b = Board {
            board_size: Point { x: x, y: y - 3 },
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

    fn draw_success(&mut self, ctx: &mut TextGraphicsContext, item_ch: u8) {
        let (max_x, _) = ctx.output_size();
        let middle_x = max_x / 2 - 4;
        let prefix = (0..middle_x).map(|_| " ").collect::<String>();
        let ch = item_ch as char;

        self.message = format!("{}{}      {}", prefix, '#', ch);
        draw_board(self, ctx);
        sleep(Duration::new(1, 0));

        self.message = format!("{} {}    {} ", prefix, '#', ch);
        draw_board(self, ctx);
        sleep(Duration::new(1, 0));

        self.message = format!("{}  {}  {}  ", prefix, '#', ch);
        draw_board(self, ctx);
        sleep(Duration::new(1, 0));

        self.message = format!("{}   {}{}   ", prefix, '#', ch);
        draw_board(self, ctx);
        sleep(Duration::new(1, 0));

        self.message = format!("{}   {}    ", prefix, HEART_CH);
        draw_board(self, ctx);

        sleep(Duration::new(3, 0));
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

fn make_default_file(filepath: &str, default_file_contents: &str) -> std::io::Result<String> {
    let mut s = String::new();

    match File::open(filepath) {
        Ok(mut file) => {
            try!(file.read_to_string(&mut s));
            Ok(s)
        }
        Err(_) => {
            let mut f = try!(File::create(filepath));
            try!(f.write_all(default_file_contents.as_bytes()));
            try!(f.sync_data());
            Ok(default_file_contents.to_string())
        }
    }
}

fn main() {
    make_default_file("log.toml", DEFAULT_LOG_TOML).unwrap();
    log4rs::init_file("log.toml", Default::default()).unwrap();

    let nki = make_default_file("vanilla.nki", VANILLA_NKI_CONTENTS).unwrap();

    let phrases: Vec<&str> = nki.lines().collect();
    let mut ctx = TextGraphicsContext::new();
    let mut b = Board::new(phrases, &ctx);


    draw_text(&mut ctx, INSTRUCTION_STRING);
    loop {
        if let Some(f_inp) = get_input(&ctx).first() {
            if *f_inp == Escape {
                debug!("Exiting!");
                return;
            } else {
                break;
            }
        } else {
            sleep(Duration::new(2, 0));
        }
    }

    loop {
        debug!("before board draw");
        draw_board(&b, &mut ctx);
        for inp in get_input(&ctx) {
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
            debug!("before board draw");
            draw_board(&b, &mut ctx);
        }

        if b.game_over {
            break;
        }
        sleep(Duration::new(0, 22_000_000));
    }
}
