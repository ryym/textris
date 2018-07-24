use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Action {
    Ok,
    Reset,
    Retry,
    Quit,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
