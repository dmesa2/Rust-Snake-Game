/*
Authors: Kennedy Hahn, Harriet Adkins, Dennis Mesanovic
Project: Rust implementation of the classic snake game
Course: CS410P Rust
Instructor: Professor Bart Massey

----------Below are the sources that we used in building this project----------
1.) https://www.youtube.com/watch?v=DnT_7M7L7vo
2.) https://github.com/tensor-programming/snake-tutorial (open source code with basic snake functionality)
3.) https://github.com/PistonDevelopers/piston
4.) https://www.youtube.com/watch?v=_oazUwpMpQg (Eat Sound Effect)
5.) https://www.youtube.com/watch?v=nQV7DKBqGdk (Under the Sea Background Music)
6.) https://www.youtube.com/watch?v=HoBa2SyvtpE (Die Sound Effect)
7.) https://www.fontsquirrel.com/fonts/list/popular (Main Menu Font)
8.) https://rustacean.net (Main Menu Image)
9.) https://github.com/lislis/manzana-attack/blob/master/src/main.rs (score)
-------------------------------------------------------------------------------

*/

/*
We are using the open source rust code from tensor-programming to build off of. This open source code consists of:

        - A basic board block that the snake will play in
        - A basic snake built in blocks
        - A randomly placed apple that when eaten the snake will grow by one block
        - If snake hits wall or self then the game automatically resets
        
        Features that we will add (or plan to):

        - A better UI (An actually looking snake, actual looking apple, better background)
        - Menu in the beginning that will consist of (easy, medium, hard) - The speed of the snake will depend on the difficulty
        - Sound effects (when snake bites itself, eats apple, hits a wall)
        - Score tracking (Perhaps keep track or previous scores to compare against)
        - Pop up menu that will declare "Game Over" at the end with the score
        - Perhaps a congratualations pop up if the user gets the snake to a big enough size and the game can't go any further
        - Menu at the end asking user to either Play Again or Quit
        - Add other elements such as berries for double points for limited time and oranges to cut snake's body (excluding head and tail) in half but keep score same (snake will not go below length 2)

        *
        Feel free to add more to this list!*
*/

#![allow(non_snake_case)]

extern crate piston_window;
extern crate rand;
extern crate music;
extern crate find_folder;

mod draw;//linking draw file
mod snake;//linking snake file
mod game;//linking game file

use piston_window::*;
use piston_window::types::Color;

use std::process;

use game::Game;
use draw::to_coord_u32;

use crate::game::SoundEffect;

use std::fs;
use std::str::FromStr;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];//back color will be gray
const WHITE: Color = [1.0, 1.0, 1.0, 0.50];//white color
const BLACK: Color = [0.0, 0.0, 0.0, 1.0];//black color
const BEACH_THEME: Color = [0.0, 0.0, 0.5, 1.0];
const DUNGEON_THEME: Color = [0.5, 0.5, 0.5, 1.0];
const FIELD_THEME: Color = [0.0, 0.9, 0.0, 0.8];
static mut MOVING_PERIOD: f64 = 0.0;
static mut NUM_PLAYERS: i32 = 1;
static mut THEME: Color = [0.0, 0.0, 0.0, 0.0];
static mut THEME_SONG: &str = "./sounds/beach_theme.wav";

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum BackgroundMusic {
    ThemeSong,
}

//let data = fs::read_to_string("highscore.txt").expect("Unable to read file");
//let highscore: i32 = FromStr::from_str(&data).unwrap();

fn main() {

//	let data = fs::read_to_string("highscore.txt").expect("Unable to read file");
//	highscore = FromStr::from_str(&data).unwrap();
//	println!("{}", highscore);

    let mut theme_menu: PistonWindow = WindowSettings::new("Theme Menu", [to_coord_u32(30),to_coord_u32(20)]).exit_on_esc(true).build().unwrap();
    let assets = find_folder::Search::ParentsThenKids(0, 0).for_folder("assets").unwrap();
    let ref font = assets.join("Roboto-Regular.ttf");
    let background_image0 = assets.join("rustacean-orig-noshadow.png");

    let factory2 = theme_menu.factory.clone();
    let mut glyphs = Glyphs::new(font, factory2,TextureSettings::new()).unwrap();
    let background_image: G2dTexture = Texture::from_path(&mut theme_menu.factory,&background_image0,Flip::None,&TextureSettings::new()).unwrap();

	let data = fs::read_to_string("highscore.txt").expect("Unable to read file");

    while let Some(e) = theme_menu.next() {
	theme_menu.draw_2d(&e, |c, g| {
	    clear([1.0,1.0,1.0,1.0], g);
	    image(&background_image, c.transform.scale(0.625,0.6), g);
	    text::Text::new_color([0.0,0.0,1.0,1.0],32).draw("THE SNAKE GAME (RUST CS410P VERSION)",&mut glyphs,&c.draw_state,c.transform.trans(10.0,100.0),g).unwrap();
	    text::Text::new_color([0.0,0.0,1.0,1.0],20).draw("Press 1 to play BEACH THEME",&mut glyphs,&c.draw_state,c.transform.trans(20.0,200.0),g).unwrap();
	    text::Text::new_color([0.0,0.0,1.0,1.0],20).draw("Press 2 to play FIELD THEME",&mut glyphs,&c.draw_state,c.transform.trans(20.0,300.0),g).unwrap();
	    text::Text::new_color([0.0,0.0,1.0,1.0],20).draw("Press 3 to play DUNGEON THEME",&mut glyphs,&c.draw_state,c.transform.trans(20.0,400.0),g).unwrap();
		text::Text::new_color([0.0,0.0,1.0,1.0],20).draw("Press 4 to EXIT",&mut glyphs,&c.draw_state,c.transform.trans(20.0,485.0),g).unwrap();
		text::Text::new_color([0.0,0.0,0.0,1.0],20).draw(&format!("Current score to beat: {}", data),&mut glyphs,&c.draw_state,c.transform.trans(235.0,150.0),g).unwrap();

	});
	  
   if let Some(Button::Keyboard(theme)) = e.press_args() {

	   let theme = match theme {
		Key::D1 => BEACH_THEME,
		Key::D2 => FIELD_THEME,
		Key::D3 => DUNGEON_THEME,
		Key::D4 => process::exit(0x0100),	
		Key::NumPad1 => BEACH_THEME,
		Key::NumPad2 => FIELD_THEME,
		Key::NumPad3 => DUNGEON_THEME,
		Key::NumPad3 => process::exit(0x0100),
		_ => BEACH_THEME,
	    };
	    
	    let theme_song = match theme {
		BEACH_THEME => "./sounds/beach_theme.wav",
		DUNGEON_THEME => "./sounds/dungeon_theme.wav",
		FIELD_THEME => "./sounds/field_theme.wav",
		_ => "./sounds/beach_theme.wav",
	     };     
	   unsafe{
	      THEME = theme;
	      THEME_SONG = theme_song;
	   }
                 
    let mut menu: PistonWindow = WindowSettings::new("Difficulty", [to_coord_u32(30),to_coord_u32(20)]).exit_on_esc(true).build().unwrap();
    let factory = menu.factory.clone();
    let mut glyphs2 = Glyphs::new(font, factory,TextureSettings::new()).unwrap();
    let background_image2: G2dTexture = Texture::from_path(&mut menu.factory,&background_image0,Flip::None,&TextureSettings::new()).unwrap();


    while let Some(e) = menu.next() {
	menu.draw_2d(&e, |c, g| {
	    clear([1.0,1.0,1.0,1.0], g);
	    image(&background_image2, c.transform.scale(0.625,0.6), g);
	    text::Text::new_color([0.0,0.0,1.0,1.0],32).draw("THE SNAKE GAME (RUST CS410P VERSION)",&mut glyphs2,&c.draw_state,c.transform.trans(10.0,100.0),g).unwrap();
	    text::Text::new_color([0.0,0.0,1.0,1.0],20).draw("Press 1 to play level EASY",&mut glyphs2,&c.draw_state,c.transform.trans(10.0,150.0),g).unwrap();
	    text::Text::new_color([0.0,0.0,1.0,1.0],20).draw("Press 2 to play level MEDIUM",&mut glyphs2,&c.draw_state,c.transform.trans(10.0,200.0),g).unwrap();
	    text::Text::new_color([0.0,0.0,1.0,1.0],20).draw("Press 3 to play level DIFFICULT",&mut glyphs2,&c.draw_state,c.transform.trans(10.0,250.0),g).unwrap();
		text::Text::new_color([0.0,0.0,1.0,1.0],20).draw("Press 4 to EXIT",&mut glyphs2,&c.draw_state,c.transform.trans(10.0,300.0),g).unwrap();

	 });
	   if let Some(Button::Keyboard(number)) = e.press_args() {
	   let level = match number {
		Key::D1 => Some(0.17),
		Key::D2 => Some(0.12),
		Key::D3 => Some(0.05),
		Key::D4 => process::exit(0x0100),
		Key::NumPad1 => Some(0.17),
		Key::NumPad2 => Some(0.12),
		Key::NumPad3 => Some(0.05),
		Key::NumPad4 =>  process::exit(0x0100),
		_ => Some(0.1),
	    };
	    let level_result = level.unwrap();
	    unsafe {
		if level_result > 0.0 {
		    MOVING_PERIOD = level_result;
		}
	    }
	    let mut player_menu: PistonWindow = WindowSettings::new("Players", [to_coord_u32(30),to_coord_u32(20)]).exit_on_esc(true).build().unwrap();
	    let factory = player_menu.factory.clone();
	    let mut glyphs2 = Glyphs::new(font, factory,TextureSettings::new()).unwrap();
	    let background_image2: G2dTexture = Texture::from_path(&mut player_menu.factory,&background_image0,Flip::None,&TextureSettings::new()).unwrap();


	    while let Some(e) = player_menu.next() {
		player_menu.draw_2d(&e, |c, g| {
		    clear([1.0,1.0,1.0,1.0], g);
		    image(&background_image2, c.transform.scale(0.625,0.6), g);
		    text::Text::new_color([0.0,0.0,1.0,1.0],32).draw("THE SNAKE GAME (RUST CS410P VERSION)",&mut glyphs2,&c.draw_state,c.transform.trans(10.0,100.0),g).unwrap();
		    text::Text::new_color([0.0,0.0,1.0,1.0],20).draw("Press 1 for ONE player",&mut glyphs2,&c.draw_state,c.transform.trans(10.0,150.0),g).unwrap();
		    text::Text::new_color([0.0,0.0,1.0,1.0],20).draw("Press 2 for TWO players (split screen)",&mut glyphs2,&c.draw_state,c.transform.trans(10.0,200.0),g).unwrap();
		    text::Text::new_color([0.0,0.0,1.0,1.0],20).draw("Press 3 for TWO players (melee)",&mut glyphs2,&c.draw_state,c.transform.trans(10.0,250.0),g).unwrap();
			text::Text::new_color([0.0,0.0,1.0,1.0],20).draw("Press 4 to EXIT",&mut glyphs2,&c.draw_state,c.transform.trans(10.0,300.0),g).unwrap();
		//	text::Text::new_color([0.0,0.0,0.0,1.0],20).draw(&format!("Current score to beat: {}", data),&mut glyphs2,&c.draw_state,c.transform.trans(10.0,300.0),g).unwrap();

		 });
		   if let Some(Button::Keyboard(players)) = e.press_args() {
		   let num_players = match players {
			Key::D1 => Some(1),
			Key::D2 => Some(2),
            Key::D3 => Some(3),
			Key::D4 => process::exit(0x0100),
			Key::NumPad1 => Some(1),
			Key::NumPad2 => Some(2), 
            Key::NumPad3 => Some(3),
			Key::NumPad4 =>  process::exit(0x0100),
			_ => Some(1),
		    };
		    let num_players_result = num_players.unwrap();
		    unsafe {
                        NUM_PLAYERS = num_players_result;
		    }
	 
          if num_players_result == 1 {
              launch_game(theme);
          }
          else if num_players_result == 2 {
              launch_two_player_game(theme);
          }
          else {
              launch_melee(theme);
          }
     } }}}}

 }

}
fn launch_melee(theme: Color) {
	    let (width, height) = (30, 30);

	    let mut window: PistonWindow =
		WindowSettings::new("Snake", [to_coord_u32(width), to_coord_u32(height)])//creates a new window
		    .exit_on_esc(true)//if we hit esc key then we will exit the game
		    .build()
		    .unwrap();//deals with any errors that may come along
	    let assets = find_folder::Search::ParentsThenKids(0, 0).for_folder("assets").unwrap();
	    let ref font = assets.join("Roboto-Regular.ttf");
	    let factory2 = window.factory.clone();
	    let mut glyphs = Glyphs::new(font, factory2,TextureSettings::new()).unwrap();
   
	        let mut p1_game = Game::new(theme, width, height,1, true,0);//
	        let mut p2_game = Game::new(theme, width, height,1, true,1);//create a new single-player game

			let data = fs::read_to_string("highscore.txt").expect("Unable to read file");
			p1_game.high_score = FromStr::from_str(&data).unwrap();

	        music::start::<BackgroundMusic, SoundEffect, _>(16, || {
                unsafe {
		    music::bind_music_file(BackgroundMusic::ThemeSong, THEME_SONG);
                }
		music::bind_sound_file(SoundEffect::Eat, "./sounds/eat.wav");
		music::bind_sound_file(SoundEffect::Die, "./sounds/die.wav");
		music::set_volume(music::MAX_VOLUME);
		music::play_music(&BackgroundMusic::ThemeSong, music::Repeat::Forever);

		while let Some(event) = window.next() {//cleans up window
	    if let Some(Button::Keyboard(key)) = event.press_args() {//if button is pushed
                
                if key == Key::A || key == Key::S || key == Key::D || key == Key::W { // P1 uses ASDW (left side of window)
		  p1_game.key_pressed(key);//pass the key
                }
                else {
                  p2_game.key_pressed(key);					      // P2 uses arrow keys (Right side of screen)
                }
	    }
	    window.draw_2d(&event, |c, g| {//else draw 2d window
		clear(theme, g);//clear window
		p1_game.draw(&c, g);//draw game
		p2_game.draw(&c, g);//draw game

	    text::Text::new_color(WHITE, 30)//display score
		.draw(
		&format!("P1 score: {}", p1_game.score),
		&mut glyphs,
		&c.draw_state,
		c.transform.trans(500.0, 55.0),
		g).unwrap(); 
		    

	    text::Text::new_color(WHITE, 30)//display score
		.draw(
		&format!("P2 score: {}", p2_game.score),
		&mut glyphs,
		&c.draw_state,
		c.transform.trans(1000.0, 55.0),
		g).unwrap(); 
		});


	    event.update(|arg| {
		p1_game.update(arg.dt);//delta time in seconds and arg is just a piston window (library stuff)
		p2_game.update(arg.dt);//delta time in seconds and arg is just a piston window (library stuff)
	    });
		}
                 });
}     

fn launch_two_player_game(theme: Color) {
        let (width, height) = (30, 30);


        let mut window: PistonWindow =
	  WindowSettings::new("Two-player Snake", [to_coord_u32(width*2), to_coord_u32(height)])//creates a new window
	    .exit_on_esc(true)//if we hit esc key then we will exit the game
	    .build()
	    .unwrap();//deals with any errors that may come along

        let assets = find_folder::Search::ParentsThenKids(0, 0).for_folder("assets").unwrap();
        let ref font = assets.join("Roboto-Regular.ttf");
        let factory2 = window.factory.clone();
        let mut glyphs = Glyphs::new(font, factory2,TextureSettings::new()).unwrap();
   
	let mut p1_game = Game::new(theme, width, height, 1, true, 0);//P1's game
	let mut p2_game = Game::new(theme, width, height, 2, true, 0);//P2's game

	let data = fs::read_to_string("highscore.txt").expect("Unable to read file");
	p1_game.high_score = FromStr::from_str(&data).unwrap();

	music::start::<BackgroundMusic, SoundEffect, _>(16, || {
	unsafe {
	    music::bind_music_file(BackgroundMusic::ThemeSong, THEME_SONG);
	}
	music::bind_sound_file(SoundEffect::Eat, "./sounds/eat.wav");
	music::bind_sound_file(SoundEffect::Die, "./sounds/die.wav");
	music::set_volume(music::MAX_VOLUME);
	music::play_music(&BackgroundMusic::ThemeSong, music::Repeat::Forever);

	while let Some(event) = window.next() {//cleans up window
	    if let Some(Button::Keyboard(key)) = event.press_args() {//if button is pushed
                
                if key == Key::A || key == Key::S || key == Key::D || key == Key::W { // P1 uses ASDW (left side of window)
		  p1_game.key_pressed(key);//pass the key
                }
                else {
                  p2_game.key_pressed(key);					      // P2 uses arrow keys (Right side of screen)
                }
	    }
	    window.draw_2d(&event, |c, g| {//else draw 2d window
		clear(theme, g);//clear window
		p1_game.draw(&c, g);//draw game
		p2_game.draw(&c, g);//draw game

	    text::Text::new_color(WHITE, 30)//display score
		.draw(
		&format!("P1 score: {}", p1_game.score),
		&mut glyphs,
		&c.draw_state,
		c.transform.trans(500.0, 55.0),
		g).unwrap(); 
		    

	    text::Text::new_color(WHITE, 30)//display score
		.draw(
		&format!("P2 score: {}", p2_game.score),
		&mut glyphs,
		&c.draw_state,
		c.transform.trans(1000.0, 55.0),
		g).unwrap(); 
		});


	    event.update(|arg| {
		p1_game.update(arg.dt);//delta time in seconds and arg is just a piston window (library stuff)
		p2_game.update(arg.dt);//delta time in seconds and arg is just a piston window (library stuff)
	    });
	  }
	 });
}     

fn launch_game(theme: Color) {
	    let (width, height) = (30, 30);


	    let mut window: PistonWindow =
		WindowSettings::new("Snake", [to_coord_u32(width), to_coord_u32(height)])//creates a new window
		    .exit_on_esc(true)//if we hit esc key then we will exit the game
		    .build()
		    .unwrap();//deals with any errors that may come along
	    let assets = find_folder::Search::ParentsThenKids(0, 0).for_folder("assets").unwrap();
	    let ref font = assets.join("Roboto-Regular.ttf");
	    let factory2 = window.factory.clone();
	    let mut glyphs = Glyphs::new(font, factory2,TextureSettings::new()).unwrap();
   
	        let mut game = Game::new(theme, width, height,1, false, 0);//create a new single-player game

			let data = fs::read_to_string("highscore.txt").expect("Unable to read file");
			game.high_score = FromStr::from_str(&data).unwrap();

	        music::start::<BackgroundMusic, SoundEffect, _>(16, || {
                unsafe {
		    music::bind_music_file(BackgroundMusic::ThemeSong, THEME_SONG);
                }
		music::bind_sound_file(SoundEffect::Eat, "./sounds/eat.wav");
		music::bind_sound_file(SoundEffect::Die, "./sounds/die.wav");
		music::set_volume(music::MAX_VOLUME);
		music::play_music(&BackgroundMusic::ThemeSong, music::Repeat::Forever);

		while let Some(event) = window.next() {//cleans up window
		    if let Some(Button::Keyboard(key)) = event.press_args() {//if button is pushed
			game.key_pressed(key);//pass the key
		    }
		    window.draw_2d(&event, |c, g| {//else draw 2d window
			clear(theme, g);//clear window
			game.draw(&c, g);//draw game

		    text::Text::new_color(WHITE, 30)//display score
			.draw(
			&format!("Score: {}", game.score),
			&mut glyphs,
			&c.draw_state,
			c.transform.trans(540.0, 55.0),
			g).unwrap(); 
			    });


		    event.update(|arg| {
			game.update(arg.dt);//delta time in seconds and arg is just a piston window (library stuff)
		    });
		  }
                 });
}     
