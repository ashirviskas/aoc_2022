use std::collections::HashMap;

    pub fn move_next_node(
        head_pos_x: i32,
        head_pos_y: i32,
        tail_pos_x: i32,
        tail_pos_y: i32,
    ) -> (i32, i32) {
        // x - 5
        // y - 2
        // xx - 4
        // yy - 0
        let distance_x = head_pos_x - tail_pos_x;
        let distance_y = head_pos_y - tail_pos_y;
        let mut new_tail_pos_x = tail_pos_x;
        let mut new_tail_pos_y = tail_pos_y;

        if distance_x.abs() == 2 && distance_y.abs() == 2 {
            new_tail_pos_x = tail_pos_x + distance_x / 2;
            new_tail_pos_y = tail_pos_y + distance_y / 2;
            return (new_tail_pos_x, new_tail_pos_y);
        }
        // println!("{} {}", distance_x, distance_y);
        if distance_x.abs() > 1 {
            new_tail_pos_x = tail_pos_x + (distance_x.signum());
            new_tail_pos_y = head_pos_y;
        }
        if distance_y.abs() > 1 {
            new_tail_pos_y = tail_pos_y + (distance_y.signum());
            new_tail_pos_x = head_pos_x;
        }
        (new_tail_pos_x, new_tail_pos_y)
    }
    pub fn calculate_tail_moves(data: String) {
        let lines = data.lines();
        let mut head_pos_x: i32 = 0;
        let mut head_pos_y: i32 = 0;
        let mut tail_pos_x: i32 = 0;
        let mut tail_pos_y: i32 = 0;
        let mut tail_visited: HashMap<(i32, i32), i32> = HashMap::new();

        for line in lines {
            let (direction, distance) = line.split_at(1);
            let distance = distance.trim().parse::<i32>().unwrap();
            for i in 0..distance {
                match direction {
                    "U" => {
                        head_pos_y += 1;
                    }
                    "D" => {
                        head_pos_y -= 1;
                    }
                    "L" => {
                        head_pos_x -= 1;
                    }
                    "R" => {
                        head_pos_x += 1;
                    }
                    _ => panic!("Unknown direction"),
                }
                let (new_tail_pos_x, new_tail_pos_y) =
                    move_next_node(head_pos_x, head_pos_y, tail_pos_x, tail_pos_y);
                if new_tail_pos_x != tail_pos_x || new_tail_pos_y != tail_pos_y {
                    tail_pos_x = new_tail_pos_x;
                    tail_pos_y = new_tail_pos_y;
                    tail_visited.insert((tail_pos_x, tail_pos_y), 1);
                }
            }
        }
        print!("Day 9: {}\n", tail_visited.len() + 1);
    }

    pub fn calculate_multiple_knots_moves(data: String) {
        let lines = data.lines();
        const n_knots: usize = 10;
        let mut nodes_positions: [(i32, i32); n_knots] = [(0, 0); n_knots];
        let mut tail_nodes_visited: HashMap<(i32, i32), i32> = HashMap::new();
        for line in lines {
            let (direction, distance) = line.split_at(1);
            let distance = distance.trim().parse::<i32>().unwrap();
            for i in 0..distance {
                match direction {
                    "U" => {
                        nodes_positions[0].1 += 1;
                    }
                    "D" => {
                        nodes_positions[0].1 -= 1;
                    }
                    "L" => {
                        nodes_positions[0].0 -= 1;
                    }
                    "R" => {
                        nodes_positions[0].0 += 1;
                    }
                    _ => panic!("Unknown direction"),
                }
                for j in 1..n_knots {
                    let (new_tail_pos_x, new_tail_pos_y) = move_next_node(
                        nodes_positions[j - 1].0,
                        nodes_positions[j - 1].1,

                        nodes_positions[j].0,
                        nodes_positions[j].1,
                    );
                    if new_tail_pos_x == nodes_positions[j].0 && new_tail_pos_y == nodes_positions[j].1
                    {
                        break;
                    }
                    
                    nodes_positions[j].0 = new_tail_pos_x;
                    nodes_positions[j].1 = new_tail_pos_y;
                    if j <= n_knots - 2 {
                        continue
                    }
                    tail_nodes_visited.insert((nodes_positions[j].0, nodes_positions[j].1), 1);
                }
            }
        }
        print!("Day 9: {}\n", tail_nodes_visited.len() + 1);
    }