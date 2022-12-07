use std::io::{self, BufRead};
use std::cmp::min;

fn dfs<'a, I>(iterator: &mut I, collector: &mut Vec<u32>) -> u32 
where I: Iterator<Item = &'a String> {
    let mut total = 0;
    while let Some(s) = iterator.next() {
        let mut args = s.split(" ");
        match args.next() {
            Some("$") => {
                match args.next() {
                    Some("cd") => {
                        match args.next() {
                            Some("..") => {
                                break;
                            }, 
                            Some(_) => {
                                total += dfs(iterator, collector);
                            }, 
                            None => ()
                        }
                    }, 
                    Some(..) => (),
                    None => ()
                }
            }, 
            Some("dir") => (), 
            Some(file) => {
                total += file.parse::<u32>().unwrap();
            }, 
            None => ()
        }
    }
    collector.push(total);
    return total;
}

fn get_min(lines: Vec<String>, limit: u32, required: u32) -> u32 {
    let mut sizes = Vec::new();
    let cur = &mut lines.iter();
    cur.next();
    let occupied = dfs(cur, &mut sizes);
    let mut best = occupied;
    for dir in sizes{
        if limit + dir >= occupied + required {
            best = min(best, dir);
        }
    }
    return best;
}

fn main() -> io::Result<()> {
    let lines_in = io::stdin().lock().lines();
    let lines: Vec<String> = lines_in.into_iter().map(|x| x.expect("Invalid input")).collect();
    let sum = get_min(lines, 70000000, 30000000);
    println!("{}", sum);
    Ok(())
}