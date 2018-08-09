use super::{KeyConverter, Order};
use std::collections::HashMap;
use termion::event::{Event, Key};

pub struct EventReader {
    key2order: HashMap<Key, Order>,
}

impl EventReader {
    pub fn new(keys: KeyConverter) -> Self {
        EventReader {
            key2order: keys.key_to_order(),
        }
    }

    pub fn order(&self, input: Event) -> Option<Order> {
        if let Event::Key(key) = input {
            self.key2order.get(&key).map(|order| *order)
        } else {
            None
        }
    }

    pub fn bound_key(&self, desired_order: Order) -> Key {
        for (&key, &order) in self.key2order.iter() {
            if order == desired_order {
                return key;
            }
        }
        panic!(
            "Order {:?} does not be associated with any key",
            desired_order
        );
    }
}
