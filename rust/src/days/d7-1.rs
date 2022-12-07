use std::io::{self, BufRead};

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

fn get_sum(lines: Vec<String>, limit: u32) -> u32 {
    let mut sizes = Vec::new();
    let cur = &mut lines.iter();
    cur.next();
    dfs(cur, &mut sizes);
    let mut total = 0;
    for dir in sizes{
        if dir <= limit {
            total += dir;
        }
    }
    return total;
}

fn main() -> io::Result<()> {
    let lines_in = io::stdin().lock().lines();
    let lines: Vec<String> = lines_in.into_iter().map(|x| x.expect("Invalid input")).collect();
    let sum = get_sum(lines, 100000);
    println!("{}", sum);
    Ok(())
}