use errors::*;
use std::io;
use std::sync::mpsc::{channel, Receiver, RecvError, TryRecvError};
use std::thread;
use termion::event::Event;
use termion::input::Events;

pub type EventResult = io::Result<Event>;

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

    pub fn try_recv(&mut self) -> Result<Option<EventResult>> {
        match self.receiver.try_recv() {
            Ok(event) => Ok(Some(event)),
            Err(TryRecvError::Empty) => Ok(None),
            Err(TryRecvError::Disconnected) => Err(RecvError.into()),
        }
    }
}
