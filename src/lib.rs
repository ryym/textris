//! Textris is a hobby project to play tetris on your terminal.

#[macro_use]
extern crate error_chain;
extern crate rand;
extern crate termion;

pub mod action;
mod block;
mod color;
pub mod coord;
mod elapsed;
mod errors;
mod field;
pub mod game;
pub mod inputs;
mod play;
pub mod screen;
mod tetromino;
