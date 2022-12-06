use std::io::{self, BufRead};

fn get_priority(c: char) -> u32 {
    return match c.to_digit(36) {
        Some(x) => if c.is_lowercase() { x - 9 }  else { x + 17 } ,
        None => 0
    };
}

fn score_group((first, second, third): (String, String, String)) -> u32 {
    static ALPHABET: &str = "abcdefghijklmnopqrstuvwxtzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    for c in ALPHABET.chars() {
        if !first.contains(c) {
            continue;
        }
        if !second.contains(c) {
            continue;
        }
        if !third.contains(c) {
            continue;
        }
        return get_priority(c);
    }
    return 0;
}

fn split_groups(lines: Vec<String>) -> Vec<(String, String, String)> {
    let mut groups = Vec::new();
    let mut it = lines.iter().peekable();
    while it.peek().is_some() {
        let group = (it.next().unwrap().clone(), it.next().unwrap().clone(), it.next().unwrap().clone());
        groups.push(group);
    }
    return groups;
}

fn score_lines(lines: Vec<String>) -> u32 {
    let mut total = 0;
    for group in split_groups(lines) {
        let score = score_group(group);
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
