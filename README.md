# Rust-Snake-Game

## Implementation of the classic snake game written in Rust. 

## ----------Below are the sources that we used in building this project----------

1.) https://www.youtube.com/watch?v=DnT_7M7L7vo
2.) https://github.com/tensor-programming/snake-tutorial
3.) https://github.com/PistonDevelopers/piston
4.) https://www.youtube.com/watch?v=_oazUwpMpQg (Eat Sound Effect)
5.) https://www.youtube.com/watch?v=nQV7DKBqGdk (Under the Sea Background Music)
6.) https://www.youtube.com/watch?v=HoBa2SyvtpE (Die Sound Effect)
7.) https://www.fontsquirrel.com/fonts/list/popular (Main Menu Font)
8.) https://rustacean.net (Main Menu Image)

-------------------------------------------------------------------------------

## We are using the open source rust code from tensor-programming to build off of. This open source code consists of:

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

        *Feel free to add more to this list!*

## Some of the crates used in this project depend on sdl2 and sdl2 mixer to be installed locally, and the repository must be downloaded and run locally as well.

## Instructions

        1. Download the repository locally, and make sure sdl2 and sdl2_mixer are installed. (We used brew install sdl2 and brew install sdl2_mixer)
        2. cd into src
        3. type 'cargo check' followed by 'cargo run'
        4. Enjoy!
