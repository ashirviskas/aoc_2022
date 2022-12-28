mod day01 {
    pub fn find_max(data: String) -> usize {
        let split = data.lines();
        let mut cur_sum = 0;
        let mut tot_max = vec![0, 0, 0];
        let mut min_idx: usize = 0;
        for s in split {
            if s.eq("") {
                if &cur_sum > tot_max.iter().min().unwrap_or(&0) {
                    tot_max[min_idx] = cur_sum;
                    // Resetting min_idx
                    for (i, v) in tot_max.iter().enumerate() {
                        if v < &cur_sum {
                            min_idx = i;
                            cur_sum = *v;
                        }
                    }
                }
                cur_sum = 0;
            } else {
                cur_sum += s.parse::<usize>().unwrap();
            }
        }
        let max: usize = tot_max.iter().sum();
        print!("Day 1: {}\n", max);
        max
    }
}
mod day02 {
    use std::collections::HashMap;

    pub fn calculate_scores(data: String) -> usize {
        let mut my_shapes: HashMap<String, usize> = HashMap::new();
        my_shapes.insert("X".to_string(), 1);
        my_shapes.insert("Y".to_string(), 2);
        my_shapes.insert("Z".to_string(), 3);

        let split = data.lines();
        let mut score: usize = 0;
        for s in split {
            // A, X - Rock
            // B, Y - Paper
            // C, Z - Scissors
            let (opp_shape, my_shape_w) = s.split_at(1);
            let my_shape = my_shape_w.trim();
            score += my_shapes[my_shape];
            match my_shape {
                "X" => match opp_shape {
                    "A" => {
                        score += 3;
                    }
                    "B" => {
                        score += 0;
                    }
                    "C" => {
                        score += 6;
                    }
                    _ => {}
                },
                "Y" => match opp_shape {
                    "A" => {
                        score += 6;
                    }
                    "B" => {
                        score += 3;
                    }
                    "C" => {
                        score += 0;
                    }
                    _ => {}
                },
                "Z" => match opp_shape {
                    "A" => {
                        score += 0;
                    }
                    "B" => {
                        score += 6;
                    }
                    "C" => {
                        score += 3;
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        print!("Day 2: {}\n", score);
        score
    }

    pub fn calculate_scores_part2(data: String) -> usize {
        let mut my_shapes: HashMap<String, usize> = HashMap::new();
        my_shapes.insert("X".to_string(), 1);
        my_shapes.insert("Y".to_string(), 2);
        my_shapes.insert("Z".to_string(), 3);

        let split = data.lines();
        let mut score: usize = 0;
        for s in split {
            // A, X - Rock
            // B, Y - Paper
            // C, Z - Scissors
            let (opp_shape, my_shape_w) = s.split_at(1);
            let my_shape = my_shape_w.trim();
            // score += my_shapes[my_shape];
            match my_shape {
                "X" => match opp_shape {
                    "A" => {
                        score += 3;
                    }
                    "B" => {
                        score += 1;
                    }
                    "C" => {
                        score += 2;
                    }
                    _ => {}
                },
                "Z" => {
                    score += 6;
                    match opp_shape {
                        "A" => {
                            score += 2;
                        }
                        "B" => {
                            score += 3;
                        }
                        "C" => {
                            score += 1;
                        }
                        _ => {}
                    }
                }
                "Y" => {
                    score += 3;
                    match opp_shape {
                        "A" => {
                            score += 1;
                        }
                        "B" => {
                            score += 2;
                        }
                        "C" => {
                            score += 3;
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        print!("Day 2 p2: {}\n", score);
        score
    }
}

mod day03 {
    // using ascii values
    // a = 1, z = 26
    // A = 27, Z = 52
    use std::collections::HashMap;
    use std::collections::HashSet;

    pub fn get_letter_value(c: char) -> usize {
        let mut val = c as usize;
        if val > 96 {
            val -= 96;
        } else {
            val -= 38;
        }
        val
    }

    pub fn get_splits_matching_character(split_a: &str, split_b: &str) -> char {
        for c in split_a.chars() {
            for c2 in split_b.chars() {
                if c == c2 {
                    return c;
                }
            }
        }
        return ' ';
    }

    // Takes a string and returns a single value
    pub fn get_rucksack_priority(data: String) -> usize {
        let (halve_a, halve_b) = data.split_at(data.len() / 2);
        // Find matching items
        let mut priority = 0;
        let matching_char = get_splits_matching_character(halve_a, halve_b);
        if matching_char != ' ' {
            priority = get_letter_value(matching_char);
        }
        priority
    }

    pub fn get_rucksacks_total_priority(data: String) -> usize {
        let mut total_priority = 0;
        let split = data.lines();
        for s in split {
            total_priority += get_rucksack_priority(s.to_string());
        }
        print!("Day 3: {}\n", total_priority);
        total_priority
    }
    pub fn get_three_rucksacks_badge(data: String) -> char {
        let split = data.lines();
        // Each string to unique chars
        let mut unique_chars_per_string = Vec::new();
        for s in split {
            let mut unique_chars = HashSet::new();
            for c in s.chars() {
                unique_chars.insert(c);
            }
            unique_chars_per_string.push(unique_chars);
        }

        let mut matching_chars = HashMap::new();
        for s in unique_chars_per_string.iter() {
            for c in s.iter() {
                matching_chars
                    .entry(*c)
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }
        }
        let matching_char = matching_chars.iter().find(|&(_, &v)| v == 3).unwrap().0;
        *matching_char
    }

    pub fn get_three_groupings_total_priority(data: String) -> usize {
        let mut total_priority = 0;
        let split = data.lines();
        let mut groupings: Vec<String> = Vec::new();
        for (i, s) in split.enumerate() {
            if i % 3 == 0 {
                groupings.push(s.to_string());
            } else {
                let last = groupings.pop().unwrap();
                let full_str = format!("{}\n{}", last, s);
                // print!("{}\n", full_str);
                groupings.push(full_str);
            }
        }
        for g in groupings {
            total_priority += get_letter_value(get_three_rucksacks_badge(g));
        }
        print!("Day 3 p2: {}\n", total_priority);
        total_priority
    }
}
mod day04 {
    pub fn range_fully_containing(
        range_a_start: usize,
        range_a_end: usize,
        range_b_start: usize,
        range_b_end: usize,
    ) -> bool {
        (range_a_start >= range_b_start && range_a_end <= range_b_end)
            || (range_b_start >= range_a_start && range_b_end <= range_a_end)
    }
    pub fn range_partially_overlapping(
        range_a_start: usize,
        range_a_end: usize,
        range_b_start: usize,
        range_b_end: usize,
    ) -> bool {
        (range_a_start <= range_b_end && range_a_start >= range_b_start)
            || (range_a_end <= range_b_end && range_a_end >= range_b_start)
            || (range_b_start <= range_a_end && range_b_start >= range_a_start)
            || (range_b_end <= range_a_end && range_b_end >= range_a_start)
    }

    pub fn get_range_containing_number(data: String) {
        let split = data.lines();
        let mut fully_contained = 0;
        let mut partially_overlapping = 0;

        for s in split {
            let mut ranges = s.split(",");
            let mut range_a = ranges.next().unwrap().split("-");
            let mut range_b = ranges.next().unwrap().split("-");
            let range_a_start = range_a.next().unwrap().parse::<usize>().unwrap();
            let range_a_end = range_a.next().unwrap().parse::<usize>().unwrap();
            let range_b_start = range_b.next().unwrap().parse::<usize>().unwrap();
            let range_b_end = range_b.next().unwrap().parse::<usize>().unwrap();
            if range_fully_containing(range_a_start, range_a_end, range_b_start, range_b_end) {
                fully_contained += 1;
            }
            if range_partially_overlapping(range_a_start, range_a_end, range_b_start, range_b_end) {
                partially_overlapping += 1;
            }
        }
        print!("Day 4: {}\n", fully_contained);
        print!("Day 4 p2: {}\n", partially_overlapping);
    }
}
mod day05 {
    pub fn reorder_crates(data: String) {
        // crates format:
        //     [D]
        // [N] [C]
        // [Z] [M] [P]
        //  1   2   3
        //
        // move 1 from 2 to 1
        // move 3 from 1 to 3
        // move 2 from 2 to 1
        // move 1 from 1 to 2
        let lines = data.lines();
        let mut crate_stacks: Vec<Vec<char>> = Vec::new();
        let mut crate_stacks_b: Vec<Vec<char>> = Vec::new();

        // Get crates
        let mut crate_part: bool = true;
        let mut crate_stack_size = 0;
        for line in lines {
            if line.contains("1") && crate_part {
                crate_part = false;
                continue;
            }

            if line == "" {
                continue;
            }
            if crate_stack_size == 0 {
                crate_stack_size = (line.len() + 1) / 4;
                for _ in 0..crate_stack_size {
                    crate_stacks.push(Vec::new());
                    crate_stacks_b.push(Vec::new());
                }
            }

            if crate_part {
                for (i, c) in line.chars().enumerate() {
                    if (i + 3) % 4 == 0 {
                        if c == ' ' {
                            continue;
                        } else {
                            crate_stacks[i / 4].insert(0, c);
                            crate_stacks_b[i / 4].insert(0, c);
                        }
                    }
                }
            } else {
                let mut instructions = line.split(" ");
                instructions.next(); // skipping first "move"
                let num = instructions.next().unwrap().parse::<usize>().unwrap();
                instructions.next(); // skipping "from"
                let from = instructions.next().unwrap().parse::<usize>().unwrap();
                instructions.next(); // skipping "to"
                let to = instructions.next().unwrap().parse::<usize>().unwrap();
                for _ in 0..num {
                    let c = crate_stacks[from - 1].pop().unwrap();
                    crate_stacks[to - 1].push(c);
                }
                let crate_stacks_b_from_size = crate_stacks_b[from - 1].len();
                let chars = crate_stacks_b[from - 1]
                    .drain(crate_stacks_b_from_size - num..crate_stacks_b_from_size)
                    .collect::<Vec<char>>();
                crate_stacks_b[to - 1].extend(chars);
            }
        }
        let top_str = crate_stacks
            .iter()
            .map(|v| v.last().unwrap())
            .collect::<String>();
        let top_str_b = crate_stacks_b
            .iter()
            .map(|v| v.last().unwrap())
            .collect::<String>();
        print!("Day 5: {}\n", top_str);
        print!("Day 5 p2: {}\n", top_str_b);
    }
}

mod day06 {
    use std::collections::HashSet;

    pub fn get_first_datastream_char(data: String, unique_len: usize) -> usize {
        let mut stack: Vec<char> = Vec::new();
        let mut res = 0;
        for (i, c) in data.chars().enumerate() {
            stack.push(c);
            if stack.len() > unique_len {
                stack.remove(0);
                let mut uniq = HashSet::new();
                if stack.iter().all(move |x| uniq.insert(x)) {
                    res = i + 1;
                    break;
                }
            }
        }
        res
    }
    pub fn do_both(data: String) {
        let a = get_first_datastream_char(data.clone(), 4);
        let b = get_first_datastream_char(data.clone(), 14);

        print!("Day 6: {}\n", a);
        print!("Day 6 p2: {}\n", b);
    }
}
mod day07 {
    pub fn get_parent_dir(dir_str: String) -> String {
        let parts = dir_str.split("/");
        let mut new_dir = "/".to_string();
        let new_parts_num = parts.clone().count() - 2;
        if new_parts_num == 1 {
            return new_dir;
        } else if new_parts_num == 0 {
            return "".to_string();
        }
        for (i, part) in parts.enumerate() {
            if i == new_parts_num {
                break;
            }
            if part == "" {
                continue;
            }
            new_dir.push_str(part);
            new_dir.push_str("/");
        }
        // print!("new_dir: {}\n", new_dir);
        new_dir
    }
    pub fn calculate_cumulative_dir_sizes(
        paths: &mut Vec<(String, usize, Vec<String>)>,
        limit: usize,
    ) -> usize {
        let mut cumulative_size = 0;
        for path in paths.iter_mut() {
            if path.0 == "/" {
                continue;
            }
            if !path.0.ends_with("/") {
                continue;
            }
            let size = path.1;
            if size > limit {
                continue;
            }
            cumulative_size += size;
        }
        cumulative_size
    }
    pub fn find_smallest_dir_to_delete(
        paths: &mut Vec<(String, usize, Vec<String>)>,
        total_space: usize,
        needed_space: usize,
    ) -> usize {
        let mut smallest_dir_size = 0;
        let current_free_space = total_space - paths[0].1;

        for path in paths.iter() {
            if path.0 == "/" {
                continue;
            }
            if !path.0.ends_with("/") {
                continue;
            }
            let size = path.1;
            if size + current_free_space < needed_space {
                continue;
            }
            if smallest_dir_size == 0 || size < smallest_dir_size {
                smallest_dir_size = size;
            }
        }
        // print!("smallest_dir: {}\nsize: {}\ntotal_space: {}\nneeded_space: {}\nspace after delete: {}\n", smallest_dir, smallest_dir_size, total_space, needed_space, current_free_space + smallest_dir_size);
        smallest_dir_size
    }
    pub fn get_dir_sizes(data: String) {
        let lines = data.lines();
        // each dir has name, a parent, size and children
        let mut paths: Vec<(String, usize, Vec<String>)> = Vec::new();
        let mut cur_dir = String::new();
        let mut prev_dir = String::new();
        for line in lines {
            let mut parts = line.split(" ");
            if line.starts_with('$') {
                parts.next();
                let cur_fun = parts.next().unwrap().to_string();
                match cur_fun.as_str() {
                    "cd" => {
                        prev_dir = cur_dir.clone();
                        let new_dir = parts.next().unwrap();
                        if new_dir == ".." {
                            cur_dir = get_parent_dir(cur_dir);
                        } else {
                            if cur_dir != "/" {
                                if new_dir == "/" {
                                    cur_dir = new_dir.to_string();
                                } else {
                                    cur_dir = cur_dir + &new_dir.to_string() + "/";
                                }
                            } else {
                                cur_dir = cur_dir + &new_dir.to_string() + "/";
                            }
                        }
                        if prev_dir == "" {
                            if new_dir == "/" {
                                paths.push((cur_dir.clone(), 0, Vec::new()));
                            }
                        }
                    }
                    "ls" => {
                        // do nothing
                    }
                    _ => {
                        cur_dir = cur_fun;
                    }
                }
            } else {
                let part_a = parts.next().unwrap();
                if part_a == "dir" {
                    paths.push((
                        cur_dir.clone() + &parts.next().unwrap().to_string().clone() + "/",
                        0,
                        Vec::new(),
                    ));
                } else {
                    let file_size = part_a.parse::<usize>().unwrap();
                    let file_name = parts.next().unwrap().to_string();
                    // print!("{} {} {}\n", cur_dir, file_size, file_name);
                    // print!("paths: {:?}\n", paths);
                    // check if path exists
                    if paths
                        .iter()
                        .find(|x| x.0 == cur_dir.clone() + &file_name)
                        .is_none()
                    {
                        paths.push((cur_dir.clone() + &file_name, file_size, Vec::new()));
                    }
                    let mut dir = paths.iter_mut().find(|x| x.0 == cur_dir).unwrap();
                    dir.1 += file_size;
                    dir.2.push(file_name);
                    let mut dir_parent = get_parent_dir(cur_dir.clone());
                    while dir_parent != "" {
                        let mut dir = paths.iter_mut().find(|x| x.0 == dir_parent).unwrap();
                        dir.1 += file_size;
                        dir_parent = get_parent_dir(dir_parent);
                    }
                }
            }
        }
        // let root = paths.iter().find(|x| x.0 == root_dir).unwrap().1;
        let cumulative_path_size = calculate_cumulative_dir_sizes(&mut paths, 100000);
        let to_delete_dir = find_smallest_dir_to_delete(&mut paths, 70000000, 30000000);

        print!("Day 7: {}\n", cumulative_path_size);
        print!("Day 7 p2: {}\n", to_delete_dir);
    }
}

mod day08 {
    pub fn count_visible_trees(data: String) {
        let lines = data.lines();
        let mut tree_heights: Vec<Vec<usize>> = Vec::new();
        for line in lines {
            let chars = line.chars();
            let mut heights: Vec<usize> = chars.map(|x| x.to_digit(10).unwrap() as usize).collect();
            tree_heights.push(heights);
        }
        let mut visible_trees: Vec<Vec<bool>> = Vec::new();
        let rows_n = tree_heights.len();
        let cols_n = tree_heights[0].len();
        // visible from top
        // visible_trees.push(vec![true; tree_heights[0].len()]);
        let mut cur_max_heights: Vec<i32> = vec![-1; cols_n];
        let mut trees_iter = tree_heights.iter();
        for (i, heights) in trees_iter.enumerate() {
            let mut cur_visible: Vec<bool> = Vec::new();
            for j in 0..heights.len() {
                if cur_max_heights[j] >= heights[j] as i32 {
                    cur_visible.push(false);
                } else {
                    cur_visible.push(true);
                    cur_max_heights[j] = heights[j] as i32;
                }
            }
            visible_trees.push(cur_visible);
        }
        // visible from bottom
        cur_max_heights = vec![-1; cols_n];
        let mut trees_iter = tree_heights.iter().rev();
        for (i, heights) in trees_iter.enumerate() {
            let mut cur_visible: Vec<bool> = Vec::new();
            for j in 0..heights.len() {
                if cur_max_heights[j] >= heights[j] as i32 {
                    cur_visible.push(false);
                } else {
                    // visible_trees[visible_trees.len() - i - 1][j] = true;
                    cur_visible.push(true);
                    cur_max_heights[j] = heights[j] as i32;
                }
            }
            // logical or
            visible_trees[rows_n - i - 1] = visible_trees[rows_n - i - 1]
                .iter()
                .zip(cur_visible.iter())
                .map(|(x, y)| *x || *y)
                .collect();
        }
        for (i, heights) in tree_heights.iter().enumerate() {
            let mut cur_max_height_left: i32 = -1;
            let mut cur_max_height_right: i32 = -1;
            for j in 0..heights.len() {
                if cur_max_height_left >= heights[j] as i32 {
                    visible_trees[i][j] = visible_trees[i][j] || false;
                } else {
                    visible_trees[i][j] = true;
                    cur_max_height_left = heights[j] as i32;
                }

                if cur_max_height_right >= heights[heights.len() - j - 1] as i32 {
                    visible_trees[i][heights.len() - j - 1] =
                        visible_trees[i][heights.len() - j - 1] || false;
                } else {
                    visible_trees[i][heights.len() - j - 1] = true;
                    cur_max_height_right = heights[heights.len() - j - 1] as i32;
                }
            }
        }

        let mut visible_trees_count = visible_trees
            .iter()
            .fold(0, |acc, x| acc + x.iter().filter(|x| **x).count());
        print!("Day 8: {}\n", visible_trees_count);
    }

    pub fn get_most_scenic_tree(data: String) {
        let lines = data.lines().collect::<Vec<_>>();
        let rows_n_soft = lines.len();
        let cols_n_soft = lines[0].len();
        const rows_n: usize = 100;
        const cols_n: usize = 100;
        let mut tree_heights = [[0; cols_n]; rows_n];
        let mut tree_scores = [[0; cols_n]; rows_n];
        for (i, line) in lines.iter().enumerate() {
            let chars = line.chars();
            let mut heights: Vec<usize> = chars.map(|x| x.to_digit(10).unwrap() as usize).collect();
            for (j, height) in heights.iter().enumerate() {
                tree_heights[i][j] = *height;
            }
        }
        // Calculating visible trees for each tree below, above, left and right of it
        for i in 0..rows_n_soft {
            for j in 0..cols_n_soft {
                let mut above = 0;
                let mut below = 0;
                let mut left = 0;
                let mut right = 0;

                if i == 0 || j == 0 || i == rows_n_soft - 1 || j == cols_n_soft - 1 {
                    continue;
                }

                for ii in i + 1..rows_n_soft {
                    if tree_heights[ii][j] < tree_heights[i][j] {
                        below += 1;
                    } else {
                        below += 1;
                        break;
                    }
                }

                for ii in (0..i).rev() {
                    if tree_heights[ii][j] < tree_heights[i][j] {
                        above += 1;
                    } else {
                        above += 1;
                        break;
                    }
                }
                for jj in j + 1..cols_n_soft {
                    if tree_heights[i][jj] < tree_heights[i][j] {
                        right += 1;
                    } else {
                        right += 1;
                        break;
                    }
                }
                for jj in (0..j).rev() {
                    if tree_heights[i][jj] < tree_heights[i][j] {
                        left += 1;
                    } else {
                        left += 1;
                        break;
                    }
                }

                tree_scores[i][j] = above * below * left * right;
            }
        }
        print!(
            "Day 8: {}\n",
            tree_scores
                .iter()
                .fold(0, |acc, x| acc.max(*x.iter().max().unwrap()))
        )
    }
}

mod day09 {
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
}

mod day23 {
    // This part is very ugly, wrote it when I was really tired. Can be improved a lot. Do not take it as an example.
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
}
fn main() {
    day01::find_max(read_input(1));
    day02::calculate_scores(read_input(2));
    day02::calculate_scores_part2(read_input(2));
    day03::get_rucksacks_total_priority(read_input(3));
    day03::get_three_groupings_total_priority(read_input(3));
    day04::get_range_containing_number(read_input(4));
    day05::reorder_crates(read_input(5));
    day06::do_both(read_input(6));
    day07::get_dir_sizes(read_input(7));
    day08::count_visible_trees(read_input(8));
    day08::get_most_scenic_tree(read_input(8));
    day09::calculate_tail_moves(read_input(9));
    day09::calculate_multiple_knots_moves(read_input(9));
    day23::game_of_elves(read_input(23));
}

fn read_input(day: usize) -> String {
    std::fs::read_to_string(format!("./data/day{:0>2}.txt", day)).unwrap()
}
