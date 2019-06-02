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
