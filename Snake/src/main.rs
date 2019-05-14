/*
Authors: Kennedy Hahn, Harriet Adkins, Dennis Mesanovic
Project: Rust implementation of the classic snake game
Course: CS410P Rust
Instructor: Professor Bart Massey

----------Below are the sources that we used in building this project----------
1.) https://www.youtube.com/watch?v=DnT_7M7L7vo
2.) https://github.com/tensor-programming/snake-tutorial
3.) https://github.com/PistonDevelopers/piston
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

mod draw;//linking draw file
mod snake;//linking snake file
mod game;//linking game file

use piston_window::*;
use piston_window::types::Color;

use game::Game;
use draw::to_coord_u32;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];//back color will be gray



fn main() {
    let (width, height) = (30, 30);

    let mut window: PistonWindow =
        WindowSettings::new("Snake", [to_coord_u32(width), to_coord_u32(height)])//creates a new window
            .exit_on_esc(true)//if we hit esc key then we will exit the game
            .build()
            .unwrap();//deals with any errors that may come along

    let mut game = Game::new(width, height);//create a new game
    while let Some(event) = window.next() {//cleans up window - every time snake moves window is cleaned
        if let Some(Button::Keyboard(key)) = event.press_args() {//if button is pushed
            game.key_pressed(key);//pass the key
        }
        window.draw_2d(&event, |c, g| {//else draw 2d window
            clear(BACK_COLOR, g);//clear window
            game.draw(&c, g);//draw game
        });

        event.update(|arg| {
            game.update(arg.dt);//delta time in seconds and arg is just a piston window (library stuff)
        });
    }
}