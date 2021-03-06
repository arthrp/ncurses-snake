#![allow(unused_parens)]

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
    apple: Apple,
    max_x: i32,
    max_y: i32,
    score: i32    
}

impl GameManager {
    pub fn run(&mut self) -> () {
        clear();
        self.snake.draw();
        mvprintw(self.apple.y, self.apple.x, "*");
        refresh();

        self.check_if_apple_eaten();
        self.snake.make_move(&self.direction);

        if(self.check_collision()){
            self.is_finished = true;
        }
    }

    pub fn new(max_x: &i32, max_y: &i32) -> GameManager {
        let apple = Apple::new(max_x, max_y);
        return GameManager{ is_finished: false, direction: Direction::East, snake: Snake::new(3), apple: apple, max_x: max_x.clone(), max_y: max_y.clone(),
            score: 0 };
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

    fn print_gameover_msg(&self) -> () {
        clear();
        let first_row_text = "Game over";
        let mut pos_x = self.get_centered_text_x(&self.max_x, first_row_text);
        mvprintw(self.max_y/2, pos_x, first_row_text);

        let second_row_text = format!("Score: {0}", self.score);
        pos_x = self.get_centered_text_x(&self.max_x, second_row_text.as_str());
        mvprintw(self.max_y/2+1, pos_x, second_row_text.as_str());

        refresh();
    }

    fn get_centered_text_x(&self, max_x: &i32, msg: &str) -> i32 {
        let pos_x = *max_x/2 - ((msg.len()/2) as i32);

        return pos_x;
    }

    fn check_collision(&self) -> bool {
        let head_x = self.snake.cells_x[0];
        let head_y = self.snake.cells_y[0];

        if(head_x < 0 || head_y < 0 || head_x >= self.max_x || head_y >= self.max_y){
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

            self.print_gameover_msg();
        }
    }

    fn check_if_apple_eaten(&mut self) -> () {
        if(self.apple.x == self.snake.cells_x[0] && self.apple.y == self.snake.cells_y[0]){
            self.snake.add_cell();
            self.score += 1;
            self.apple = Apple::new(&self.max_x, &self.max_y);
        }
    }
}