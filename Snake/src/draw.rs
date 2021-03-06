//use piston_window::{rectangle, Context, G2d, G2dTexture, image};
use piston_window::types::Color;
use piston_window::{rectangle, Context, G2d};

const BLOCK_SIZE: f64 = 25.0; //Blocks will scale up to 25 pixels

pub fn to_coord(game_coord: i32) -> f64 {
    //Take in a coord and then cast it as f64 then mult by block size
    f64::from(game_coord) * BLOCK_SIZE
}

pub fn to_coord_u32(game_coord: i32) -> u32 {
    to_coord(game_coord) as u32
}

pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    //Helper func - Draw a block
    let gui_x = to_coord(x); //bind gui and pass it to_coord to convert to f64
    let gui_y = to_coord(y);

    rectangle(
        color,                                  //pass in color
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE], //passing in parameters
        con.transform,                          //pass in context transform
        g,                                      //graphics buffer
    );
}

pub fn draw_rectangle(
    //allows us to draw rectangles
    color: Color, //passing in color
    x: i32,       //passing in x
    y: i32,       //passing in y
    width: i32,
    height: i32,
    con: &Context,
    g: &mut G2d,
) {
    let x = to_coord(x);
    let y = to_coord(y);

    rectangle(
        color,
        [
            x,
            y,
            BLOCK_SIZE * f64::from(width), //cast Block Size by width and height and convert to f64
            BLOCK_SIZE * f64::from(height),
        ],
        con.transform,
        g,
    );
}
/*
pub fn draw_crab(x: f64, y: f64, c: &Context, g: &mut G2d, background_image: G2dTexture) {
    let gui_x = to_coord(x as i32);
    let gui_y = to_coord(y as i32);
    let to_draw = image::Image::new().rect([gui_x,gui_y,BLOCK_SIZE,BLOCK_SIZE]);
    to_draw.draw(&background_image,&c.draw_state,c.transform,g);
//    text::Text::new_color([1.0,0.0,0.0,1.0],32).draw("\u{1f980}",glyphs,&c.draw_state,c.transform.trans(gui_x,gui_y),g).unwrap();
}*/
