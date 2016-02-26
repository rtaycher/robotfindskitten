use std::mem::swap;

use common::Point;
use common::GItem::*;
use common::UsefulInput;
use common::UsefulInput::*;
use common::Board;

use common::VERSION_STRING;

extern crate wio;
use wio::console::{Input, ScreenBuffer, CharInfo};

static HEART_CH: char = '♥';

// see https://msdn.microsoft.com/en-us/library/windows/desktop/ms682013%28v=vs.85%29.aspx for windows console colors
static FOREGROUND_RED_WINDOWS: u16 = 4;
// http://blog.tedd.no/2015/08/02/better-text-console-for-c/
//        Black = 0x0000,
//        DarkBlue = 0x0001,
//        DarkGreen = 0x0002,
//        DarkRed = 0x0004,
//        Gray = DarkBlue | DarkGreen | DarkRed,
//        DarkYellow = DarkRed | DarkGreen,
//        DarkPurple = DarkRed | DarkBlue,
//        DarkCyan = DarkGreen | DarkBlue,
//        LightBlue = DarkBlue | HighIntensity,
//        LightGreen = DarkGreen | HighIntensity,
//        LightRed = DarkRed | HighIntensity,
//        LightWhite = Gray | HighIntensity,
//        LightYellow = DarkYellow | HighIntensity,
//        LightPurple = DarkPurple | HighIntensity,
//        LightCyan = DarkCyan | HighIntensity

static FOREGROUND_DARK_BLUE: u16 = 1;
static FOREGROUND_DARK_GREEN: u16 = 2;
static FOREGROUND_DARK_RED: u16 = 4;
static FOREGROUND_GREY: u16 = FOREGROUND_DARK_BLUE | FOREGROUND_DARK_GREEN | FOREGROUND_DARK_RED;


#[cfg(target_os = "windows")]
pub struct TextGraphicsContext {
    frontbuf: ScreenBuffer,
    backbuf: ScreenBuffer,
    stdin: ScreenBuffer,
}

impl TextGraphicsContext {
    pub fn new() -> TextGraphicsContext {
        TextGraphicsContext {
            frontbuf: ScreenBuffer::new().unwrap(),
            backbuf: ScreenBuffer::new().unwrap(),
            stdin: ScreenBuffer::from_stdin().unwrap(),
        }
    }
    pub fn output_size(&self) -> (i16, i16) {
        self.backbuf.info().unwrap().size()
    }
}

#[cfg(target_os = "windows")]
pub fn get_input(ctx: &TextGraphicsContext) -> Vec<UsefulInput> {
    let mut res: Vec<UsefulInput> = Vec::new();
    if ctx.stdin.available_input().unwrap() > 0 {
        let input: Vec<_> = ctx.stdin.read_input().unwrap();

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

        res.push(match input[0] {
            0x1B => Escape,
            0x25 => Left,
            0x26 => Up,
            0x27 => Right,
            0x28 => Down,
            _ => Other,
        });
    }
    res
}

#[cfg(target_os = "windows")]
pub fn draw_board(b: &Board, ctx: &mut TextGraphicsContext) {
    let (max_x, max_y) = ctx.output_size();
    let mut buf: Vec<_> = (0..(max_x * max_y))
                              .map(|_| CharInfo::new(0u16, 0u16))
                              .collect();


    for (i, ch) in VERSION_STRING.chars().enumerate() {
        buf[i] = CharInfo::new(ch as u16, 0x0fu16);
    }

    for (i, ch) in b.message.chars().enumerate() {
        if ch == HEART_CH {
            buf[(max_x as usize + i) as usize] = CharInfo::new(ch as u16, FOREGROUND_RED_WINDOWS);
        } else if b.game_over && (!(ch == ' ' || ch == '#')) {
            buf[(max_x as usize + i) as usize] = CharInfo::new(ch as u16, b.kitten_color);
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
            match b.board_locations.get(&Point { x: x, y: y }) {
                Some(&Kitten(ch, color)) => {
                    buf[((3 + y) * max_x + x) as usize] = CharInfo::new(ch as u16, color);
                }
                Some(&NonKittenItem(_, ch, color)) => {
                    buf[((3 + y) * max_x + x) as usize] = CharInfo::new(ch as u16, color);
                }
                _ => {}
            }


            if (Point { x: x, y: y }) == b.robot_location {
                buf[((3 + y) * max_x + x) as usize] = CharInfo::new(b'#' as u16, 0x0fu16);
            }

        }
    }
    ctx.backbuf.write_output(&buf, (max_x, max_y), (0, 0)).unwrap();
    swap(&mut ctx.backbuf, &mut ctx.frontbuf);
    ctx.frontbuf.set_active().unwrap();
}

#[cfg(target_os = "windows")]
pub fn draw_text(ctx: &mut TextGraphicsContext, text: &str) {
    let (max_x, max_y) = ctx.output_size();
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
