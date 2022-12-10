
mod day01 {
    pub fn find_max(data: String) -> usize {
        let mut split = data.lines();
        let mut cur_sum = 0;
        let mut tot_max = vec![0, 0, 0];
        let mut min_idx: usize = 0;
        for s in split {
            if s.eq(""){
                if &cur_sum > tot_max.iter().min().unwrap_or(&0) {
                    tot_max[min_idx] = cur_sum;
                    // Resetting min_idx
                    for (i, v) in tot_max.iter().enumerate(){
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
        for s in split{
            // A, X - Rock
            // B, Y - Paper
            // C, Z - Scissors
            let (opp_shape, my_shape_w) = s.split_at(1);
            let my_shape = my_shape_w.trim();
            score += my_shapes[my_shape];
            match my_shape {
                "X" => {
                    match opp_shape {
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
                    }
                }
                "Y" => {
                    match opp_shape {
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
                    }
                }
                "Z" => {
                    match opp_shape {
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
                    }
                }
                _ => {

                }
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
        for s in split{
            // A, X - Rock
            // B, Y - Paper
            // C, Z - Scissors
            let (opp_shape, my_shape_w) = s.split_at(1);
            let my_shape = my_shape_w.trim();
            // score += my_shapes[my_shape];
            match my_shape {
                "X" => {
                    match opp_shape {
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
                    }
                }
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
                _ => {

                }
            }

        }
        print!("Day 2 p2: {}\n", score);
        score
    }
}

mod day03{
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
        let (halve_a, halve_b) = data.split_at(data.len()/2);
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
                matching_chars.entry(*c).and_modify(|e| *e += 1).or_insert(1);
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
    pub fn range_fully_containing(range_a_start: usize, range_a_end: usize, range_b_start: usize, range_b_end: usize) -> bool {
        (range_a_start >= range_b_start && range_a_end <= range_b_end) || (range_b_start >= range_a_start && range_b_end <= range_a_end)
    }
    pub fn range_partially_overlapping(range_a_start: usize, range_a_end: usize, range_b_start: usize, range_b_end: usize) -> bool {
        (range_a_start <= range_b_end && range_a_start >= range_b_start) || (range_a_end <= range_b_end && range_a_end >= range_b_start) || (range_b_start <= range_a_end && range_b_start >= range_a_start) || (range_b_end <= range_a_end && range_b_end >= range_a_start)
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
mod day05{
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
        let mut lines = data.lines();
        let mut crate_stacks :Vec<Vec<char>> = Vec::new();
        let mut crate_stacks_b :Vec<Vec<char>> = Vec::new();

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
                let chars = crate_stacks_b[from - 1].drain(crate_stacks_b_from_size - num..crate_stacks_b_from_size).collect::<Vec<char>>();
                crate_stacks_b[to - 1].extend(chars);

            }
        }
        let top_str = crate_stacks.iter()
            .map(|v| v.last().unwrap())
            .collect::<String>();
        let top_str_b = crate_stacks_b.iter()
            .map(|v| v.last().unwrap())
            .collect::<String>();
        print!("Day 5: {}\n", top_str);
        print!("Day 5 p2: {}\n", top_str_b);
    }
}

mod day06{
    use std::collections::HashSet;

    pub fn get_first_datastream_char(data: String, unique_len: usize) -> usize{
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
        let mut parts = dir_str.split("/");
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
    pub fn calculate_cumulative_dir_sizes(paths: &mut Vec<(String, usize, Vec<String>)>, limit: usize) -> usize{
        let mut cumulative_size = 0;
        for path in paths.iter_mut() {
            if path.0 == "/" {
                continue;
            }
            if !path.0.ends_with("/")
            {
                continue;
            }
            let mut size = path.1;
            if size > limit {
                continue;
            }
            cumulative_size += size;
        }
        cumulative_size
    }
    pub fn find_smallest_dir_to_delete(paths: &mut Vec<(String, usize, Vec<String>)>, total_space: usize,  needed_space:usize) -> usize {
        let mut smallest_dir = String::new();
        let mut smallest_dir_size = 0;
        let current_free_space = total_space - paths[0].1;

        for path in paths.iter() {
            if path.0 == "/" {
                continue;
            }
            if !path.0.ends_with("/")
            {
                continue;
            }
            let size = path.1;
            if size + current_free_space < needed_space {
                continue;
            }
            if smallest_dir_size == 0 || size < smallest_dir_size {
                smallest_dir_size = size;
                smallest_dir = path.0.clone();
            }
        }
        // print!("smallest_dir: {}\nsize: {}\ntotal_space: {}\nneeded_space: {}\nspace after delete: {}\n", smallest_dir, smallest_dir_size, total_space, needed_space, current_free_space + smallest_dir_size);
        smallest_dir_size
    }
    pub fn get_dir_sizes(data: String) {
        let mut lines = data.lines();
        // each dir has name, a parent, size and children
        let mut paths: Vec<(String, usize, Vec<String>)> = Vec::new();
        let mut getting_output = false;
        let mut cur_dir = String::new();
        let mut prev_dir = String::new();
        let mut root_dir = String::new();
        for line in lines {
            let mut parts = line.split(" ");
            if line.starts_with('$') {
                getting_output = false;
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
                            root_dir = cur_dir.clone();
                            if new_dir == "/" {
                                paths.push((cur_dir.clone(), 0, Vec::new()));
                            }
                        }
                    },
                    "ls" => {
                        // do nothing
                    },
                    _ => {
                        cur_dir = cur_fun;
                    }
                }
            } else {
                let part_a = parts.next().unwrap();
                if part_a == "dir" {
                    paths.push((cur_dir.clone() + &parts.next().unwrap().to_string().clone() + "/", 0, Vec::new()));
                } else {
                    let file_size = part_a.parse::<usize>().unwrap();
                    let file_name = parts.next().unwrap().to_string();
                    // print!("{} {} {}\n", cur_dir, file_size, file_name);
                    // print!("paths: {:?}\n", paths);
                    // check if path exists
                    if paths.iter().find(|x| x.0 == cur_dir.clone() + &file_name).is_none() {
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




}

fn read_input(day: usize) -> String {
    std::fs::read_to_string(format!("./data/day{:0>2}.txt", day)).unwrap()
}