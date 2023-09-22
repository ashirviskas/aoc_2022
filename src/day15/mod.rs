use itertools::Itertools;

pub fn draw_locations(locations: &Vec<(i32, i32, char)>) -> Vec<Vec<char>> {
    let mut min_x = locations.iter().map(|l| l.0).min().unwrap();
    let mut max_x = locations.iter().map(|l| l.0).max().unwrap();
    let mut min_y = locations.iter().map(|l| l.1).min().unwrap();
    let mut max_y = locations.iter().map(|l| l.1).max().unwrap();

    let mut framebuffer = vec![vec!['.'; (max_x - min_x + 1) as usize]; (max_y - min_y + 1) as usize];
    for location in locations {
        if framebuffer[(location.1 - min_y) as usize][(location.0 - min_x) as usize] != '.' {
            continue;
        }
        framebuffer[(location.1 - min_y) as usize][(location.0 - min_x) as usize] = location.2;
    }
    for line in &framebuffer {
        println!("{}", line.iter().collect::<String>());
    }
    println!("");
    return framebuffer;
}

pub fn check_location_possibility(x: i32, y: i32, locations: &Vec<(i32, i32, i32, i32, i32)>, ignore_existing: bool) -> bool {
    let mut location_possibility = true;
    for location in locations {
        if location.2 == x && location.3 == y && !ignore_existing{
            location_possibility = true;
            break;
        }
        let distance = (x - location.0).abs() + (y - location.1).abs();
        if distance <= location.4 {
            location_possibility = false;
            break;
        }
    }
    location_possibility
}

pub fn get_line_intervals(locations: &Vec<(i32, i32, i32, i32, i32)>, line: i32, x_limit: (i32, i32)) -> Vec<(i32, i32)> {
    let mut intervals = Vec::new();
    for location in locations {
        // Check bounds
        if !(location.1 - location.4 < line && location.1 + location.4 > line) {
            continue;
        }
        let mut first_intersection = location.0 - (location.4 - (line - location.1).abs());
        let mut second_intersection = location.0 + (location.4 - (line - location.1).abs());
        if first_intersection < x_limit.0 {
            first_intersection = x_limit.0;
        }
        if second_intersection > x_limit.1 {
            second_intersection = x_limit.1;
        }
        let interval = (first_intersection, second_intersection);
        intervals.push(interval);
    }
    intervals
}

pub fn merge_intervals(sorted_intervals: &Vec<(i32, i32)>)  -> Vec<(i32, i32)> {
    let mut merged_intervals = Vec::new();
    let mut current_interval = sorted_intervals[0];
    for interval in sorted_intervals {
        if interval.0 - 1 > current_interval.1 {
            merged_intervals.push(current_interval);
            current_interval = *interval;
        } else {
            current_interval.1 = (*interval).1.max(current_interval.1);
        }
    }
    merged_intervals.push(current_interval);
    merged_intervals
}


pub fn beacon_the_sensors(data: String, line_to_check: i32, boundary: i32) {
    let mut sensors_beacons = data.split("\n")
        .flat_map(|s| s.split("="))
        .map(|s| s
            .chars()
            .filter(|c| c.is_digit(10) || c == &'-')
            .collect::<String>())
        .filter(|s| s.len() > 0)
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
        .chunks(4).map(|s| (s[0], s[1], s[2], s[3]))
        .collect::<Vec<(i32, i32, i32, i32)>>();

    let min_x = sensors_beacons.iter().flat_map(|s| vec![s.0, s.2]).min().unwrap();
    let max_x = sensors_beacons.iter().flat_map(|s| vec![s.0, s.2]).max().unwrap();
    let min_y = sensors_beacons.iter().flat_map(|s| vec![s.1, s.3]).min().unwrap();
    let max_y = sensors_beacons.iter().flat_map(|s| vec![s.1, s.3]).max().unwrap();
    // println!("min_x: {}, max_x: {}, min_y: {}, max_y: {}", min_x, max_x, min_y, max_y);
    let sensors_beacons_distances = sensors_beacons.iter().map(|s| (s.0, s.1, s.2, s.3, (s.0 - s.2).abs() + (s.1 - s.3).abs())).collect::<Vec<(i32, i32, i32, i32, i32)>>();
    let max_distance = sensors_beacons_distances.iter().map(|s| s.4).max().unwrap();
    println!("max_distance: {}", max_distance);
    let mut line_impossible_locations = 0;
    for x in (min_x - max_distance * 2)..(max_x + max_distance * 2) {
        // println!("x: {}, line_impossible_locations: {}", x, line_impossible_locations);
        if !check_location_possibility(x, line_to_check, &sensors_beacons_distances, false) {
            line_impossible_locations += 1;
        }
    }

    // 'outer: for x in 0..boundary {
    //     for y in 0..boundary {
    //         if check_location_possibility(x, y, &sensors_beacons_distances, true) {
    //             println!("Found! x: {}, y: {}", x, y);
    //             let frequency = x * 4000000 + y;
    //             println!("Frequency: {}", frequency);
    //             break 'outer;
    //         }
    //     }
    // }
    // Intervals
    for y in (min_y - max_distance * 2)..(max_y + max_distance * 2) {
        if y < 0 {
            continue;
        }
        if y > boundary {
            break;
        }
        let mut intervals = get_line_intervals(&sensors_beacons_distances, y, (0, boundary));
        // println!("y: {}, intervals: {:?}", y, intervals);
        intervals.sort_by(|a, b| a.0.cmp(&b.0));
        // println!("y: {}, sorted intervals: {:?}", y, intervals);
        let merged_intervals = merge_intervals(&intervals);
        // println!("y: {}, merged intervals: {:?}", y, merged_intervals);
        if merged_intervals.len() == 1 {
            continue;
        } else {
            println!("Found! x: {}, y: {}", merged_intervals[0].1 + 1, y);
            let frequency = (merged_intervals[0].1 + 1) as i64 * 4000000 + y as i64;
            println!("Frequency: {}", frequency);
            break;
        }


    }

    println!("Day 15: {}", line_impossible_locations);
}