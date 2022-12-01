use std::io::{self, BufRead};

fn get_high(high: &mut i32, total: i32) {
    if total > *high {
        *high = total;
    }
}

fn main() -> io::Result<()> {
    let lines = io::stdin().lock().lines();
    let mut high: i32 = 0;
    let mut total: i32 = 0;
    for line in lines {
        match line.unwrap().trim().parse::<i32>() {
            Ok(n) => {
                total += n;
            },
            Err(_e) => {
                get_high(&mut high, total);
                total = 0;
            },
        }
    }
    get_high(&mut high, total);
    println!("{}", high);
    Ok(())
}
