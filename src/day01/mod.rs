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
