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

fn get_possible(sensor: (i32, i32), beacon: (i32, i32), set: &mut HashSet<(i32, i32)>, limit: i32) {
    let distance = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs() + 1;
    for d in 0..(distance+1) {
        let x = sensor.0 + d;
        let y = sensor.1 + distance - d;
        if x >= 0 && x <= limit && y >= 0 && y <= limit {
            set.insert((x, y));
        }
        let x = sensor.0 - d;
        let y = sensor.1 + distance - d;
        if x >= 0 && x <= limit && y >= 0 && y <= limit {
            set.insert((x, y));
        }
        let x = sensor.0 + d;
        let y = sensor.1 - distance + d;
        if x >= 0 && x <= limit && y >= 0 && y <= limit {
            set.insert((x, y));
        }
        let x = sensor.0 - d;
        let y = sensor.1 - distance + d;
        if x >= 0 && x <= limit && y >= 0 && y <= limit {
            set.insert((x, y));
        }
    }
}

fn count_invalid_spaces_at_y(sensors_n_beacons: Vec<((i32, i32), (i32, i32))>, limit: i32) -> u64 {
    let mut possible = HashSet::new();
    for (sensor, beacon) in &sensors_n_beacons {
        get_possible(*sensor, *beacon, &mut possible, limit);
    }
    for (x, y) in possible {
        let mut possible = true;
        for (sensor, beacon) in &sensors_n_beacons {
            let distance = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs() + 1;
            if (sensor.0 - x).abs() + (sensor.1 - y).abs() < distance {
                possible = false;
            }
        }
        if possible {
            return x as u64 * 4000000 + y as u64;
        }
    }
    return 0;
}

fn main() -> io::Result<()> {
    let lines_in = io::stdin().lock().lines();
    let lines: Vec<String> = lines_in.into_iter().map(|x| x.expect(ERROR_STR)).collect();
    println!("{}", count_invalid_spaces_at_y(parse_sensors_n_beacons(lines), 4000000));
    Ok(())
}
