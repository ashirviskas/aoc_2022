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