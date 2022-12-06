use std::io::{self, BufRead};
use std::collections::HashSet;

fn get_priority(c: char) -> u32 {
    return match c.to_digit(36) {
        Some(x) => if c.is_lowercase() { x - 9 }  else { x + 17 } ,
        None => 0
    };
}

fn score_line(line: String) -> u32 {
    let mut char_present = HashSet::new();
    let length = line.len();
    for (i, c) in line.char_indices() {
        if 2 * i < length {
            char_present.insert(c);
        } else {
            if char_present.contains(&c) {
                return get_priority(c);
            }
        }
    }
    return 0;
}

fn score_lines(lines: Vec<String>) -> u32 {
    let mut total = 0;
    for line in lines {
        let score = score_line(line);
        total += score;
    }
    return total;
}

fn main() -> io::Result<()> {
    let lines_in = io::stdin().lock().lines();
    let lines: Vec<String> = lines_in.into_iter().map(|x| x.expect("Invalid input")).collect();
    println!("{}", score_lines(lines));
    Ok(())
}
