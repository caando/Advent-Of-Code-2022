use std::io::{self, BufRead};
use std::collections::HashMap;

fn get_unique(line: String, k: usize) -> usize {
    let mut char_cnt = HashMap::new();
    let chars = line.as_bytes();
    for i in 0..line.len() {
        let c = chars[i] as char;
        match char_cnt.get_mut(&c) {
            Some(v) => *v += 1,
            None => {
                char_cnt.insert(c, 1);
            }
        }
        if i >= k {
            let d = chars[i-k] as char;
            match char_cnt.get_mut(&d) {
                Some(1) => {char_cnt.remove(&d);},
                Some(v) => *v -= 1,
                None => ()
            }
        }
        if char_cnt.len() == k {
            return i + 1;
        }
    }
    return 0;

}

fn main() -> io::Result<()> {
    let lines_in = io::stdin().lock().lines();
    let lines: Vec<String> = lines_in.into_iter().map(|x| x.expect("Invalid input")).collect();
    for line in lines{
        println!("{}", get_unique(line, 14));
    }
    Ok(())
}

