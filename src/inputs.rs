use errors::*;
use std::io;
use std::sync::mpsc::{channel, Receiver, RecvError, TryRecvError};
use std::thread;
use termion::event::{Event, Key};
use termion::input::Events;

pub type EventResult = io::Result<Event>;
pub type KeyResult = io::Result<Key>;

// https://users.rust-lang.org/t/alias-for-trait-bounds/8198
pub trait EventStream: Iterator<Item = EventResult> + Send {}
impl<R: io::Read + Send> EventStream for Events<R> {}

pub struct Inputs {
    receiver: Receiver<EventResult>,
}

fn filter_key_event(event: EventResult) -> Option<KeyResult> {
    match event {
        Ok(Event::Key(key)) => Some(Ok(key)),
        Ok(_) => None,
        Err(err) => Some(Err(err)),
    }
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

    pub fn recv_event(&mut self) -> Result<EventResult> {
        self.receiver.recv().map_err(|err| err.into())
    }

    pub fn recv_key(&mut self) -> Result<KeyResult> {
        loop {
            if let Some(key) = filter_key_event(self.recv_event()?) {
                return Ok(key);
            }
        }
    }

    pub fn try_recv_event(&mut self) -> Result<Option<EventResult>> {
        match self.receiver.try_recv() {
            Ok(event) => Ok(Some(event)),
            Err(TryRecvError::Empty) => Ok(None),
            Err(TryRecvError::Disconnected) => Err(RecvError.into()),
        }
    }

    pub fn try_recv_key(&mut self) -> Result<Option<KeyResult>> {
        self.try_recv_event()
            .map(|event| event.and_then(|ev| filter_key_event(ev)))
    }
}
