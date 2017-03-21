use std::str;
use common::Point;
use common::GItem::*;
use common::UsefulInput;
use common::UsefulInput::*;
use common::Board;
use common::HEART_CH;
use common::VERSION_STRING;

extern crate ncurses;

use ncurses::*;
use rand::{Rng, ThreadRng};

#[cfg(target_os = "linux")]
#[allow(dead_code)]
pub struct TextGraphicsContext {
    dummy: i32,
}

impl TextGraphicsContext {
    pub fn new() -> TextGraphicsContext {
        setlocale(ncurses::LcCategory::all, "");
        initscr();
        clear();
        noecho();
        nodelay(stdscr(), true);
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        cbreak();
        keypad(stdscr(), true);

        // see http://tldp.org/HOWTO/NCURSES-Programming-HOWTO/color.html
        if has_colors() {
            start_color();

            init_pair(1, COLOR_BLACK, COLOR_BLACK);
            init_pair(2, COLOR_BLUE, COLOR_BLACK);
            init_pair(3, COLOR_GREEN, COLOR_BLACK);
            init_pair(4, COLOR_CYAN, COLOR_BLACK);
            init_pair(5, COLOR_YELLOW, COLOR_BLACK);
            init_pair(6, COLOR_MAGENTA, COLOR_BLACK);
            init_pair(7, COLOR_GREEN, COLOR_BLACK);
            init_pair(8, COLOR_WHITE, COLOR_BLACK);
            init_pair(9, COLOR_WHITE, COLOR_BLACK);
            init_pair(10, COLOR_BLUE, COLOR_BLACK);
            init_pair(11, COLOR_GREEN, COLOR_BLACK);
            init_pair(12, COLOR_CYAN, COLOR_BLACK);
            init_pair(13, COLOR_RED, COLOR_BLACK);
            init_pair(14, COLOR_MAGENTA, COLOR_BLACK);
            init_pair(15, COLOR_YELLOW, COLOR_BLACK);
            init_pair(16, COLOR_WHITE, COLOR_BLACK);

            init_pair(17, COLOR_WHITE, COLOR_BLACK);
        }

        TextGraphicsContext { dummy: 0 }

    }
    pub fn output_size(&self) -> (i16, i16) {
        let mut max_x = 0;
        let mut max_y = 0;
        getmaxyx(stdscr(), &mut max_y, &mut max_x);
        (max_x as i16, max_y as i16)
    }
    pub fn get_rand_non_black_color(&self, rng: &mut ThreadRng) -> u16 {
        rng.gen_range(1, 7)
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
    let mut res = Vec::new();
    loop {
        let ch = getch();

        if ch == ERR {
            break;
        }
        res.push(match ch {
            0x1B => Escape,
            KEY_LEFT => Left,
            KEY_UP => Up,
            KEY_RIGHT => Right,
            KEY_DOWN => Down,
            _ => Other,
        });
    }
    res
}

#[cfg(target_os = "linux")]
pub fn draw_board(b: &Board, ctx: &mut TextGraphicsContext) {
    clear();

    let (max_x, max_y) = ctx.output_size();


    let mut buf = String::new();
    buf.push_str(VERSION_STRING);
    buf.push_str("\n");
    buf.push_str("\n");

    buf.push_str(&(0..max_x - 1).map(|_| "-").collect::<String>());
    buf.push_str("\n");

    printw(&*buf);

    if b.message.contains(HEART_CH) && has_colors() {
        attron(COLOR_PAIR(13));
        mvprintw(1, 0, &b.message);
        attroff(COLOR_PAIR(13));
    } else {
        mvprintw(1, 0, &b.message);
    }

    for y in 0..max_y - 3 {
        for x in 0..max_x - 1 {
            match b.board_locations.get(&Point { x: x, y: y }) {
                Some(&Kitten(ch, color)) => {
                    if has_colors() {
                        attron(COLOR_PAIR(color as i16 + 1));
                    }
                    mvprintw((3 + y).into(), x.into(), str::from_utf8(&[ch]).unwrap());
                    if has_colors() {
                        attroff(COLOR_PAIR(color as i16 + 1));
                    }
                }
                Some(&NonKittenItem(_, ch, color)) => {
                    if has_colors() {
                        attron(COLOR_PAIR(color as i16 + 1));
                    }
                    mvprintw((3 + y).into(), x.into(), str::from_utf8(&[ch]).unwrap());
                    if has_colors() {
                        attroff(COLOR_PAIR(color as i16 + 1));
                    }
                }
                _ => {}
            }

            if (Point { x: x, y: y }) == b.robot_location {
                if has_colors() {
                    attron(COLOR_PAIR(17));
                }
                mvprintw((3 + y).into(), x.into(), "#");
                if has_colors() {
                    attroff(COLOR_PAIR(17));
                }
            }
        }
    }
    refresh();
}

#[cfg(target_os = "linux")]
#[allow(unused_variables)]
pub fn draw_text(ctx: &mut TextGraphicsContext, text: &str) {
    printw(text);
    refresh();
}
