use std::io::{self, BufRead};
use std::iter::Peekable;

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

fn sum_indexes(lines: Vec<String>) -> usize {
    let mut total = 0;
    let mut it = lines.iter().peekable();
    let mut index = 1;
    while it.peek().is_some() {
        let mut a = it.next().expect(ERROR_STR).chars().peekable();
        let mut b = it.next().expect(ERROR_STR).chars().peekable();
        it.next();
        if compare(&mut a, &mut b, 0) < 0 {
            total += index;
        }
        index += 1;
    }
    return total;
}

fn main() -> io::Result<()> {
    let lines_in = io::stdin().lock().lines();
    let lines: Vec<String> = lines_in.into_iter().map(|x| x.expect(ERROR_STR)).collect();
    println!("{}", sum_indexes(lines));
    Ok(())
}
