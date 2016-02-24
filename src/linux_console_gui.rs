use common::Point;
use common::GItem::*;
use common::UsefulInput;
use common::UsefulInput::*;
use common::Board;

use common::VERSION_STRING;

extern crate ncurses;

use ncurses::*;

#[cfg(target_os = "linux")]
#[allow(dead_code)]
pub struct TextGraphicsContext {
    dummy: i32
}

impl TextGraphicsContext {
    pub fn new() -> TextGraphicsContext {
        initscr();

        TextGraphicsContext {
            dummy: 0
        }
    }
    pub fn output_size(&self) -> (i16, i16) {
        let mut max_x = 0;
        let mut max_y = 0;
        getmaxyx(stdscr, &mut max_y, &mut max_x);
        (max_x as i16,max_y as i16)
    }
}

impl Drop for TextGraphicsContext {
    fn drop(&mut self) {
        endwin();
    }
}

#[cfg(target_os = "linux")]
#[allow(unused_variables)]
pub fn get_input(ctx: &TextGraphicsContext) -> Vec<UsefulInput> {
    let ch = getch();
    let mut res = Vec::new();
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
    

    let mut buf = String::new();
    buf.push_str(VERSION_STRING);
    buf.push_str("\n");
    buf.push_str(&b.message);
    buf.push_str("\n");
        // if ch == HEART_CH {
        //     buf[(max_x as usize + i) as usize] = CharInfo::new(ch as u16, FOREGROUND_RED_linux);
        // } else if b.game_over && (!(ch == ' ' || ch == '#')) {
        //     buf[(max_x as usize + i) as usize] = CharInfo::new(ch as u16, b.kitten_color);
        // } else {
        // buf[(max_x as usize + i) as usize] = CharInfo::new(ch as u16, 0x0fu16);
        
    buf.push_str(&(0..max_x - 1).map(|_| "-").collect::<String>());
    buf.push_str("\n");
    let mut grid_buf: Vec<u8> = (0..(max_x * (max_y-3))).map(|_| b' ').collect(); 
      
    buf.push_str("\n");  
    
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
    buf.push_str(&String::from_utf8(grid_buf).expect("should be utf-8"));
    printw(&*buf);
    
    refresh();
}

#[cfg(target_os = "linux")]
#[allow(unused_variables)]
pub fn draw_text(ctx: &mut TextGraphicsContext, text: &str) {
    printw(text);
    refresh();
}
