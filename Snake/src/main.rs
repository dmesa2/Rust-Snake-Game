/*
Authors: Kennedy Hahn, Harriet Adkins, Dennis Mesanovic
Project: Rust implementation of the classic snake game
Course: CS410P Rust
Instructor: Professor Bart Massey

----------Below are the sources that we used in building this project----------
1.) https://www.youtube.com/watch?v=DnT_7M7L7vo
2.) https://github.com/tensor-programming/snake-tutorial
3.) https://github.com/PistonDevelopers/piston
4.) https://www.youtube.com/watch?v=_oazUwpMpQg (Eat Sound Effect)
5.) https://www.youtube.com/watch?v=nQV7DKBqGdk (Under the Sea Background Music)
6.) https://www.youtube.com/watch?v=HoBa2SyvtpE (Die Sound Effect)
7.) https://www.fontsquirrel.com/fonts/list/popular (Main Menu Font)
8.) https://rustacean.net (Main Menu Image)
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
        - Add other elements such as berries for double points for limited time and/or grapes to cut snake in half but keep score same

        *
        Feel free to add more to this list!*
*/


extern crate piston_window;
extern crate rand;
extern crate music;
extern crate find_folder;

mod draw;//linking draw file
mod snake;//linking snake file
mod game;//linking game file

use piston_window::*;
use piston_window::types::Color;

use game::Game;
use draw::to_coord_u32;

use crate::game::SoundEffect;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];//back color will be gray
const BEACH_THEME: Color = [0.0, 0.0, 0.5, 1.0];
const DUNGEON_THEME: Color = [0.5, 0.5, 0.5, 1.0];
const FIELD_THEME: Color = [0.0, 0.9, 0.0, 0.8];
static mut MOVING_PERIOD: f64 = 0.0;
static mut THEME: Color = [0.0, 0.0, 0.0, 0.0];
static mut THEME_SONG: &str = "./sounds/beach_theme.wav";

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum BackgroundMusic {
    ThemeSong,
}


fn main() {

    let mut theme_menu: PistonWindow = WindowSettings::new("Theme Menu", [to_coord_u32(30),to_coord_u32(20)]).exit_on_esc(true).build().unwrap();
    let assets = find_folder::Search::ParentsThenKids(0, 0).for_folder("assets").unwrap();
    let ref font = assets.join("Roboto-Regular.ttf");
    let background_image0 = assets.join("rustacean-orig-noshadow.png");

    let factory2 = theme_menu.factory.clone();
    let mut glyphs = Glyphs::new(font, factory2,TextureSettings::new()).unwrap();
    let background_image: G2dTexture = Texture::from_path(&mut theme_menu.factory,&background_image0,Flip::None,&TextureSettings::new()).unwrap();

    while let Some(e) = theme_menu.next() {
	theme_menu.draw_2d(&e, |c, g| {
	    clear([1.0,1.0,1.0,1.0], g);
	    image(&background_image, c.transform.scale(0.625,0.6), g);
	    text::Text::new_color([0.0,0.0,1.0,1.0],32).draw("THE SNAKE GAME (RUST CS410P VERSION)",&mut glyphs,&c.draw_state,c.transform.trans(10.0,100.0),g).unwrap();
	    text::Text::new_color([0.0,0.0,1.0,1.0],20).draw("Press 1 to play BEACH THEME",&mut glyphs,&c.draw_state,c.transform.trans(20.0,200.0),g).unwrap();
	    text::Text::new_color([0.0,0.0,1.0,1.0],20).draw("Press 2 to play FIELD THEME",&mut glyphs,&c.draw_state,c.transform.trans(20.0,300.0),g).unwrap();
	    text::Text::new_color([0.0,0.0,1.0,1.0],20).draw("Press 3 to play DUNGEON THEME",&mut glyphs,&c.draw_state,c.transform.trans(20.0,400.0),g).unwrap();

	});
	  
   if let Some(Button::Keyboard(theme)) = e.press_args() {

	   let theme = match theme {
		Key::D1 => BEACH_THEME,
		Key::D2 => FIELD_THEME,
		Key::D3 => DUNGEON_THEME,
		Key::NumPad1 => BEACH_THEME,
		Key::NumPad2 => FIELD_THEME,
		Key::NumPad3 => DUNGEON_THEME,
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

	 });
	   if let Some(Button::Keyboard(number)) = e.press_args() {
	   let level = match number {
		Key::D1 => Some(0.17),
		Key::D2 => Some(0.12),
		Key::D3 => Some(0.05),
		Key::NumPad1 => Some(0.17),
		Key::NumPad2 => Some(0.12),
		Key::NumPad3 => Some(0.05),
		_ => Some(0.1),
	    };
	    let level_result = level.unwrap();
	    unsafe {
		if level_result > 0.0 {
		    MOVING_PERIOD = level_result;
		}
	    }
	 
//        unsafe {     
        //    launch_game(THEME);
 //       }
          launch_game(theme);
     } }}

 }

}

fn launch_game(theme: Color) {
	    let (width, height) = (30, 30);
   
	    let mut window: PistonWindow =
		WindowSettings::new("Snake", [to_coord_u32(width), to_coord_u32(height)])//creates a new window
		    .exit_on_esc(true)//if we hit esc key then we will exit the game
		    .build()
		    .unwrap();//deals with any errors that may come along
	        let mut game = Game::new(width, height, theme);//create a new game
          
	        music::start::<BackgroundMusic, SoundEffect, _>(16, || {
                unsafe {
		    music::bind_music_file(BackgroundMusic::ThemeSong, THEME_SONG);
                }
		music::bind_sound_file(SoundEffect::Eat, "./sounds/eat.wav");
		music::bind_sound_file(SoundEffect::Die, "./sounds/die.wav");
		music::set_volume(music::MAX_VOLUME);
		music::play_music(&BackgroundMusic::ThemeSong, music::Repeat::Forever);

		while let Some(event) = window.next() {//cleans up window - every time snake moves window is cleaned
		    if let Some(Button::Keyboard(key)) = event.press_args() {//if button is pushed
			game.key_pressed(key);//pass the key
		    }
		    window.draw_2d(&event, |c, g| {//else draw 2d window
			    clear(theme, g);//clear window
			game.draw(&c, g);//draw game
		    });

		    event.update(|arg| {
			game.update(arg.dt);//delta time in seconds and arg is just a piston window (library stuff)
		    });
		  }
                 });
}     
            


