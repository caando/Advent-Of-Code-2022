use std::io::{self, BufRead};
use std::iter::Peekable;
use std::cmp::Ordering;

const ERROR_STR: &str = "Invalid Input!";

fn compare_num<I: Iterator<Item= char>>(a: &mut Peekable<I>, b: &mut Peekable<I>, x: i32, y: i32) -> i32 {
    match (*a.peek().unwrap(), *b.peek().unwrap()) {
        (c, d) if c.is_digit(10) && d.is_digit(10) => {
            a.next();
            b.next();
            compare_num(a, b, x * 10 + c.to_digit(10).unwrap()as i32, y * 10 + d.to_digit(10).unwrap() as i32)
        },
        (c, _) if c.is_digit(10) => {
            a.next();
            compare_num(a, b, x * 10 + c.to_digit(10).unwrap() as i32, y)
        },
        (_, d) if d.is_digit(10) => {
            b.next();
            compare_num(a, b, x, y * 10 + d.to_digit(10).unwrap() as i32)
        },
        (_, _) => x - y
    }
}

fn compare<I: Iterator<Item= char>>(a: &mut Peekable<I>, b: &mut Peekable<I>, z: i32) -> i32 {
    
    return match (a.peek(), b.peek()) {
        (None, None) => 0,
        (None, Some(_)) => -1,
        (Some(_), None) => 1, 
        (Some(c), Some(d)) => match (*c, *d) {
            (c, d) if c.is_digit(10) && d.is_digit(10) => {
                match compare_num(a, b, 0, 0) {
                0 => compare(a, b, z),
                x => x
            }},
            (c, d) if z > 0 => match c {
                '[' => {
                    a.next();
                    compare(a, b, z+1)
                },
                _ if d.is_digit(10) => -1,
                ']' => {
                    a.next();
                    compare(a, b, z-1)
                },
                _ => 1
            },
            (c, d) if z < 0 => match d {
                '[' => {
                    b.next();
                    compare(a, b, z-1)
                },
                _ if c.is_digit(10) => 1,
                ']' => {
                    b.next();
                    compare(a, b, z+1)
                },
                _ => -1
            },
            (',', ',') | ('[', '[') | (']', ']') => {
                a.next();
                b.next();
                compare(a, b, z)
            },
            ('[', ']') => 1,
            (']', '[') => -1,
            
            (',', ']') => 1,
            (']', ',') => -1,

            ('[', _) => {
                a.next();
                compare(a, b, z+1)
            }, 
            (_, '[') => {
                b.next();
                compare(a, b, z-1)
            }, 

            (_, ']') => 1,
            (']', _) => -1,
            
            (_, _) => 0
        }
    }
}

fn sort_n_find(mut lines: Vec<String>) -> usize {
    lines.push(String::from("[[2]]"));
    lines.push(String::from("[[6]]"));

    lines.sort_by(|a, b| match compare(&mut a.chars().peekable(), &mut b.chars().peekable(), 0) {
        x if x < 0 => Ordering::Less,
        x if x > 0 => Ordering::Greater,
        _ => Ordering::Equal
    });

    let mut result = 1;
    for i in 0..lines.len() {
        if lines[i] == String::from("[[2]]") || lines[i] == String::from("[[6]]") {
            result *= i + 1;
        }
    }
    return result;
}

fn main() -> io::Result<()> {
    let lines_in = io::stdin().lock().lines();
    let lines: Vec<String> = lines_in.into_iter().map(|x| x.expect(ERROR_STR)).filter(|s| !s.is_empty()).collect();
    println!("{}", sort_n_find(lines));
    Ok(())
}
