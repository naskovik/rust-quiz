mod problem;
mod countdown;

use std::error::Error;
use std::process;
use crate::problem::Problem;
use crate::countdown::Countdown;
use std::thread;
use std::sync::mpsc;


fn read_csv_file(path: &str) -> Result<Vec<Problem>, Box<dyn Error>> {

    let mut problems: Vec<Problem> = Vec::new();
    let mut reader = csv::Reader::from_path(path)?;
    for record in reader.records() {
        let result = record?;
        problems.push(Problem {
            question: result[0].to_string(),
            answer: result[1].to_string()
        });
    }
    Ok(problems)
}

fn main() {
    let file =
        std::env::args()
        .nth(1)
        .expect("No file path provided");

    let mut correct: u16 = 0;
    let (tx, rx) = mpsc::channel();

    if let Ok(problems) = read_csv_file(&file) {

        thread::spawn( move || {

            let mut countdown = Countdown::new(30);
            countdown.start();
            tx.send(0).unwrap(); // TODO this doesn't work

        });

        let mut index: usize = 0;

        loop {

            let p = &problems[index];
            let mut ans = String::new();
            use std::io::stdin;
            println!("{:?}", p.question);
            stdin().read_line(&mut ans)
                .expect("Did not get a correct string");

            if Problem::check(&p, ans) == true {
                correct += 1;
            }

            if let Ok(_) = rx.try_recv() {
                println!("Time's up");
                break;
            }

            if index == problems.len() - 1 {
                break;
            }

            index += 1;
        }

        println!("Your score: {}", correct);
        process::exit(0);

    }
    else {
        process::exit(1);
    }
}
