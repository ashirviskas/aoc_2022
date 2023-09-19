use std::str::Lines;

    #[derive(Clone)]
    pub struct Monke{
        starting_items: Vec<u64>,
        operation: (usize, usize, bool),
        test_case: usize,
        conditional_destination: (usize, usize),
    }

    pub fn read_monkey(data: String, monkes: &mut Vec<Monke>) {
        let mut lines = data.lines();
        // Starting items
        let starting_items_line = lines.next().unwrap();
        let starting_items_str = starting_items_line.split_at(18).1;
        let starting_items_m = starting_items_str.split(",");
        let starting_items_vec: Vec<u64> = starting_items_m.map(|x| x.trim().parse::<u64>().unwrap()).collect();
        // Operation parsing
        let operations_line = lines.next().unwrap();
        let operations_str = operations_line.split_at(19).1;
        let mut operation = (0, 0, false);
        if operations_str.matches("old").count() == 2 {
            operation = (0, 0, true);
        } else {
            let operation_char = operations_str.chars().nth(4).unwrap();
            let operation_value = operations_str.split_at(5).1.trim().parse::<usize>().unwrap();
            match operation_char {
                '+' => {
                    operation = (operation_value, 0, false);
                }
                '*' => {
                    operation = (0, operation_value, false);
                }
                _ => panic!("Unknown operation"),
            }
        }
        // Test case
        let test_case_line = lines.next().unwrap();    
        let test_case_str = test_case_line.split_at(21).1;
        let test_case = test_case_str.trim().parse::<usize>().unwrap();
        // Conditional destination

        let conditional_true_line = lines.next().unwrap();
        let conditional_true_str = conditional_true_line.split_at(28).1;
        let conditional_true = conditional_true_str.trim().parse::<usize>().unwrap();

        let conditional_false_line = lines.next().unwrap();
        let conditional_false_str = conditional_false_line.split_at(29).1;
        let conditional_false = conditional_false_str.trim().parse::<usize>().unwrap();

        let conditional_destination = (conditional_true, conditional_false);

        monkes.push(Monke{
            starting_items: starting_items_vec,
            operation: operation,
            test_case: test_case,
            conditional_destination: conditional_destination,
        });

    }
    pub fn take_n_lines(lines: &mut Lines<'_>, n: usize) -> String {
        return lines.take(n).collect::<Vec<&str>>().join("\n");
    }

    pub fn do_rounds(monkes: &mut Vec<Monke>, worry_function: impl Fn(u64) -> u64, n_rounds: usize) -> u64{
        let mut inspections_count: Vec<usize> = vec![0; monkes.len()];
        for _ in 0..n_rounds {
            for i in 0..monkes.len() {
                inspections_count[i] += monkes[i].starting_items.len();
                let operation = &monkes[i].operation.clone();
                let test_case = monkes[i].test_case;
                let conditional_destinations = monkes[i].conditional_destination;
                while monkes[i].starting_items.len() > 0 {
                    let item = monkes[i].starting_items.remove(0).clone();
                    let mut new_item = item;
                    if operation.2 {
                        new_item = new_item * new_item;
                    }
                    if operation.0 != 0 {
                        new_item += operation.0 as u64;
                    }
                    if operation.1 != 0 {
                        new_item *= operation.1 as u64;
                    }
                    new_item = worry_function(new_item);
                    let destination:usize;

                    if new_item % test_case as u64 == 0 {
                        destination = conditional_destinations.0;
                    } else {
                        destination = conditional_destinations.1;
                    }
                    monkes[destination].starting_items.push(new_item);
                }
            }
        }
        inspections_count.sort();
        let top_two_inspection_count = inspections_count.iter().rev().take(2).collect::<Vec<&usize>>();
        let monkey_business = *top_two_inspection_count[0] as u64 * *top_two_inspection_count[1] as u64;
        return monkey_business;
    }

    pub fn play_keep_away(data: String) {
        let mut monkes: Vec<Monke> = Vec::new();
        let mut inspections_count: Vec<usize> = Vec::new();

        let mut lines = data.lines();
        // skip first, get next 5 lines
        lines.next();
        let mut monkey_data = take_n_lines(&mut lines, 5);


        while monkey_data != "" {
            read_monkey(monkey_data, &mut monkes);
            inspections_count.push(0);
            lines.next();
            lines.next();
            monkey_data = take_n_lines(&mut lines, 5);
        }
        let monke_product = monkes.iter().map(|x| x.test_case).product::<usize>();

        
        let monkey_business = do_rounds(&mut monkes.clone(), |x| x / 3, 20);
        println!("Day 11: {}", monkey_business);

        let monkey_business = do_rounds(&mut monkes, |x| x % monke_product as u64, 10000);
        println!("Day 11 p2: {}", monkey_business);


        
    }