//For some reason rustc thinks some structs are never used
#![allow(dead_code)]

extern crate ncurses;
extern crate rand;

use ncurses::*;
use rand::{thread_rng, Rng};

#[derive(PartialEq)]
pub enum Direction {
    North,
    South,
    West,
    East
}

pub struct Snake {
    pub cells_x: Vec<i32>,
    pub cells_y: Vec<i32>,
    pub cell_count: i32
}

impl Snake {
    pub fn new(cell_count: i32) -> Snake {
        let mut s = Snake{ cells_x: Vec::new(), cells_y: Vec::new(), cell_count: cell_count };
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
        for i in 0..self.cell_count {
            mvprintw(self.cells_y[i as usize], self.cells_x[i as usize], "O");
        }
    }

    pub fn make_move(&mut self, dir: &Direction) -> () {
        //Move body
        for i in (1..self.cell_count).rev() {
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

    pub fn add_cell(&mut self) -> () {
        self.cell_count += 1;
        self.cells_x.push(self.cells_x[(self.cell_count -2) as usize]);
        self.cells_y.push(self.cells_y[(self.cell_count -2) as usize]);
    }
}

pub struct Apple {
    pub x: i32,
    pub y: i32
}

impl Apple {
    pub fn new(max_x: &i32, max_y: &i32) -> Apple {
        let mut rng = thread_rng();

        let x = rng.gen_range(0..*max_x);
        let y = rng.gen_range(0..*max_y);

        return Apple { x: x, y: y};
    }
}