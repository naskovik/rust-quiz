mod problem;
mod countdown;

use std::error::Error;
use std::process;
use crate::problem::Problem;
use crate::countdown::Countdown;
use std::thread;


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
    if let Ok(problems) = read_csv_file(&file) {
        thread::spawn(|| {
            let mut countdown = Countdown::new(30);
            countdown.start();
            process::exit(0);
        });
        for p in problems {
            let mut ans = String::new();
            use std::io::stdin;
            println!("{:?}", p.question);
            stdin().read_line(&mut ans).expect("Did not get a correct string");
            if Problem::check(&p, ans) == true {
                correct += 1;
            }
        }
        println!("Your score: {}", correct);
    }
    else {
        process::exit(1);
    }
}
