#[macro_use] extern crate text_io;

mod game_loop;
mod gameboard;
mod input;
mod space;

fn main() {
    game_loop::run();
}
