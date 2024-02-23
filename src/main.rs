mod countdown;
mod problem;

use crate::{
    countdown::Countdown,
    problem::Problem
};
use std::error::Error;
use std::{process, io};
use std::sync::mpsc;
use std::thread;

const QUIZ_TIME: usize = 30;

fn read_csv_file(path: &str)
    -> Result<Vec<Problem>, Box<dyn Error>>
{
    let mut problems: Vec<Problem> = Vec::new();
    let mut reader = csv::Reader::from_path(path)?;

    for record in reader.records() {
        let result = record?;
        problems.push(
            Problem::new(
                result[0].to_string(),
                result[1].to_string()
            )
        );

    }

    Ok(problems)
}


fn main() {
    let file = std::env::args()
        .nth(1)
        .expect("No file path provided");

    let mut correct: u16 = 0;
    let (countdown_tx, countdown_rx) = mpsc::channel();
    let (input_tx, input_rx) = mpsc::channel::<bool>();

    if let Ok(problems) = read_csv_file(&file) {

        let thread1 = thread::spawn(move || {
            let mut countdown = Countdown::new(QUIZ_TIME);
            countdown.start();
            countdown_tx.send(0).unwrap();
        });

        let thread2 = thread::spawn(move || {
            let mut index: usize = 0;
            while index < problems.len() {

                let p = &problems[index];
                let mut buffer = String::new();

                println!("{:?}", p.question);

                io::stdin().read_line(&mut buffer).unwrap();
                input_tx.send(
                    p.check(&mut buffer)
                ).unwrap();

                index += 1;
                
            }

            let _ = input_tx.send(false);
        });

        loop {

            match input_rx.try_recv() {
                Ok(is_correct) => {
                    if is_correct {
                        correct += 1;
                    }
                },
                _ => ()
            }

            if let Ok(_) = countdown_rx.try_recv() {

                println!("Time's up");
                println!("Your score: {}", correct);

                break;
            }


        }

        process::exit(0);

    }
    else {
        process::exit(1);
    }
}
