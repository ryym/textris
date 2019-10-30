use std::fmt;

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Elapsed {
    hours: u64,
    minutes: u64,
    seconds: u64,
}

impl Elapsed {
    pub fn new() -> Self {
        Elapsed::default()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn elapsed_display() {
        let e = Elapsed {
            hours: 7,
            minutes: 34,
            seconds: 52,
        };
        assert_eq!(format!("{}", e), "07:34:52");
    }

    #[test]
    fn elapsed_add_secs() {
        struct Case {
            secs: u64,
            expected: Elapsed,
        };
        let cases = [
            Case {
                secs: 30,
                expected: Elapsed {
                    hours: 0,
                    minutes: 0,
                    seconds: 30,
                },
            },
            Case {
                secs: 5001,
                expected: Elapsed {
                    hours: 1,
                    minutes: 23,
                    seconds: 21,
                },
            },
        ];
        for (i, Case { secs, expected }) in cases.iter().enumerate() {
            let mut e = Elapsed::new();
            e.add_secs(*secs);
            assert_eq!(e, *expected, "test[{}]", i);
        }
    }
}
