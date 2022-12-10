
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
fn main() {
    day01::find_max(read_input(1));
    day02::calculate_scores(read_input(2));
    day02::calculate_scores_part2(read_input(2));
    day03::get_rucksacks_total_priority(read_input(3));
    day03::get_three_groupings_total_priority(read_input(3));
    day04::get_range_containing_number(read_input(4));
    day05::reorder_crates(read_input(5));
    day06::do_both(read_input(6));




}

fn read_input(day: usize) -> String {
    std::fs::read_to_string(format!("./data/day{:0>2}.txt", day)).unwrap()
}