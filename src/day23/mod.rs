const GRID_SIZE : usize = 200;

    pub fn construct_grid_from_positions(elve_positions: &Vec<(usize, usize)>) -> [[char; GRID_SIZE]; GRID_SIZE] {
        let mut grid: [[char; GRID_SIZE]; GRID_SIZE] = [['.'; GRID_SIZE]; GRID_SIZE];
        for elve_position in elve_positions {
            let (x, y) = elve_position;
            grid[*x][*y] = '#';
        }
        grid
    }

    pub fn check_direction(grid: &[[char; GRID_SIZE]; GRID_SIZE], x: usize, y: usize, direction: char) -> bool {
        match direction {
            'N' => {
                grid[x-1][y-1] != '#' && grid[x-1][y] != '#' && grid[x-1][y+1] != '#'
            }
            'S' => {
                grid[x+1][y-1] != '#' && grid[x+1][y] != '#' && grid[x+1][y+1] != '#'
            }
            'W' => {
                grid[x-1][y-1] != '#' && grid[x][y-1] != '#' && grid[x+1][y-1] != '#'
            }
            'E' => {
                grid[x-1][y+1] != '#' && grid[x][y+1] != '#' && grid[x+1][y+1] != '#'
            }
            _ => panic!("Unknown direction"),
        }
    }

    pub fn do_round(elve_positions: &mut Vec<(usize, usize)>, grid: &mut [[char; GRID_SIZE]; GRID_SIZE], directions: &Vec<char>) -> [[char; GRID_SIZE]; GRID_SIZE] {
        let mut next_positions: Vec<(usize, usize)> = elve_positions.clone();

        let mut next_grid: [[i32; GRID_SIZE]; GRID_SIZE] = [[0; GRID_SIZE]; GRID_SIZE];

        for (i, elve_position) in elve_positions.iter().enumerate() {
            let (x, y) = elve_position;
            // checking 8 directions
            let mut found_neighbour = false;
            for xx in -1..2 {
                for yy in -1..2 {
                    if xx == 0 && yy == 0 {
                        continue;
                    }
                    if grid[(*x as i32 + xx) as usize][(*y as i32 + yy) as usize] == '#' {
                        found_neighbour = true;
                        break;
                    }
                }
            }
            if !found_neighbour {
                continue;
            }

            for direction in directions {
                if check_direction(grid, *x, *y, *direction) {
                    match direction {
                        'N' => {
                            next_positions[i].0 -= 1;
                            next_grid[*x-1][*y] += 1;
                            break;
                        }
                        'S' => {
                            next_positions[i].0 += 1;
                            next_grid[*x+1][*y] += 1;
                            break;
                        }
                        'W' => {
                            next_positions[i].1 -= 1;
                            next_grid[*x][*y-1] += 1;
                            break;
                        }
                        'E' => {
                            next_positions[i].1 += 1;
                            next_grid[*x][*y+1] += 1;
                            break;
                        }
                        _ => panic!("Unknown direction"),
                    }
                }
            }
            
        }
        for i in 0..elve_positions.len() {
            if next_grid[next_positions[i].0][next_positions[i].1] > 1 {
                next_positions[i] = elve_positions[i];
            }
        }
        for i in 0..next_positions.len() {
            elve_positions[i] = next_positions[i];
        }
        let new_grid: [[char; GRID_SIZE]; GRID_SIZE] = construct_grid_from_positions(&next_positions);
        new_grid      



    }


    pub fn get_grid_bbox(elve_positions: &Vec<(usize, usize)>) -> (usize, usize, usize, usize) {
        let mut min_x = GRID_SIZE;
        let mut max_x = 0;
        let mut min_y = GRID_SIZE;
        let mut max_y = 0;
        for (x, y) in elve_positions {
            if *x < min_x {
                min_x = *x;
            }
            if *x > max_x {
                max_x = *x;
            }
            if *y < min_y {
                min_y = *y;
            }
            if *y > max_y {
                max_y = *y;
            }
        }
        (min_x, max_x, min_y, max_y)
    }

    pub fn count_empty_spaces_in_bbox(elve_positions: &Vec<(usize, usize)>, min_x: usize, max_x: usize, min_y: usize, max_y: usize) -> usize {
        let count = (max_x - min_x + 1) * (max_y - min_y + 1) - elve_positions.len();
        count
    }

    pub fn center_positions(elve_positions: &Vec<(usize, usize)>, grid: &mut [[char; GRID_SIZE]; GRID_SIZE]) -> Vec<(usize, usize)> {
        let (min_x, max_x, min_y, max_y) = get_grid_bbox(&elve_positions);
        let mut new_positions: Vec<(usize, usize)> = Vec::new();
        let mut new_grid: [[char; GRID_SIZE]; GRID_SIZE] = [['.'; GRID_SIZE]; GRID_SIZE];
        let offset_x:i32 = ((GRID_SIZE - max_x + min_x) / 2) as i32 - min_x as i32;
        let offset_y:i32 = ((GRID_SIZE - max_y + min_y) / 2) as i32 - min_y as i32;
        for (x, y) in elve_positions {
            let new_x = (*x  as i32 + offset_x) as usize;
            let new_y = (*y  as i32 + offset_y) as usize;
            new_positions.push((new_x, new_y));
            new_grid[new_x][new_y] = '#';
        }
        *grid = new_grid;
        new_positions
    }

    pub fn game_of_elves(data: String) {
        let n_rounds: usize = 10;
        let mut grid: [[char; GRID_SIZE]; GRID_SIZE] = [['.'; GRID_SIZE]; GRID_SIZE];
        let mut elve_positions: Vec<(usize, usize)> = Vec::new();
        // stack of 4 directions
        let mut directions: Vec<char> = Vec::new();
        directions.push('N');
        directions.push('S');
        directions.push('W');
        directions.push('E');


        let lines = data.lines();
        for (i, line) in lines.enumerate() {
            for (j, c) in line.chars().enumerate() {
                if c == '#' {
                    elve_positions.push((i, j));
                    grid[i][j] = '#';
                }
            }
        }

        let mut round_count = 0;
        loop {
            let grid_copy = grid.clone();
            elve_positions = center_positions(&elve_positions, &mut grid);
            grid = do_round(&mut elve_positions, &mut grid, &directions);
            let first_direction = directions.remove(0);
            directions.push(first_direction);
            if grid == grid_copy { //change to checking round count for p1
                break;
            }
            round_count += 1;
        }
        let (min_x, max_x, min_y, max_y) = get_grid_bbox(&elve_positions);
        let empty_spaces = count_empty_spaces_in_bbox(&elve_positions, min_x, max_x, min_y, max_y);
        print!("Day 23: {}\n", empty_spaces);
        print!("Day 23 p2: {}\n", round_count + 1);
    }