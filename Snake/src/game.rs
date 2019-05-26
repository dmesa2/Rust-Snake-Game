extern crate piston_window;
extern crate find_folder;

use crate::main;

use std::process;
use piston_window::*;//import all of piston 
use piston_window::types::Color;

use rand::{thread_rng, Rng};//allows us to create thread local rand num seeded by the system

use crate::MOVING_PERIOD;
use crate::snake::{Direction, Snake};//bring in snake
use crate::draw::{draw_block, draw_rectangle};//bringing in draw

use std::fs;

use std::fs::File;
use std::io::Write;

const APPLE_COLOR: Color = [0.80, 0.00, 0.00, 1.0]; // 80% red with 100% opacity
const BERRY_COLOR: Color = [0.80, 0.00, 0.80, 1.0]; // 80% red, 80% blue with 100% opacity
const ORANGE_COLOR: Color = [0.80, 0.50, 0.00, 1.0];
const BORDER_COLOR: Color = [0.00, 0.00, 0.00, 1.0]; // Dark black border
const GAMEOVER_COLOR: Color = [0.90, 0.00, 0.00, 0.5];//Game Over screen - red but with 50% opacit
const BLACK: Color = [0.0, 0.0, 0.0, 1.0];//white color
const RED: Color = [1.0, 0.0, 0.0, 1.0]; //red color
const BLUE: Color = [0.0,0.0,1.0,1.0]; //blue color
//const MOVING_PERIOD: f64 = 0.1; //Snake's speed (FPS) -  We can adjust this 3 times for difficulty!
const RESTART_TIME: f64 = 1.0; //Amount of time between failure state and next game (1 second)

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum SoundEffect {
    Eat,
    Die,
}

pub struct Game {//Game struct
    snake: Snake,

    theme: Color,
    food_exists: bool,
    food_x: i32,
    food_y: i32,
    food_type: String, //Apple or Berry

    width: i32,
    height: i32,

    obs_x: i32,
    obs_y: i32,

    pub score: i32, // score for game
    pub high_score: i32, //high score for game

    game_over: bool,
    waiting_time: f64,
}

impl Game {//implementation method for the struct game
    pub fn new(theme: Color, width: i32, height: i32) -> Game {//instatiates new game
        Game {
            snake: Snake::new(2, 2),//snake starts at 2,2 (top left corner)
            waiting_time: 0.0,//snake automatically starts moving
            theme,
            food_exists: true,//food will spawn at below x and y (6 and 4 coord)
            food_x: 6,
            food_y: 4,
            food_type: "apple".to_string(),
            width, // size of board
            height,
            obs_x: 25,
            obs_y: 5,
            score: 0, // score
            high_score: 0, //high score
            game_over: false // when we hit wall this will be true
        }
    }



    pub fn key_pressed(&mut self, key: Key) {//if game over then quit
        if self.game_over {
            return;
        }

        let dir = match key {//if key up then go up and etc
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => Some(self.snake.head_direction())
        };

        if dir.unwrap() == self.snake.head_direction().opposite() {//if snake is moving up and we hit down
            return;                                                 //nothing will happen
        }

        self.update_snake(dir);
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.snake.draw(con, g);//iterates through linked list

        if self.food_exists {//draw block
            if self.food_type == "apple".to_string() {
                draw_block(APPLE_COLOR, self.food_x, self.food_y, con, g);
            }
            else if self.food_type == "berry".to_string() {
                draw_block(BERRY_COLOR, self.food_x, self.food_y, con, g);
            }
            else if self.food_type == "orange".to_string() {
                draw_block(ORANGE_COLOR, self.food_x, self.food_y, con, g);
            }
        }
 
        draw_block([0.5,0.5,0.0,1.0], self.obs_x, self.obs_y, con, g);
        draw_block([0.5,0.5,0.0,1.0], self.obs_x+1, self.obs_y, con, g);
        draw_block([0.5,0.5,0.0,1.0], self.obs_x, self.obs_y+1, con, g);
        draw_block([0.5,0.5,0.0,1.0], self.obs_x+1, self.obs_y+1, con, g);

        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);//draws the borders
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);
        draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g);

        if self.game_over {//if game over then draw game over screen (in this case it is entire screen)
            draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, con, g);
        }
    }

    fn count_up_score(&mut self, points: i32) {
        self.score += points;
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;//iterate waiting time

        if self.game_over {//if game over then 
            if self.waiting_time > RESTART_TIME {//restart the game

                self.restart();
            }
            return;//else return
        }

        if !self.food_exists {//if food does not exist then add food
            self.add_food();
        }

        unsafe {
            if self.waiting_time > MOVING_PERIOD {//update snake if this is true
                self.update_snake(None);
            }
        }
    }

    fn check_eating(&mut self) {//if snake is eating then 
        let (head_x, head_y): (i32, i32) = self.snake.head_position();
        if self.food_exists && self.food_x == head_x && self.food_y == head_y {//snake eats food
            music::play_sound(&SoundEffect::Eat, music::Repeat::Times(0), music::MAX_VOLUME);
            self.food_exists = false;//food doesn't exist anymore
            self.snake.restore_tail();//our snake is going to grow one block
            if self.food_type == "apple".to_string() {
                self.count_up_score(1); //add 1 to score
            }
            else if self.food_type == "berry".to_string() {
                self.count_up_score(2); //add 2 to score
            }
            else if self.food_type == "orange".to_string() {
                self.snake.cut_in_half();
                self.count_up_score(1);
            }
        }
    }

    fn check_if_snake_alive(&self, dir: Option<Direction>) -> bool {//check if snake is alive
        let (next_x, next_y) = self.snake.next_head(dir);

        if self.snake.overlap_tail(next_x, next_y) {//if snake head overlaps with tail
            music::play_sound(&SoundEffect::Die, music::Repeat::Times(0), music::MAX_VOLUME);
            return false;//return false
        }

        if self.snake.overlap_tail(self.obs_x, self.obs_y) {//if snake runs into obstacle
            music::play_sound(&SoundEffect::Die, music::Repeat::Times(0), music::MAX_VOLUME);
            return false;//return false
        }
        if self.snake.overlap_tail(self.obs_x+1, self.obs_y) {//if snake runs into obstacle
            music::play_sound(&SoundEffect::Die, music::Repeat::Times(0), music::MAX_VOLUME);
            return false;//return false
        }
        if self.snake.overlap_tail(self.obs_x, self.obs_y+1) {//if snake runs into obstacle
            music::play_sound(&SoundEffect::Die, music::Repeat::Times(0), music::MAX_VOLUME);
            return false;//return false
        }
        if self.snake.overlap_tail(self.obs_x+1, self.obs_y+1) {//if snake runs into obstacle
            music::play_sound(&SoundEffect::Die, music::Repeat::Times(0), music::MAX_VOLUME);
            return false;//return false
        }

        let result = next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height - 1; //if we go out of bounds
        if result == false {
            music::play_sound(&SoundEffect::Die, music::Repeat::Times(0), music::MAX_VOLUME);
            return false;
        }
        return true;
    }

    fn add_food(&mut self) {//adding food
        let mut rng = thread_rng();

        let mut new_x = rng.gen_range(1, self.width - 1);
        let mut new_y = rng.gen_range(1, self.height - 1);
        while self.snake.overlap_tail(new_x, new_y) {//we don't want snake to overlap with apple
            new_x = rng.gen_range(1, self.width - 1);
            new_y = rng.gen_range(1, self.height - 1);
        }

        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exists = true;
        let temp_type = rng.gen_range(1,4);
        if temp_type == 1 {
            self.food_type = "apple".to_string();
        }
        else if temp_type == 2 {
            self.food_type = "berry".to_string();
        }
        else if temp_type == 3 {
            self.food_type = "orange".to_string();
        }
    }

    fn update_snake(&mut self, dir: Option<Direction>) {

        if self.check_if_snake_alive(dir) {//if snake is alive
            self.snake.move_forward(dir);//then move snake forward
            self.check_eating(); //if snake ate a fruit
        } else {
            let mut window: PistonWindow =
            WindowSettings::new("Game Over!", [375; 2])
            .build().unwrap();
            let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
            let ref font = assets.join("FiraSans-Regular.ttf");
            let factory = window.factory.clone();
            let mut glyphs = Glyphs::new(font, factory, TextureSettings::new()).unwrap();

            if self.score > self.high_score{

                let data: String = self.score.to_string();
                let mut f = File::create("highscore.txt").expect("Unable to create file");
                f.write_all(data.as_bytes()).expect("Unable to write data");

                while let Some(e) = window.next() {//Game over menu
                    window.draw_2d(&e, |c, g| {
                    clear([0.5, 0.5, 0.5, 1.0], g);

                    text::Text::new_color(BLACK, 25)
                    .draw(
                    &format!("New High Score: {}", self.score),
                    &mut glyphs,
                    &c.draw_state,
                    c.transform.trans(80.0, 150.0),
                    g).unwrap();

                    text::Text::new_color(BLUE, 20)
                    .draw(
                    &format!("Choose from the below menu:"),
                    &mut glyphs,
                    &c.draw_state,
                    c.transform.trans(40.0, 200.0),
                    g).unwrap();

                    text::Text::new_color(BLUE,20)
                    .draw("Press 1 to Play Again",
                    &mut glyphs,
                    &c.draw_state,
                    c.transform.trans(40.0,250.0),
                    g).unwrap();

	                text::Text::new_color(BLUE,20)
                    .draw("Press 2 to Quit",
                    &mut glyphs,&c.draw_state,
                    c.transform.trans(40.0,300.0),
                    g).unwrap();
                    });

                if let Some(Button::Keyboard(theme)) = e.press_args() {

                let theme = match theme {
		        Key::D1 => window.set_should_close(true),
		        Key::D2 => process::exit(0x0100),
		        Key::NumPad1 => window.set_should_close(true),
		        Key::NumPad2 => process::exit(0x0100),
	      	    _ => window.set_should_close(true),
	          };
              }
              }
            }

            while let Some(e) = window.next() {//Game over menu
                window.draw_2d(&e, |c, g| {
                    clear([0.5, 0.5, 0.5, 1.0], g);

                    text::Text::new_color(BLACK, 25)
                    .draw(
                    &format!("Your score is: {}", self.score),
                    &mut glyphs,
                    &c.draw_state,
                    c.transform.trans(93.0, 150.0),
                    g).unwrap();

                    text::Text::new_color(BLUE, 20)
                    .draw(
                    &format!("Choose from the below menu:"),
                    &mut glyphs,
                    &c.draw_state,
                    c.transform.trans(40.0, 200.0),
                    g).unwrap();

                    text::Text::new_color(BLUE,20)
                    .draw("Press 1 to Play Again",
                    &mut glyphs,
                    &c.draw_state,
                    c.transform.trans(40.0,250.0),
                    g).unwrap();

	                text::Text::new_color(BLUE,20)
                    .draw("Press 2 to Quit",
                    &mut glyphs,&c.draw_state,
                    c.transform.trans(40.0,300.0),
                    g).unwrap();
                });
            if let Some(Button::Keyboard(theme)) = e.press_args() {

            let theme = match theme {
		        Key::D1 => window.set_should_close(true),
		        Key::D2 => process::exit(0x0100),
		        Key::NumPad1 => window.set_should_close(true),
		        Key::NumPad2 => process::exit(0x0100),
	      	    _ => window.set_should_close(true),
	         };
                
            }
          }
            self.game_over = true;//else game over
        }
        self.waiting_time = 0.0;//reset waiting time and restart game
    }

    fn restart(&mut self) {//this is similar to our new func but we don't want to call that because it will render a new window every reset
        self.snake = Snake::new(2, 2);//new board game
        self.waiting_time = 0.0;//start automatically
        self.food_exists = true;//food is available immediately
        self.food_x = 6;//food will start here
        self.food_y = 4;//food will start here
        self.food_type = "apple".to_string();
        self.obs_x = 25;
        self.obs_y = 5;
        self.game_over = false;//game over is false
        self.score = 0; // reset score to 0
    }
}
