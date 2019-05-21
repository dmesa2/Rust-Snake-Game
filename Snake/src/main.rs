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
const WHITE: Color = [1.0, 1.0, 1.0, 0.50];//white color

static mut MOVING_PERIOD: f64 = 0.0;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum BackgroundMusic {
    ThemeSong,
}


fn main() {
    let mut menu: PistonWindow = WindowSettings::new("Main Menu", [to_coord_u32(30),to_coord_u32(20)]).exit_on_esc(true).build().unwrap();
    let assets = find_folder::Search::ParentsThenKids(0, 0).for_folder("assets").unwrap();
    let ref font = assets.join("Roboto-Regular.ttf");
    let factory = menu.factory.clone();
    let mut glyphs = Glyphs::new(font, factory,TextureSettings::new()).unwrap();
    let background_image = assets.join("rustacean-orig-noshadow.png");
    let background_image: G2dTexture = Texture::from_path(&mut menu.factory,&background_image,Flip::None,&TextureSettings::new()).unwrap();
    while let Some(e) = menu.next() {
        menu.draw_2d(&e, |c, g| {
            clear([1.0,1.0,1.0,1.0], g);
            image(&background_image, c.transform.scale(0.625,0.6), g);
            text::Text::new_color([0.0,0.0,1.0,1.0],32).draw("THE SNAKE GAME (RUST CS410P VERSION)",&mut glyphs,&c.draw_state,c.transform.trans(10.0,100.0),g).unwrap();
            text::Text::new_color([0.0,0.0,1.0,1.0],32).draw("Press 1 to play level EASY",&mut glyphs,&c.draw_state,c.transform.trans(20.0,200.0),g).unwrap();
            text::Text::new_color([0.0,0.0,1.0,1.0],32).draw("Press 2 to play level MEDIUM",&mut glyphs,&c.draw_state,c.transform.trans(20.0,300.0),g).unwrap();
            text::Text::new_color([0.0,0.0,1.0,1.0],32).draw("Press 3 to play level DIFFICULT",&mut glyphs,&c.draw_state,c.transform.trans(20.0,400.0),g).unwrap();
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
            let result = level.unwrap();
            unsafe {
                if result > 0.0 {
                    MOVING_PERIOD = result;
                }
            }
            
	    let (width, height) = (30, 30);

	    let mut window: PistonWindow =
		WindowSettings::new("Snake", [to_coord_u32(width), to_coord_u32(height)])//creates a new window
		    .exit_on_esc(true)//if we hit esc key then we will exit the game
		    .build()
		    .unwrap();//deals with any errors that may come along

	    let mut game = Game::new(width, height);//create a new game

	    music::start::<BackgroundMusic, SoundEffect, _>(16, || {
		music::bind_music_file(BackgroundMusic::ThemeSong, "./sounds/theme.wav");
		music::bind_sound_file(SoundEffect::Eat, "./sounds/eat.wav");
		music::bind_sound_file(SoundEffect::Die, "./sounds/die.wav");
		music::set_volume(music::MAX_VOLUME);
		music::play_music(&BackgroundMusic::ThemeSong, music::Repeat::Forever);

        while let Some(event) = window.next() {//cleans up window - every time snake moves window is cleaned

		    if let Some(Button::Keyboard(key)) = event.press_args() {//if button is pushed
			game.key_pressed(key);//pass the key
		    }
		    window.draw_2d(&event, |c, g| {//else draw 2d window
			clear(BACK_COLOR, g);//clear window
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
    }

}