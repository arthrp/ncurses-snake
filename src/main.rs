extern crate ncurses;

mod game_manager;
mod other;

use game_manager::*;
use ncurses::*;


fn main() {
    initscr();
    keypad(stdscr(), true);
    noecho();
    halfdelay(1);
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    
    let (mut max_y, mut max_x) = (0,0);
    getmaxyx(stdscr(), &mut max_y, &mut max_x);

    let mut g = GameManager::new(&max_x, &max_y);
    g.run_game_loop();
    
    endwin();
}
