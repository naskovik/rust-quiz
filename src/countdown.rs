use std::time::Duration;
use std::thread::sleep;
use std::io::{ self, Write };

pub struct Countdown {
    duration: usize,
}

impl Countdown {
    pub fn new(duration: usize)-> Self {
        Self {
            duration,
        }
    }

    pub fn start(&self) {
        let mut duration_remaining = self.duration;
        while duration_remaining > 0 {
            self.countdown_one_second_from();
            duration_remaining -= 1;
        }
    }

    fn countdown_one_second_from(&self) {
        let quarter_of_second = Duration::from_millis(250);
        for _ in 0..3 {
            io::stdout().flush().unwrap();
            sleep(quarter_of_second);
        }
    }
}