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