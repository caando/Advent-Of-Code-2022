use std::io::{self, BufRead};
use std::collections::HashSet;

const ERROR_STR: &str = "Invalid Input!";

fn parse_walls(lines: Vec<String>) -> Vec<Vec<(i32, i32)>> {
    return lines.iter().map(|line| line.split(" -> ").map(|x| {
        let mut y = x.split(",");
        (y.next().expect(ERROR_STR).parse::<i32>().expect(ERROR_STR), y.next().expect(ERROR_STR).parse::<i32>().expect(ERROR_STR))
    }).collect()).collect();
}

fn plot_wall(wall: Vec<(i32, i32)>, occupied: &mut HashSet<(i32, i32)>) {
    let mut it = wall.iter().peekable();
    let mut cur = *it.next().expect(ERROR_STR);
    occupied.insert(cur);
    while it.peek().is_some() {
        let nx = it.next().unwrap();
        while cur != *nx {
            if cur.0 == nx.0 {
                cur.1 += (nx.1 - cur.1) / (nx.1 - cur.1).abs();
            } else {
                cur.0 += (nx.0 - cur.0) / (nx.0 - cur.0).abs();
            }
            occupied.insert(cur);
        }
    }
}

fn drop_sand(x: i32, y: i32, occupied: &mut HashSet<(i32, i32)>, floor: i32) -> bool {
    if y > floor {
        return true;
    } else if !occupied.contains(&(x, y+1)) {
        return drop_sand(x, y+1, occupied, floor);
    } else if !occupied.contains(&(x-1, y+1)) {
        return drop_sand(x-1, y+1, occupied, floor);
    } else if !occupied.contains(&(x+1, y+1)) {
        return drop_sand(x+1, y+1, occupied, floor);
    } else {
        occupied.insert((x, y));
        return false;
    }
}

fn get_max_sand(walls: Vec<Vec<(i32, i32)>>, floor : i32) -> usize {
    let mut total = 0;
    let mut occupied: HashSet<(i32, i32)> = HashSet::new();
    for wall in walls {
        plot_wall(wall, &mut occupied);
    }
    while !drop_sand(500, 0, &mut occupied, floor) {
        total += 1;
    }
    return total;
}

fn main() -> io::Result<()> {
    let lines_in = io::stdin().lock().lines();
    let lines: Vec<String> = lines_in.into_iter().map(|x| x.expect(ERROR_STR)).collect();
    println!("{}", get_max_sand(parse_walls(lines), 500));
    Ok(())
}
