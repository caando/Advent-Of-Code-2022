use std::io::{self, BufRead};
use std::collections::VecDeque;

const ERROR_STR: &str = "Invalid Input!";

fn bfs(map: Vec<Vec<u8>>) -> i32 {
    let mut map = map;
    let height = map.len();
    let width = map[0].len();
    let mut visited = vec![vec![false; width]; height];
    let mut queue = VecDeque::new();
    let mut end = (0, 0);
    for i in 0..height {
        for j in 0..width {
            map[i][j] = match map[i][j] {
                83 => {
                    queue.push_back((i, j, 0));
                    97
                },
                69 => {
                    end = (i, j);
                    122
                }
                x => x
            }
        }
    }

    fn go(x: usize, y: usize, c:i32, v: u8, height: usize, width: usize, map: &Vec<Vec<u8>>, visited: &mut Vec<Vec<bool>>, queue: &mut VecDeque<(usize, usize, i32)>) {
        if x < height && y < width && !visited[x][y] && map[x][y] <= v + 1 {
            queue.push_back((x, y, c+1));
            visited[x][y] = true;
        }
    }
    while !queue.is_empty() {
        let (x, y, c) = queue.pop_front().unwrap();
        if (x, y) == end {
            return c;
        }
        if x != 0 {
            go(x-1, y, c, map[x][y], height, width, &map, &mut visited, &mut queue);
        }
        go(x+1, y, c, map[x][y], height, width, &map, &mut visited, &mut queue);
        if y != 0 {
            go(x, y-1, c, map[x][y], height, width, &map, &mut visited, &mut queue);
        }
        go(x, y+1, c, map[x][y], height, width, &map, &mut visited, &mut queue);
    }
    return 0;
}


fn main() -> io::Result<()> {
    let lines_in = io::stdin().lock().lines();
    let map: Vec<Vec<u8>> = lines_in.into_iter().map(|x| x.expect(ERROR_STR).as_bytes().to_vec()).collect();
    println!("{}", bfs(map));
    Ok(())
}
