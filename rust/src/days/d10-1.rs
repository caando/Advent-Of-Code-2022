use std::io::{self, BufRead};
use std::iter::Peekable;

fn get_score(instructions: Vec<String>, points: Vec<i32>) -> i32 {
    let mut points_it = points.iter().peekable();
    let mut instructions_it = instructions.iter().peekable();
    let mut current = 1;
    let mut steps = 0;
    let mut total = 0;
    fn add_step<'a, I: Iterator<Item=&'a i32>>(steps: &mut i32, current: i32, total: &mut i32, points_it: &mut Peekable<I>) {
        while match points_it.peek() {
            | Some(val) => {
                if *steps+1 == **val {
                    *total += current * (*steps + 1);
                    points_it.next();
                    true
                } else {
                    false
                }
            },
            | None => false
        } {};
        *steps += 1;
    }
    while points_it.peek().is_some() && instructions_it.peek().is_some()  {
        match instructions_it.next() {
            Some (s) => {
                let mut split = s.split(" ");
                match split.next() {
                    | Some("addx") => {
                        let x = split.next().unwrap().parse::<i32>().expect("Invalid input");
                        add_step(&mut steps, current, &mut total, &mut points_it);
                        add_step(&mut steps, current, &mut total, &mut points_it);
                        current += x;
                    },
                    | Some(_) => add_step(&mut steps, current, &mut total, &mut points_it),
                    | None => ()
                }
            },
            None => add_step(&mut steps, current, &mut total, &mut points_it)
        }
    }
    return total;
}

fn main() -> io::Result<()> {
    let lines_in = io::stdin().lock().lines();
    let lines: Vec<String> = lines_in.into_iter().map(|x| x.expect("Invalid input")).collect();
    let points = vec![20, 60, 100, 140, 180, 220];
    println!("{}", get_score(lines, points));
    Ok(())
}
