use std::io::{self, BufRead};

fn parse_range(ranges: String) -> (i32, i32) {
    let mut range = ranges.split('-');
    let l = range.next().unwrap().parse::<i32>().unwrap();
    let r = range.next().unwrap().parse::<i32>().unwrap();
    return (l, r);
}

fn parse_ranges(line: String) -> ((i32, i32), (i32, i32)) {
    let mut ranges = line.split(',');
    let first = parse_range(ranges.next().unwrap().to_string());
    let second = parse_range(ranges.next().unwrap().to_string());
    return (first, second);
}

fn parse_lines(lines: Vec<String>) -> Vec<((i32, i32), (i32, i32))> {
    let mut ranges = Vec::new();
    for line in lines{
        ranges.push(parse_ranges(line));
    }
    return ranges;
}

fn count_overlaps(ranges: Vec<((i32, i32), (i32, i32))>) -> i32 {
    let mut count = 0;
    for ((first_l, first_r), (second_l, second_r)) in ranges{
        if first_l <= second_r && second_l <= first_r{
            count += 1;
        } else if second_l <= first_r && first_l <= second_r{
            count += 1;
        }
    }
    return count;
}

fn main() -> io::Result<()> {
    let lines_in = io::stdin().lock().lines();
    let lines: Vec<String> = lines_in.into_iter().map(|x| x.expect("Invalid input")).collect();
    println!("{}", count_overlaps(parse_lines(lines)));
    Ok(())
}
