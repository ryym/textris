use std::io;
use std::sync::mpsc::{channel, Receiver};
use std::thread;
use termion::event::Event;
use termion::input::Events;

pub type EventResult = Result<Event, io::Error>;

// https://users.rust-lang.org/t/alias-for-trait-bounds/8198
pub trait EventStream: Iterator<Item = EventResult> + Send {}
impl<R: io::Read + Send> EventStream for Events<R> {}

pub struct Inputs {
    receiver: Receiver<EventResult>,
}

impl Inputs {
    pub fn new<ES: 'static + EventStream>(events: ES) -> Inputs {
        let (sender, receiver) = channel();
        thread::spawn(move || {
            for event in events {
                sender.send(event).expect("send event from Inputs");
            }
        });
        Inputs { receiver }
    }

    pub fn try_recv(&mut self) -> Option<EventResult> {
        match self.receiver.try_recv() {
            Ok(event) => Some(event),
            Err(_) => None,
        }
    }
}
