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