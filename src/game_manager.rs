extern crate ncurses;

#[path = "other.rs"]
pub mod other;

use other::*;
use ncurses::*;
use std::{thread, time};

pub struct GameManager {
    is_finished: bool,
    pub direction: Direction,
    snake: Snake,
    max_x: i32,
    max_y: i32    
}

impl GameManager {
    pub fn run(&mut self) -> () {
        clear();
        self.snake.draw();
        self.snake.make_move(&self.direction);

        refresh();

        if(self.check_collision()){
            self.is_finished = true;
        }
    }

    pub fn new(max_x: &i32, max_y: &i32) -> GameManager {
        return GameManager{ is_finished: false, direction: Direction::East, snake: Snake::new(3), max_x: max_x.clone(), max_y: max_y.clone() };
    }

    pub fn run_game_loop(&mut self) -> () {
        let mut ch: i32;

        loop {
            ch = getch();
    
            match ch {
                KEY_UP => 
                    if(self.direction != Direction::South) { self.direction = Direction::North },
                KEY_DOWN => 
                    if(self.direction != Direction::North) { self.direction = Direction::South },
                KEY_RIGHT =>
                    if(self.direction != Direction::West) { self.direction = Direction::East },
                KEY_LEFT =>
                    if(self.direction != Direction::East) { self.direction = Direction::West },
                _ => ()
            }
    
            self.run();

            if(self.is_finished){
                break;
            }
    
            thread::sleep(time::Duration::from_millis(5));
        }

        self.game_over_loop();
    }

    fn print_centered(&self, msg: &str) -> () {
        clear();
        let pos_x = self.max_x/2 - ((msg.len()/2) as i32);
        mvprintw(self.max_y/2, pos_x, &msg);
        refresh();
    }

    fn check_collision(&self) -> bool {
        let head_x = self.snake.cells_x[0];
        let head_y = self.snake.cells_y[0];

        if(head_x < 0 || head_y < 0 || head_x > self.max_x || head_y > self.max_y){
            return true;
        }

        return false;
    }

    fn game_over_loop(&self) -> () {
        let mut ch: i32;

        loop {
            ch = getch();

            match ch {
                ERR => (),
                _ => return
            }

            self.print_centered("Game over");
        }
    }
}