// Originally by sebglazebrook

use std::time::Duration;
use std::thread::sleep;
use std::io::{ self, Write };

pub struct Countdown {
    duration: usize,
    pub running: bool,
}

impl Countdown {
    pub fn new(duration: usize)-> Self {
        Self {
            duration,
            running: false
        }
    }

    pub fn start(&mut self) {
        let mut duration_remaining = self.duration;
        self.running = true;
        while duration_remaining > 0 {
            self.countdown_one_second_from();
            duration_remaining -= 1;
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