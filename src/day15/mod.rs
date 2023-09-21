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

pub fn beacon_the_sensors(data: String) {
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

    let mut min_x = sensors_beacons.iter().flat_map(|s| vec![s.0, s.2]).min().unwrap();
    let mut max_x = sensors_beacons.iter().flat_map(|s| vec![s.0, s.2]).max().unwrap();

    let mut min_y = sensors_beacons.iter().flat_map(|s| vec![s.1, s.3]).min().unwrap();
    let mut max_y = sensors_beacons.iter().flat_map(|s| vec![s.1, s.3]).max().unwrap();
    println!("min_x: {}, max_x: {}, min_y: {}, max_y: {}", min_x, max_x, min_y, max_y);
    // draw sensors and beacons
    // let mut framebuffer = vec![vec!['.'; (max_x - min_x + 1) as usize]; (max_y - min_y + 1) as usize];
    let mut locations: Vec<(i32, i32, char)> = Vec::new();
    for sensor_beacon in &sensors_beacons {
        // framebuffer[(sensor_beacon.1 - min_y) as usize][(sensor_beacon.0 - min_x) as usize] = 'S';
        // framebuffer[(sensor_beacon.3 - min_y) as usize][(sensor_beacon.2 - min_x) as usize] = 'B';
        locations.push((sensor_beacon.0, sensor_beacon.1, 'S'));
        locations.push((sensor_beacon.2, sensor_beacon.3, 'B'));
    }
    draw_locations(&locations);
    // for each pair, mark locations with no possible beacons
    for sensor_beacon in &sensors_beacons {
        let x = sensor_beacon.0;
        let y = sensor_beacon.1;
        let xx = sensor_beacon.2;
        let yy = sensor_beacon.3;
        let distance = (xx - x).abs() + (yy - y).abs();
        for i in -distance..=distance {
            for j in -distance..=distance {
                let new_x = x + i;
                let new_y = y + j;
                if i.abs() + j.abs() > distance {
                    continue;
                }
                locations.push((new_x, new_y, '#'));
            }
        }
        draw_locations(&locations);
    }
    let framebuffer = draw_locations(&locations);
    // counting locations with no possible beacons in line 10
    let no_beacon_count = locations.iter()
    .filter(|l| l.1 == 10)
    .filter(|l| l.2 != '.')
    .unique_by(|l| vec![l.0, l.1])
    .count() - 1;
    println!("");
    draw_locations(&locations);

    println!("Day 15: {}", no_beacon_count);
}