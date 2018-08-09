use super::{EventReader, KeyConverter, Order};
use errors::*;
use std::io;
use std::sync::mpsc::{channel, Receiver, RecvError, TryRecvError};
use std::thread;
use termion::event::{Event, Key};
use termion::input::Events;

pub type EventResult = io::Result<Event>;

// https://users.rust-lang.org/t/alias-for-trait-bounds/8198
pub trait EventStream: Iterator<Item = EventResult> + Send {}
impl<R: io::Read + Send> EventStream for Events<R> {}

pub struct Inputs {
    receiver: Receiver<EventResult>,
    converter: EventReader,
}

impl Inputs {
    pub fn new<ES: 'static + EventStream>(events: ES, key: KeyConverter) -> Inputs {
        let (sender, receiver) = channel();
        thread::spawn(move || {
            for event in events {
                sender.send(event).expect("send event from Inputs");
            }
        });

        let converter = EventReader::new(key);
        Inputs {
            receiver,
            converter,
        }
    }

    pub fn recv_event(&mut self) -> Result<EventResult> {
        self.receiver.recv().map_err(|err| err.into())
    }

    pub fn try_recv_event(&mut self) -> Result<Option<EventResult>> {
        match self.receiver.try_recv() {
            Ok(event) => Ok(Some(event)),
            Err(TryRecvError::Empty) => Ok(None),
            Err(TryRecvError::Disconnected) => Err(RecvError.into()),
        }
    }

    pub fn recv_order(&mut self) -> Result<io::Result<Order>> {
        loop {
            match self.recv_event()? {
                Ok(event) => {
                    if let Some(order) = self.converter.order(event) {
                        return Ok(Ok(order));
                    }
                }
                Err(err) => {
                    return Ok(Err(err));
                }
            }
        }
    }

    pub fn try_recv_order(&mut self) -> Result<Option<io::Result<Order>>> {
        match self.try_recv_event()? {
            Some(event) => {
                let order = match event {
                    Ok(event) => self.converter.order(event).map(|o| Ok(o)),
                    Err(err) => Some(Err(err)),
                };
                Ok(order)
            }
            None => Ok(None),
        }
    }

    pub fn bound_key(&self, order: Order) -> Key {
        self.converter.bound_key(order)
    }
}
