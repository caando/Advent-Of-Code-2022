use std::io::{self, BufRead};

fn get_high(high1: &mut i32, high2: &mut i32, high3: &mut i32, total: i32) {
    if total > *high1 {
        *high3 = *high2;
        *high2 = *high1;
        *high1 = total;
    } else if total > *high2 {
        *high3 = *high2;
        *high2 = total;
    } else if  total > *high3 {
        *high3 = total;
    }
}

fn main() -> io::Result<()> {
    let lines = io::stdin().lock().lines();
    let mut high1: i32 = 0;
    let mut high2: i32 = 0;
    let mut high3: i32 = 0;
    let mut total: i32 = 0;
    for line in lines {
        match line.unwrap().trim().parse::<i32>() {
            Ok(n) => {
                total += n;
            },
            Err(_e) => {
                get_high(&mut high1, &mut high2, &mut high3, total);
                total = 0;
            },
        }
    }
    get_high(&mut high1, &mut high2, &mut high3, total);
    println!("{}", high1 + high2 + high3);
    Ok(())
}
