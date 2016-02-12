use std::mem::swap;

use common::Point;
use common::GItem::*;
use common::UsefulInput;
use common::UsefulInput::*;
use common::Board;

use common::VERSION_STRING;

extern crate ncurses;

use ncurses::*;

#[cfg(target_os = "linux")]
pub struct TextGraphicsContext {
}

impl TextGraphicsContext {
    pub fn new() -> TextGraphicsContext {
        initscr();

        TextGraphicsContext {
        }
    }
    pub fn output_size(&self) -> (i16, i16) {
        self.backbuf.info().unwrap().size()
    }
}

impl Drop for TextGraphicsContext {
    fn drop(&mut self) {
        endwin();
    }
}

#[cfg(target_os = "linux")]
pub fn get_input(ctx: &TextGraphicsContext) -> Vec<UsefulInput> {
    let ch = getch();
    if ch == ERR {
        return res;
    }
    res.push(match ch {
        0x1B => Escape,
        KEY_LEFT => Left,
        KEY_UP => Up,
        KEY_RIGHT => Right,
        KEY_DOWN => Down,
        _ => Other,
    });

    res
}

#[cfg(target_os = "linux")]
pub fn draw_board(b: &Board, ctx: &mut TextGraphicsContext) {
    let (max_x, max_y) = ctx.output_size();
    

    let buf = String::new();
    buf.push_str(VERSION_STRING);
    b.push_str("\n");
    buf.push_str(b.message);
    b.push_str("\n");
        // if ch == HEART_CH {
        //     buf[(max_x as usize + i) as usize] = CharInfo::new(ch as u16, FOREGROUND_RED_linux);
        // } else if b.game_over && (!(ch == ' ' || ch == '#')) {
        //     buf[(max_x as usize + i) as usize] = CharInfo::new(ch as u16, b.kitten_color);
        // } else {
        // buf[(max_x as usize + i) as usize] = CharInfo::new(ch as u16, 0x0fu16);
        
    buf.push_str((0..max_x - 1).map(|_| "-").collect::<String>());
    b.push_str("\n");
    let mut grid_buf: Vec<_> = (0..(max_x * (max_y-3))).map(|_| ' ').collect(); 
      
    b.push_str("\n");  
    
    for y in 0..max_y - 1 {
        for x in 0..max_x - 1 {
            match b.board_locations.get(&Point { x: x, y: y }) {
                Some(&Kitten(ch, color)) => {
                    grid_buf[((3 + y) * max_x + x) as usize] = ch;
                }
                Some(&NonKittenItem(_, ch, color)) => {
                    grid_buf[((3 + y) * max_x + x) as usize] = ch;
                }                    
                _ => {}
            }

            if (Point { x: x, y: y }) == b.robot_location {
                grid_buf[((3 + y) * max_x + x) as usize] =b'#' ;
            }
        }
    }
    buf.push_str(grid_buf.to_string());
    printw(&*buf);
    
    refresh();
}

#[cfg(target_os = "linux")]
pub fn draw_text(ctx: &mut TextGraphicsContext, text: &str) {
    printw(text);
    refresh();
}
