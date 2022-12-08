use std::io::{self, BufRead};
use std::cmp::max;

fn get_best_score(lines: Vec<String>) -> usize {
    let table = lines.into_iter().map(|x| x.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let cols = table[0].len();
    let rows = table.len();
    let mut score = vec![vec![1; cols]; rows];

    for x in 0..rows {
        let mut decreasing = Vec::new();
        decreasing.push(('9', 0));
        for y in 0..cols {
            while !decreasing.is_empty() {
                let (h, _) = decreasing.last().unwrap();
                if h >= &table[x][y] {
                    break;
                }
                decreasing.pop();
            }
            let (_, d) = decreasing.last().unwrap();
            score[x][y] *= y - d;
            decreasing.push((table[x][y], y));
        }
    }
    
    for x in 0..rows {
        let mut decreasing = Vec::new();
        decreasing.push(('9', cols-1));
        for y in (0..cols).rev() {
            while !decreasing.is_empty() {
                let (h, _) = decreasing.last().unwrap();
                if h >= &table[x][y] {
                    break;
                }
                decreasing.pop();
            }
            let (_, d) = decreasing.last().unwrap();
            score[x][y] *= d - y;
            decreasing.push((table[x][y], y));
        }
    }
    
    for y in 0..cols {
        let mut decreasing = Vec::new();
        decreasing.push(('9', 0));
        for x in 0..rows {
            while !decreasing.is_empty() {
                let (h, _) = decreasing.last().unwrap();
                if h >= &table[x][y] {
                    break;
                }
                decreasing.pop();
            }
            let (_, d) = decreasing.last().unwrap();
            score[x][y] *= x - d;
            decreasing.push((table[x][y], x));
        }
    }
    
    for y in 0..cols {
        let mut decreasing = Vec::new();
        decreasing.push(('9', rows-1));
        for x in (0..rows).rev() {
            while !decreasing.is_empty() {
                let (h, _) = decreasing.last().unwrap();
                if h >= &table[x][y] {
                    break;
                }
                decreasing.pop();
            }
            let (_, d) = decreasing.last().unwrap();
            score[x][y] *= d - x;
            decreasing.push((table[x][y], x));
        }
    }

    let mut highest = 0;
    for x in 0..cols {
        for y in 0..rows {
            highest = max(highest, score[x][y]);
        }
    }

    return highest;
}

fn main() -> io::Result<()> {
    let lines_in = io::stdin().lock().lines();
    let lines: Vec<String> = lines_in.into_iter().map(|x| x.expect("Invalid input")).collect();
    let best_score = get_best_score(lines);
    println!("{}", best_score);
    Ok(())
}