use std::io::{self, BufRead};
use std::collections::HashSet;

const ERROR_STR: &str = "Invalid Input!";

fn parse_sensors_n_beacons(lines: Vec<String>) -> Vec<((i32, i32), (i32, i32))> {
    return lines.iter().map(|s| {
        let (sensor, beacon) = s.strip_prefix("Sensor at x=").expect(ERROR_STR).split_once(": closest beacon is at x=").expect(ERROR_STR);
        let (sensor_x, sensor_y) = sensor.split_once(", y=").expect(ERROR_STR);
        let (beacon_x, beacon_y) = beacon.split_once(", y=").expect(ERROR_STR);
        ((sensor_x.parse::<i32>().expect(ERROR_STR), sensor_y.parse::<i32>().expect(ERROR_STR)),
        (beacon_x.parse::<i32>().expect(ERROR_STR), beacon_y.parse::<i32>().expect(ERROR_STR)))
    }).collect();
}

fn collect_invalid_spaces_at_y(set: &mut HashSet<i32>, sensor: (i32, i32), beacon: (i32, i32), y: i32) {
    let distance = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();
    if distance < (sensor.1 - y).abs() {
        return;
    }
    for x in (sensor.0 - (distance - (sensor.1 - y).abs()))..(sensor.0 + (distance - (sensor.1 - y).abs()) + 1) {
        set.insert(x);
    }
}

fn count_invalid_spaces_at_y(sensors_n_beacons: Vec<((i32, i32), (i32, i32))>, y: i32) -> usize {
    let mut set = HashSet::new();
    for (sensor, beacon) in &sensors_n_beacons {
        collect_invalid_spaces_at_y(&mut set, *sensor, *beacon, y);
    }
    for (_, (beacon_x, beacon_y)) in &sensors_n_beacons {
        if *beacon_y == y {
            set.remove(&beacon_x);
        }
    }
    return set.len();
}

fn main() -> io::Result<()> {
    let lines_in = io::stdin().lock().lines();
    let lines: Vec<String> = lines_in.into_iter().map(|x| x.expect(ERROR_STR)).collect();
    println!("{}", count_invalid_spaces_at_y(parse_sensors_n_beacons(lines), 2000000));
    Ok(())
}
