//! Textris is a hobby project to play tetris on your terminal.

#[macro_use]
extern crate failure;
extern crate getopts;
extern crate rand;
extern crate termion;

pub mod action;
mod block;
pub mod cli;
mod color;
pub mod coord;
mod elapsed;
mod field;
pub mod game;
pub mod inputs;
mod play;
pub mod screen;
mod tetromino;
