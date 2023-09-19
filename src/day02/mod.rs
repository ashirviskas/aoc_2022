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