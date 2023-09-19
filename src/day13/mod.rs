use std::cmp::Ordering;
    use std::fmt::Display;
    use std::fmt::Formatter;

    // Derive comparison trait
    #[derive(Debug, Clone)]
    pub struct ListElement {
        value_number: Option<usize>,
        value_list: Option<Vec<ListElement>>,
    }

    impl ListElement {
        pub fn new(data: String) -> ListElement {
            if data.starts_with('[') {
                ListElement {
                    value_number: None,
                    value_list: Some(get_elements_from_string(&data)),
                }
            } else if data.len() == 0 {
                ListElement {
                    value_number: None,
                    value_list: Some(vec![]),
                }
            } else {
                ListElement {
                    value_number: Some(data.parse::<usize>().unwrap()),
                    value_list: None,
                }
            }
        }
    }

    impl Ord for ListElement {
        fn cmp(&self, other: &ListElement) -> Ordering {
            // println!("Comparing {} and {}", self, other);
            if self.value_number.is_some() && other.value_number.is_some() {
                self.value_number.unwrap().cmp(&other.value_number.unwrap())
            } else if self.value_number.is_some() && other.value_list.is_some() {
                let new_list_element = ListElement {
                    value_number: None,
                    value_list: Some(vec![self.clone()]),
                };
                // Debug printing
                // println!("Original: {} vs {}", self, other);
                // println!("New: {} vs {}", new_list_element, other);
                new_list_element.cmp(&other)
            } else if self.value_list.is_some() && other.value_number.is_some() {
                let new_list_element = ListElement {
                    value_number: None,
                    value_list: Some(vec![other.clone()]),
                };
                // Debug printing
                // println!("Original: {} vs {}", self, other);
                // println!("New: {} vs {}", self, new_list_element);
                self.cmp(&new_list_element)
            } else {
                for (i, element) in self.value_list.as_ref().unwrap().iter().enumerate() {
                    if i >= other.value_list.as_ref().unwrap().len() {
                        return std::cmp::Ordering::Greater;
                    }
                    let other_element = &other.value_list.as_ref().unwrap()[i];
                    let result = element.cmp(other_element);
                    if result != std::cmp::Ordering::Equal {
                        return result;
                    }
                }
                if self.value_list.as_ref().unwrap().len() < other.value_list.as_ref().unwrap().len() {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Equal
                }
            }
        }
    }

    impl PartialOrd for ListElement {
        fn partial_cmp(&self, other: &ListElement) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl PartialEq for ListElement {
        fn eq(&self, other: &ListElement) -> bool {
            self.cmp(other) == Ordering::Equal
        }
    }

    impl Eq for ListElement {}

    impl Display for ListElement {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            if self.value_number.is_some() {
                write!(f, "{}", self.value_number.unwrap())
            } else {
                write!(f, "[")?;
                for (i, element) in self.value_list.as_ref().unwrap().iter().enumerate() {
                    if i != 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "{}", element)?;
                }
                write!(f, "]")
            }
        }
    }

    pub fn get_elements_from_string(data: &str) -> Vec<ListElement> {
        let mut elements = Vec::new();
        // let mut element = String::new();
        let mut running_string = String::new();
        let mut running_brackets = 0;

        for (i, character) in data.chars().enumerate() {
            if i == 0 {
                continue;
            }
            if i == data.len() - 1 {
                continue;
            }

            if character == '[' {
                running_brackets += 1;
            }
            if character == ']' {
                running_brackets -= 1;

                if running_string == "[" {
                    running_string = "[]".to_string();
                    continue
                }
            }
            if character == ',' && running_brackets == 0 {
                let element = ListElement::new(running_string);
                elements.push(element);
                running_string = String::new();
            } else {
                running_string.push(character);
            }
        }
        let element = ListElement::new(running_string);
        elements.push(element);
        // println!("{:?}", elements);
        elements
        
    }

    pub fn do_brackets(data: String) {
        let mut pairs = data.split("\n\n").map(|x| x.to_string()).collect::<Vec<String>>();
        let mut matching_indices_sum = 0;

        for (i, pair) in pairs.iter().enumerate() {
            let (pair_a, pair_b) = pair.split_once("\n").unwrap();
            let pair_a = ListElement::new(pair_a.to_string());
            let pair_b = ListElement::new(pair_b.to_string());
            // println!("{} vs {} = {:?}", pair_a, pair_b, pair_a.cmp(&pair_b));

            let comparison = pair_a.cmp(&pair_b);
            if comparison == Ordering::Less {
                matching_indices_sum += i + 1;
            }
            // println!("{} vs {} = {:?}", pair_a, pair_b, comparison);
        }
        println!("Day 13: {}", matching_indices_sum);
        // all brackets, skip empty liness, use ListElement
        let mut all_brackets = data.split("\n").filter(|x| x.len() > 0).map(|x| x.to_string()).map(|x| ListElement::new(x)).collect::<Vec<ListElement>>();
        // Add 2 more custom brackets
        all_brackets.append(&mut vec![ListElement::new("[[2]]".to_string()), ListElement::new("[[6]]".to_string())]);
        // let all_brackets_str = all_brackets.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("\n");
        // println!("Day 13 p2: {:?}", all_brackets_str);
        all_brackets.sort();
        // let all_brackets_str_sorted = all_brackets.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("\n");
        let custom_brackets_index_a = all_brackets.iter().position(|x| x.to_string() == "[[2]]").unwrap();
        let custom_brackets_index_b = all_brackets.iter().position(|x| x.to_string() == "[[6]]").unwrap();
        println!("Day 13 p2: {}x{}={}", custom_brackets_index_a + 1, custom_brackets_index_b + 1, (custom_brackets_index_a + 1) * (custom_brackets_index_b + 1));
    }