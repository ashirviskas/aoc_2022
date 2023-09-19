use pathfinding::prelude::dijkstra;

    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct Pos(usize, usize, char);

    impl Pos {
        pub fn neighbours(&self, char_map: &Vec<Vec<char>>, do_reverse:bool) -> Vec<(Pos, usize)> {
            let mut neighbours = Vec::new();
            let positions = vec![
                (self.0 as i32 - 1, self.1 as i32 ),
                (self.0 as i32 + 1, self.1 as i32 ),
                (self.0 as i32 , self.1 as i32  - 1),
                (self.0 as i32 , self.1 as i32  + 1),
            ];
            for pos in positions {
                if pos.0 < char_map.len() as i32 && pos.0 >= 0 && pos.1 < char_map[0].len() as i32 && pos.1 >= 0 {
                    let pos = (pos.0 as usize, pos.1 as usize);
                    let val = get_val_cost(self.2, char_map[pos.0][pos.1], do_reverse);
                    if val != 10000 {
                        neighbours.push((Pos(pos.0, pos.1, char_map[pos.0][pos.1]), val));
                    }
                }
            }
            // println!("{:?}", neighbours);
            neighbours
        }
    }


    pub fn get_val_cost(cur_char: char, other_char: char, do_reverse: bool) -> usize {
        let cur_char_val = get_char_cost(cur_char) as i32;
        let other_val = get_char_cost(other_char) as i32;

        let score = other_val - cur_char_val;
        
        if score > 1 && !do_reverse {
            return 10000;
        }
        if score < -1 && do_reverse {
            return 10000;
        }

        return 1;
    }

    pub fn get_char_cost(character:char) -> usize {
        if character == 'S' {
            return 0;
        }
        if character == 'E' {
            return 'z' as usize - 'a' as usize;
        }
        return character as usize - 'a' as usize;
    }

    pub fn build_path_map(positions: Vec<Pos>, char_map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
        let mut travel_map = vec![vec!['.'; char_map[0].len()]; char_map.len()];
        let mut previous_pos = positions[0].clone();
        for position in positions {
            // if right
            if position.1 > previous_pos.1 {
                travel_map[previous_pos.0][previous_pos.1] = '>';
            }
            // if left
            if position.1 < previous_pos.1 {
                travel_map[previous_pos.0][previous_pos.1] = '<';
            }
            // if up
            if position.0 < previous_pos.0 {
                travel_map[previous_pos.0][previous_pos.1] = '^';
            }
            // if down
            if position.0 > previous_pos.0 {
                travel_map[previous_pos.0][previous_pos.1] = 'v';
            }
            previous_pos = position;
        }
        travel_map
    }
    pub fn hill_climb_racing(data:String) {
        let char_map = data.lines().map(|x| x.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
        let start_pos = char_map.iter().enumerate().flat_map(|(y, x)| x.iter().enumerate().filter_map(move |(x, c)| if *c == 'S' { Some((y, x)) } else { None })).next().unwrap();
        let result_a = dijkstra(&Pos(start_pos.0, start_pos.1, 'S'), |x| x.neighbours(&char_map, false), |x| x.2 == 'E');
        let result_a = result_a.unwrap();
        print!("Day 12: {}\n", result_a.0.len() -1);

        let start_pos_b = char_map.iter().enumerate().flat_map(|(y, x)| x.iter().enumerate().filter_map(move |(x, c)| if *c == 'E' { Some((y, x)) } else { None })).next().unwrap();
        let result_b = dijkstra(&Pos(start_pos_b.0, start_pos_b.1, 'E'), |x| x.neighbours(&char_map, true), |x| x.2 == 'a');
        let result_b = result_b.unwrap();
        print!("Day 12 p2: {}\n", result_b.0.len() -1);
        // let travel_map = build_path_map(result_a.0, &char_map);
        // for line in travel_map {
        //     for character in line {
        //         print!("{}", character);
        //     }
        //     println!("");
        // }
    }