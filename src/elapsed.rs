use std::fmt;

pub struct Elapsed {
    hours: u64,
    minutes: u64,
    seconds: u64,
}

impl Elapsed {
    pub fn new() -> Self {
        Elapsed {
            hours: 0,
            minutes: 0,
            seconds: 0,
        }
    }

    pub fn add_secs(&mut self, added: u64) {
        let secs = self.hours * 3600 + self.minutes * 60 + self.seconds + added;
        self.hours = secs / 3600;
        self.minutes = (secs % 3600) / 60;
        self.seconds = secs % 60;
    }
}

impl fmt::Display for Elapsed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:02}:{:02}:{:02}",
            self.hours, self.minutes, self.seconds
        )
    }
}
