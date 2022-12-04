
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
}
fn main() {
    day01::find_max(read_input(1));
    day02::calculate_scores(read_input(2));

}

fn read_input(day: usize) -> String {
    std::fs::read_to_string(format!("./data/day{:0>2}.txt", day)).unwrap()
}