#[derive(Debug, Clone)]
    pub struct RockLine {
        x: u32,
        y: u32,
        xx: u32,
        yy: u32,
    }
    pub fn display_map(rock_lines: &Vec<RockLine>, add_floor: bool) {
        let mut min_x = rock_lines.iter().flat_map(|rock_line| vec![rock_line.x, rock_line.xx]).min().unwrap();
        let mut max_x = rock_lines.iter().flat_map(|rock_line| vec![rock_line.x, rock_line.xx]).max().unwrap();
        if max_x < 500 {
            max_x = 500;
        }

        let mut min_y = rock_lines.iter().flat_map(|rock_line| vec![rock_line.y, rock_line.yy]).min().unwrap();
        let mut max_y = rock_lines.iter().flat_map(|rock_line| vec![rock_line.y, rock_line.yy]).max().unwrap();
        if min_y > 0 {
            min_y = 0;
        }
        if add_floor {
            max_y += 2;
            min_x = 500 - max_y;
            max_x = 500 + max_y;
        }
        let mut framebuffer = vec![vec!['.'; (max_x - min_x + 1) as usize]; (max_y - min_y + 1) as usize];
        // draw sand source
        framebuffer[0][500 - min_x as usize] = '+';
        if add_floor {
            for x in min_x..=max_x {
                framebuffer[(max_y - min_y) as usize][(x - min_x) as usize] = '#';
            }
        }
        // draw rocks
        for rock_line in rock_lines {
            // for x in rock_line.x..=rock_line.xx {
            //     for y in rock_line.y..=rock_line.yy{
            //         framebuffer[(y - min_y) as usize][(x - min_x) as usize] = '#';
            //     }
            let x = rock_line.x.min(rock_line.xx);
            let xx = rock_line.x.max(rock_line.xx);

            let y = rock_line.y.min(rock_line.yy);
            let yy = rock_line.y.max(rock_line.yy);


            for x in x..=xx {
                for y in y..=yy {
                    framebuffer[(y - min_y) as usize][(x - min_x) as usize] = '#';
                }
            }
        }
        // for line in &framebuffer {
        //     println!("{}", line.iter().collect::<String>());
        // }
        let mut to_break = false;
        let mut sands_fallen = 0;
        loop {
            to_break = true;
            // place sand
            let mut sand_position = (0, 500 - min_x as usize);
            loop {
                if sand_position.0 == framebuffer.len() - 1 || sand_position.1 == 0 || sand_position.1 == framebuffer[0].len() - 1 {
                    to_break = true;
                    break;
                }
                // below
                if framebuffer[sand_position.0 + 1][sand_position.1] == '.' {
                    sand_position.0 += 1;
                    to_break = false;
                    continue;
                }
                // bottom left
                if framebuffer[sand_position.0 + 1][sand_position.1 - 1] == '.' {
                    sand_position.0 += 1;
                    sand_position.1 -= 1;
                    to_break = false;
                    continue;
                }
                // bottom right
                if framebuffer[sand_position.0 + 1][sand_position.1 + 1] == '.' {
                    sand_position.0 += 1;
                    sand_position.1 += 1;
                    to_break = false;
                    continue;
                }
                framebuffer[sand_position.0][sand_position.1] = 'o';
                break;
            }
            sands_fallen += 1;

            if to_break {
                break;
            }
        }
        for line in &framebuffer {
            println!("{}", line.iter().collect::<String>());
        }
        println!("Day 14: {}", sands_fallen - 1 + add_floor as u32);
    }

    pub fn do_sand(data: String) {
        let mut lines = data.split("\n").map(|x| x.to_string()).collect::<Vec<String>>();
        let mut rock_lines: Vec<RockLine> = Vec::new();
        for line in lines {
            let mut prev_x = 0;
            let mut prev_y = 0;
            for (i, segment) in line.split(" -> ").enumerate() {
                // println!("{}: {}", i, segment);
                // println!("{}: {}", i, segment.split(",").nth(0).unwrap());
                // println!("{}: {}", i, segment.split(",").nth(1).unwrap());
                if prev_x == 0 {
                    prev_x = segment.split(",").nth(0).unwrap().parse::<u32>().unwrap();
                    prev_y = segment.split(",").nth(1).unwrap().parse::<u32>().unwrap();
                    continue;
                }
                let x = segment.split(",").nth(0).unwrap().parse::<u32>().unwrap();
                let y = segment.split(",").nth(1).unwrap().parse::<u32>().unwrap();
                rock_lines.push(RockLine{
                    x: prev_x,
                    y: prev_y,
                    xx: x,
                    yy: y,
                });
                prev_x = x;
                prev_y = y;
            }
        }
        display_map(&rock_lines, true);
        // println!("{:?}", rock_lines);
    }