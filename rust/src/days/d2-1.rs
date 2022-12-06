use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let lines = io::stdin().lock().lines();
    let mut score = 0;
    for line in lines {
        let line = line?;
        let mut chars = line.chars();
        let a = chars.nth(0);
        let b = chars.nth(1);
        match b.unwrap() {
            'X' => {
                score += 1;
                match a.unwrap() {
                    'A' =>  score += 3,
                    'B' =>  score += 0,
                    'C' =>  score += 6,
                    _ => println!("Invalid character"),
                }
            },
            'Y' => {
                score += 2;
                match a.unwrap() {
                    'A' =>  score += 6,
                    'B' =>  score += 3,
                    'C' =>  score += 0,
                    _ => println!("Invalid character"),
                }
            },
            'Z' => {
                score += 3;
                match a.unwrap() {
                    'A' =>  score += 0,
                    'B' =>  score += 6,
                    'C' =>  score += 3,
                    _ => println!("Invalid character"),
                }
            },
            _ => println!("Invalid character"),
        }
    }
    println!("{}", score);
    Ok(())
}
