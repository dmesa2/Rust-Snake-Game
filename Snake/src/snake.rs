use std::collections::LinkedList;//allows us to push and pop from either end of the list
use piston_window::{Context, G2d};//bringing in context and graphical buffer
use piston_window::types::Color;//bringing in color

use crate::draw::draw_block;//bring in draw block from draw.rs file

const SNAKE_COLOR: Color = [0.00, 0.80, 0.00, 1.0];//RGB, Opacity - So 80% green with 100% opacity
#[derive(Copy, Clone, PartialEq)]
pub enum Direction {//Handles direction of snake and how keyboard interacts with snake
    Up,
    Down,
    Left,
    Right,
}

impl Direction {//Method for our direction enum
    pub fn opposite(&self) -> Direction {//So if snake is going up and we hit down then snake will not go down
        match *self { /*We could potentially change this to allow the snake to move in whatever direction*/
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}
#[derive(Debug, Clone)]
struct Block {//Block size for our snake
    x: i32,
    y: i32,
}

pub struct Snake {//Snakes functionality (direction that snake is travelling in, body is a linked list, and adding to tail)
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>,
}

impl Snake {//implementing Snake
    pub fn new(x: i32, y: i32) -> Snake {//This is how the snake will start out at the beginning of the game
        let mut body: LinkedList<Block> = LinkedList::new();//Creating the mutable body
        body.push_back(Block {//Snake starts at length of three
            x: x + 2, //Third block x + 2, y
            y,
        });
        body.push_back(Block {//second block being x + 1, y
            x: x + 1,
            y,
        });
        body.push_back(Block {//First block being x and y
            x,
            y,
        });

        Snake {
            direction: Direction::Right,//Starts moving right
            body, // body = body
            tail: None, //Tail starts with none by default
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {//Iterate through list and call draw block function
        for block in &self.body {//Renders out green snake
            draw_block(SNAKE_COLOR, block.x, block.y, con, g);
        }
    }

    pub fn head_position(&self) -> (i32, i32) {//Finds head of snake 
        let head_block = self.body.front().unwrap();
        (head_block.x, head_block.y)
    }

    pub fn move_forward(&mut self, dir: Option<Direction>) {//Match on dir, set direction to d
        match dir {
            Some(d) => self.direction = d,
            None => (),
        }

        let (last_x, last_y): (i32, i32) = self.head_position();

        let new_block = match self.direction {//match on actual snake direction
            Direction::Up => Block {//If we are going in direction up, then we are going to create a new block
                x: last_x,          //and we are going to move forward in the negative y axis (movement is inverted)
                y: last_y - 1,
            },
            Direction::Down => Block {
                x: last_x,
                y: last_y + 1,
            },
            Direction::Left => Block {
                x: last_x - 1,
                y: last_y,
            },
            Direction::Right => Block {
                x: last_x + 1,
                y: last_y,
            },
        };
        self.body.push_front(new_block);//push this new block in front of snake
        let removed_block = self.body.pop_back().unwrap();//This function basically removoves the last element and adds
        self.tail = Some(removed_block);                    //to the top creating the illusion of movement
    }

    pub fn head_direction(&self) -> Direction {//cloning a direction
        self.direction
    }

    pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {//Get head position and match on direction
        let (head_x, head_y): (i32, i32) = self.head_position();

        let mut moving_dir = self.direction;
        match dir {
            Some(d) => moving_dir = d,
            None => {}
        }

        match moving_dir {
            Direction::Up => (head_x, head_y - 1),
            Direction::Down => (head_x, head_y + 1),
            Direction::Left => (head_x - 1, head_y),
            Direction::Right => (head_x + 1, head_y),
        }
    }

    pub fn restore_tail(&mut self) {//Create a block based on tail which we will clone
        let blk = self.tail.clone().unwrap();
        self.body.push_back(blk);//push clone tail into back of body - so if we eat apple this method will be run
    }

    pub fn overlap_tail(&self, x: i32, y: i32) -> bool {//This will check if tail overlaps and if so it will fail
        let mut ch = 0;
        for block in &self.body {//iterate through body
            if x == block.x && y == block.y {
                return true;//if snake overlaps return true
            }

            ch += 1;//else increment
            if ch == self.body.len() - 1 {//if ch == length of snake body - 1
                break;
            }
        }
        return false;
    }
    pub fn cut_in_half(&mut self) {


    }
}
