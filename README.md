# RustProject - Snake Game
Rust Project for CS410P

## Engine
This project uses the Piston Engine

## Building

Building on macOS

Make sure cargo is installed locally using rustup. Clone the repo.

The following packages need to be installed:

First run the following: 

```
$ cargo check
```

Followed by (assuming you have brew installed): 

```
$ brew install sdl2
```
and

```
$ brew install sdl2_mixer
```

Building on Linux

Same instructions as above but instead of installing sdl2 and sdl2_mixer, 
the following should be installed instead: libsdl2-mixer-2.0-0 and libsdl2-mixer-dev

## How to run

cd into Snake/src

```
 $ cargo run
```

## Instructions

Choose any of the options from the menu

Use the arrow keys to change direction of the snake

Apples - 1 point and snake grows one block
Berries - 2 points and snake grows one block
Orange - 0 point and snake is cut in half

Avoid hitting the walls or any obstacles. Your score is tracked in the top right corner.
Your objective is to beat the high score which is saved to a text-file and overwritten if
that score is beaten.

## Sources

Our main source for this project comes from tensor programming. We used this source code and built off of it and added a lot of features. 
These features are commented at the top in the main.rs file.

1.) https://github.com/tensor-programming/snake-tutorial 
2.)  https://www.youtube.com/watch?v=_oazUwpMpQg (Eat Sound Effect) 
3.) https://github.com/PistonDevelopers/piston
4.) https://www.youtube.com/watch?v=nQV7DKBqGdk (Under the Sea Background Music)
5.) https://www.youtube.com/watch?v=HoBa2SyvtpE (Die Sound Effect) 
6.) https://www.fontsquirrel.com/fonts/list/popular(Main Menu Font)
7.) https://rustacean.net (Main Menu Image)
8.) https://github.com/lislis/manzana-attack/blob/master/src/main.rs (score)

## Project Writeup

For this project, we added the following features: 

        - Menu in the beginning that will allow players to configure game difficulty, theme, and mode (single vs. two player)
        - Different themes (beach, field, and dungeon) with background music to match each
        - Sound effects when snake eats and dies
        - Score tracking for each game as well as a high score over all games
        - Pop up menu that will declare "Game Over" at the end with the score and give players options to play again or quit
        - More food options (berries for double points, oranges for zero points that halve snake's body length)
        - A poison apple obstacle that appears when score >= 20 and score % 20 = 0, and disappears when score % 5 = 0 (and score % 20 != 0)
        - Two-player splitscreen mode

These features were all implemented successfully. We also worked on better graphics for the game (turning snake into line of crabs), but didn't finish its implementation. Adding unit tests was also difficult to figure out given the game's structure. We learned about the process of working on a project using Rust, including the importance of research when planning to implement something unfamiliar.
