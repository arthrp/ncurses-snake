//For some reason rustc thinks some structs are never used
#![allow(dead_code)]

extern crate ncurses;

use ncurses::*;

#[derive(PartialEq)]
pub enum Direction {
    North,
    South,
    West,
    East
}

pub struct Snake {
    pub cells_x: Vec<i32>,
    pub cells_y: Vec<i32>
}

impl Snake {
    pub fn new(cell_count: i32) -> Snake {
        let mut s = Snake{ cells_x: Vec::new(), cells_y: Vec::new() };
        let mut x = 10;
        let y = 5;

        for _ in 0..cell_count{
            s.cells_x.push(x);
            s.cells_y.push(y);
            x -= 1;
        }
        return s;
    }

    pub fn draw(&self){
        let cell_count = *&self.cells_x.len() as i32;

        for i in 0..cell_count {
            mvprintw(self.cells_y[i as usize], self.cells_x[i as usize], "O");
        }
    }

    pub fn make_move(&mut self, dir: &Direction) -> () {
        let cell_count = *&self.cells_x.len() as i32;

        //Move body
        for i in (1..cell_count).rev() {
            let ii = i as usize;
            self.cells_x[ii] = self.cells_x[ii-1];
            self.cells_y[ii] = self.cells_y[ii-1];
        }

        //Move head
        match *dir {
            Direction::East => self.cells_x[0] += 1,
            Direction::West => self.cells_x[0] -= 1,
            Direction::North => self.cells_y[0] -= 1,
            Direction::South => self.cells_y[0] += 1
        }
    }
}