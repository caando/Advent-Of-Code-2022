use std::io::{self, BufRead};
use std::collections::LinkedList;

fn process(mut crates: Vec<LinkedList<char>>, moves: Vec<(usize, usize, usize)>) -> Vec<LinkedList<char>> {
    for (count, a, b) in moves {
        let mut buffer = LinkedList::new();
        for _ in 0..count {
            match crates[a].pop_back() {
                Some(x) => buffer.push_front(x),
                None => ()
            }
        }
        for c in buffer {
            crates[b].push_back(c);
        }
    }
    return crates;
}

fn top_char(crates: Vec<LinkedList<char>>) -> String{
    let mut s = String::new();
    for list in crates {
        s.push(match list.back() {
            None => ' ',
            Some(x) => *x
        });
    }
    return s;
}

fn parse_input(lines: Vec<String>) -> (Vec<LinkedList<char>>, Vec<(usize, usize, usize)>) {
    let mut crates = Vec::new();
    let mut moves = Vec::new();
    let mut it = lines.iter().peekable();
    while it.peek().is_some() && it.peek().unwrap().len() > 0 {
        let line = it.next().unwrap();
        if crates.len() == 0 {
            let cols = (line.len() + 1) / 4;
            for _i in 0..cols {
                crates.push(LinkedList::new());
            }
        }
        let mut c = line.chars();
        for col in &mut crates {
            match c.nth(1) {
                Some(' ') => (),
                None => (),
                Some(x) => col.push_front(x)
            }
            c.nth(1);
        }
    }
    it.next();
    while it.peek().is_some() {
        let line = it.next().unwrap();
        let line = line.strip_prefix("move ").unwrap();
        let (a, b) = line.rsplit_once(" from ").unwrap();
        let (c, d) = b.rsplit_once(" to ").unwrap();
        moves.push((a.parse::<usize>().unwrap(), c.parse::<usize>().unwrap() - 1, d.parse::<usize>().unwrap() - 1));
    }
    return (crates, moves);
}

fn main() -> io::Result<()> {
    let lines_in = io::stdin().lock().lines();
    let lines: Vec<String> = lines_in.into_iter().map(|x| x.expect("Invalid input")).collect();
    let (crates, moves) = parse_input(lines);
    println!("{}", top_char(process(crates, moves)));
    Ok(())
}
