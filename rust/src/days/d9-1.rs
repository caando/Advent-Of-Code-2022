use std::io::{self, BufRead};
use std::collections::HashSet;

fn update_tail_coords((hx, hy): (i32, i32), (tx, ty): (i32, i32)) -> (i32, i32) {
    if hx == tx && hy == ty {
        return (hx, hy);
    } else if (hx-tx).abs() == (hy - ty).abs() {
        return (hx - (hx - tx) / (hx - tx).abs(), hy - (hy - ty) / (hy - ty).abs());
    } else if (hx-tx).abs() > (hy - ty).abs() {
        return (hx - (hx - tx) / (hx - tx).abs(), hy);
    } else {
        return (hx, hy - (hy - ty) / (hy - ty).abs());
    }
    
}

fn count_tail_positions(moves: Vec<(char, i32)>) -> usize {
    let mut coords: HashSet<(i32, i32)> = HashSet::new();
    let mut hx = 0;
    let mut hy = 0;
    let mut tx = 0;
    let mut ty = 0;
    coords.insert((tx, ty));
    for (c, d) in moves {
        for _ in 0..d {
            match c {
                | 'R' =>  hx += 1,
                | 'U' =>  hy += 1,
                | 'L' =>  hx -= 1,
                | 'D' =>  hy -= 1, 
                | _ => ()
            }
            (tx, ty) = update_tail_coords((hx, hy), (tx, ty));
            coords.insert((tx, ty));
        }
    }
    return coords.len();
}

fn parse_lines(lines: Vec<String>) -> Vec<(char, i32)> {
    return lines.into_iter().map(|x| {
        let mut s = x.split(" "); 
        (s.next().unwrap().chars().next().unwrap(), s.next().unwrap().parse::<i32>().expect("Invalid line"))
    }).collect::<Vec<(char, i32)>>();

}

fn main() -> io::Result<()> {
    let lines_in = io::stdin().lock().lines();
    let lines: Vec<String> = lines_in.into_iter().map(|x| x.expect("Invalid input")).collect();
    
    let tail_positions = count_tail_positions(parse_lines(lines));

    println!("{}", tail_positions);
    Ok(())
}