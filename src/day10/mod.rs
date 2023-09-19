pub fn check_cycle(base: usize, every_nth: usize, cycle_index: usize) -> bool {
    if cycle_index < base {
        return false;
    }
    if (cycle_index - base) % every_nth == 0 {
        return true;
    }
    false
}

pub fn draw_pixel(width: usize, cycle_index: usize, register: i32) {
    let pos_x = (cycle_index - 1) % width;

    let draw_pixel = (register == (pos_x as i32) -1) || (register == (pos_x as i32)) || (register == (pos_x as i32) + 1);
    if draw_pixel {
        print!("#");
    } else {
        print!(".");
    }
    if pos_x == width - 1 {
        print!("\n");
    }
}

pub fn simulate_cpu(data: String) {
    let lines = data.lines();
    let mut register: i32 = 1;
    let mut cycle_index: usize = 0;
    let cycle_base = 20;
    let cycle_every_nth = 40;
    let display_width = 40;
    let mut signal_strength: i32 = 0;
    print!("Day 10 p2:\n");

    for line in lines {
        let (command, value) = line.split_at(4);
        let value = value.trim().parse::<i32>().unwrap_or(0);
        match command {
            "noop" => {
                cycle_index += 1;
                if check_cycle(cycle_base, cycle_every_nth, cycle_index) {
                    let cycle_strength = register * cycle_index as i32;
                    signal_strength += cycle_strength;
                }
                draw_pixel(display_width, cycle_index, register);
            }   
            "addx" => {
                cycle_index += 1;
                if check_cycle(cycle_base, cycle_every_nth, cycle_index) {
                    let cycle_strength = register * cycle_index as i32;
                    signal_strength += cycle_strength;
                }
                draw_pixel(display_width, cycle_index, register);
                cycle_index += 1;
                if check_cycle(cycle_base, cycle_every_nth, cycle_index) {
                    let cycle_strength = register * cycle_index as i32;
                    signal_strength += cycle_strength;
                }
                draw_pixel(display_width, cycle_index, register);
                register += value;
            }
            _ => panic!("Unknown command"),
        }
    }
    print!("Day 10: {}\n", signal_strength);

}