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
                score += 0;
                match a.unwrap() {
                    'A' =>  score += 3,
                    'B' =>  score += 1,
                    'C' =>  score += 2,
                    _ => println!("Invalid character"),
                }
            },
            'Y' => {
                score += 3;
                match a.unwrap() {
                    'A' =>  score += 1,
                    'B' =>  score += 2,
                    'C' =>  score += 3,
                    _ => println!("Invalid character"),
                }
            },
            'Z' => {
                score += 6;
                match a.unwrap() {
                    'A' =>  score += 2,
                    'B' =>  score += 3,
                    'C' =>  score += 1,
                    _ => println!("Invalid character"),
                }
            },
            _ => println!("Invalid character"),
        }
    }
    println!("{}", score);
    Ok(())
}
