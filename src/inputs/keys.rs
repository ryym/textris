use super::Order;
use coord::{Dir, RotateDir};
use std::collections::HashMap;
use termion::event::Key;

macro_rules! hash_map {
    ($($key:expr => $val:expr),*$(,)*) => {{
        let mut m = HashMap::new();
        $(m.insert($key, $val);)*
        m
    }}
}

pub enum KeyConverter {
    Vim,
}

impl KeyConverter {
    pub fn key_to_order(&self) -> HashMap<Key, Order> {
        use super::Order::*;
        match self {
            KeyConverter::Vim => hash_map!{
                Key::Char('h') => Move(Dir::Left),
                Key::Char('l') => Move(Dir::Right),
                Key::Char('j') => Move(Dir::Down),
                Key::Char('d') => Rotate(RotateDir::AntiClockwise),
                Key::Char('f') => Rotate(RotateDir::Clockwise),
                Key::Char('\n') => Select,
                Key::Char('?') => Help,
                Key::Char('q') => Quit,
            },
        }
    }
}
