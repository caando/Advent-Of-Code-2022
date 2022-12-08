use std::io::{self, BufRead};

fn get_visible(lines: Vec<String>) -> u32 {
    let table = lines.into_iter().map(|x| x.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let cols = table[0].len();
    let rows = table.len();
    let mut visible = vec![vec![false; cols]; rows];
    for x in 0..rows {
        let mut high = '/';
        for y in 0..cols {
            if table[x][y] > high {
                visible[x][y] = true;
                high = table[x][y];
            }
        }
    }
    for x in 0..rows {
        let mut high = '/';
        for y in (0..cols).rev() {
            if table[x][y] > high {
                visible[x][y] = true;
                high = table[x][y];
            }
        }
    }

    for y in 0..cols {
        let mut high = '/';
        for x in 0..rows {
            if table[x][y] > high {
                visible[x][y] = true;
                high = table[x][y];
            }
        }
    }

    for y in 0..cols {
        let mut high = '/';
        for x in (0..rows).rev() {
            if table[x][y] > high {
                visible[x][y] = true;
                high = table[x][y];
            }
        }
    }
    let mut total = 0;
    for x in 0..cols {
        for y in 0..rows {
            if visible[x][y] {
                total += 1;
            }
        }
    }

    return total;
}

fn main() -> io::Result<()> {
    let lines_in = io::stdin().lock().lines();
    let lines: Vec<String> = lines_in.into_iter().map(|x| x.expect("Invalid input")).collect();
    let total_visible = get_visible(lines);
    println!("{}", total_visible);
    Ok(())
}