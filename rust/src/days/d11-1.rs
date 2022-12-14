use std::io::{self, BufRead};
use std::str::FromStr;

const ERROR_STR: &str = "Invalid Input!";

struct Monkey {
    operator: char,
    factor: Result<i64, <i64 as FromStr>::Err>,
    divisor: i64,
    first_monkey: usize,
    second_monkey: usize,
    items: Vec<i64>,
    count:usize
}

trait MonkeyBehaviour{
    fn catch_item(&mut self, item: i64);
    fn clear(&mut self);
}

trait MonkeyGroup {
    fn throw_items(&mut self);
}

impl MonkeyGroup for Vec<Monkey> {
    fn throw_items(&mut self) {
        for i in 0..self.len() {
            while !self[i].items.is_empty() {
                self[i].count+=1;
                let item = self[i].items.pop().unwrap();
                let worry = match self[i].operator{
                    '*' => match self[i].factor {
                        Err(..) => item * item, 
                        Ok(v) => item * v
                    },
                    '+' => match self[i].factor {
                        Err(..) => item + item,
                        Ok(v) => item + v
                    },
                    _ => 0
                };
                let catcher = if worry/3 % self[i].divisor == 0 {self[i].first_monkey} else {self[i].second_monkey};
                self[catcher].catch_item(worry/3);
            }
        }
    }
}

impl MonkeyBehaviour for Monkey {
    fn catch_item(&mut self, item: i64) {
        self.items.push(item);
    }

    fn clear(&mut self) {
        self.items = Vec::new();
    }
}

fn parse_monkeys<'a>(lines: Vec<String>) -> Vec<Monkey> {
    let mut monkeys = Vec::new();
    let mut it = lines.iter();

    while it.next().is_some() {
        let items: Vec<i64> = it.next().expect(ERROR_STR).strip_prefix("  Starting items: ").expect(ERROR_STR).split(", ").map(|s| s.parse::<i64>().expect(ERROR_STR)).collect();
        let (operator_str, factor_str) = it.next().expect(ERROR_STR).strip_prefix("  Operation: new = old ").expect(ERROR_STR).split_once(" ").expect(ERROR_STR);
        let operator = operator_str.chars().next().unwrap();
        let factor = factor_str.parse::<i64>();

        let divisor = it.next().expect(ERROR_STR).strip_prefix("  Test: divisible by ").expect(ERROR_STR).parse::<i64>().expect(ERROR_STR);
        let first = it.next().expect(ERROR_STR).strip_prefix("    If true: throw to monkey ").expect(ERROR_STR).parse::<usize>().expect(ERROR_STR);
        let second = it.next().expect(ERROR_STR).strip_prefix("    If false: throw to monkey ").expect(ERROR_STR).parse::<usize>().expect(ERROR_STR);
        it.next();

        let monkey = Monkey{operator: operator, factor: factor, divisor: divisor, first_monkey: first, second_monkey: second, count: 0, items: items};
        monkeys.push(monkey);
    }
    return monkeys;
}

fn count_inspects_of_highest_2(mut monkeys: Vec<Monkey>, turns: u32) -> usize {
    for _ in 0..turns{
        monkeys.throw_items();
    }
    let mut cummulate = monkeys.iter().map(|monkey| monkey.count).collect::<Vec<_>>();
    cummulate.sort_by(|a, b| b.cmp(a));
    return cummulate[0] * cummulate[1];
}

fn main() -> io::Result<()> {
    let lines_in = io::stdin().lock().lines();
    let lines: Vec<String> = lines_in.into_iter().map(|x| x.expect(ERROR_STR)).collect();
    println!("{}", count_inspects_of_highest_2(parse_monkeys(lines), 20));
    Ok(())
}
