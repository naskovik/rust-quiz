// Originally by sebglazebrook

use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

pub struct Countdown {
    remaining: usize,
    pub running: bool,
}

impl Countdown {
    pub fn new(duration: usize) -> Self {
        Self {
            remaining: duration,
            running: false,
        }
    }

    pub fn start(&mut self) {
        self.running = true;
        while self.remaining > 0 {
            self.countdown_one_second_from();
            self.remaining -= 1;
        }
        self.running = false;
    }

    fn countdown_one_second_from(&self) {
        let quarter_of_second = Duration::from_millis(250);
        for _ in 0..3 {
            io::stdout().flush().unwrap();
            sleep(quarter_of_second);
        }
    }
}
