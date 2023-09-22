pub fn get_parent_dir(dir_str: String) -> String {
    let parts = dir_str.split("/");
    let mut new_dir = "/".to_string();
    let new_parts_num = parts.clone().count() - 2;
    if new_parts_num == 1 {
        return new_dir;
    } else if new_parts_num == 0 {
        return "".to_string();
    }
    for (i, part) in parts.enumerate() {
        if i == new_parts_num {
            break;
        }
        if part == "" {
            continue;
        }
        new_dir.push_str(part);
        new_dir.push_str("/");
    }
    // print!("new_dir: {}\n", new_dir);
    new_dir
}
pub fn calculate_cumulative_dir_sizes(
    paths: &mut Vec<(String, usize, Vec<String>)>,
    limit: usize,
) -> usize {
    let mut cumulative_size = 0;
    for path in paths.iter_mut() {
        if path.0 == "/" {
            continue;
        }
        if !path.0.ends_with("/") {
            continue;
        }
        let size = path.1;
        if size > limit {
            continue;
        }
        cumulative_size += size;
    }
    cumulative_size
}
pub fn find_smallest_dir_to_delete(
    paths: &mut Vec<(String, usize, Vec<String>)>,
    total_space: usize,
    needed_space: usize,
) -> usize {
    let mut smallest_dir_size = 0;
    let current_free_space = total_space - paths[0].1;

    for path in paths.iter() {
        if path.0 == "/" {
            continue;
        }
        if !path.0.ends_with("/") {
            continue;
        }
        let size = path.1;
        if size + current_free_space < needed_space {
            continue;
        }
        if smallest_dir_size == 0 || size < smallest_dir_size {
            smallest_dir_size = size;
        }
    }
    // print!("smallest_dir: {}\nsize: {}\ntotal_space: {}\nneeded_space: {}\nspace after delete: {}\n", smallest_dir, smallest_dir_size, total_space, needed_space, current_free_space + smallest_dir_size);
    smallest_dir_size
}
pub fn get_dir_sizes(data: String) {
    let lines = data.lines();
    // each dir has name, a parent, size and children
    let mut paths: Vec<(String, usize, Vec<String>)> = Vec::new();
    let mut cur_dir = String::new();
    // let mut prev_dir = String::new();
    for line in lines {
        let mut parts = line.split(" ");
        if line.starts_with('$') {
            parts.next();
            let cur_fun = parts.next().unwrap().to_string();
            match cur_fun.as_str() {
                "cd" => {
                    let prev_dir = cur_dir.clone();
                    let new_dir = parts.next().unwrap();
                    if new_dir == ".." {
                        cur_dir = get_parent_dir(cur_dir);
                    } else {
                        if cur_dir != "/" {
                            if new_dir == "/" {
                                cur_dir = new_dir.to_string();
                            } else {
                                cur_dir = cur_dir + &new_dir.to_string() + "/";
                            }
                        } else {
                            cur_dir = cur_dir + &new_dir.to_string() + "/";
                        }
                    }
                    if prev_dir == "" {
                        if new_dir == "/" {
                            paths.push((cur_dir.clone(), 0, Vec::new()));
                        }
                    }
                }
                "ls" => {
                    // do nothing
                }
                _ => {
                    cur_dir = cur_fun;
                }
            }
        } else {
            let part_a = parts.next().unwrap();
            if part_a == "dir" {
                paths.push((
                    cur_dir.clone() + &parts.next().unwrap().to_string().clone() + "/",
                    0,
                    Vec::new(),
                ));
            } else {
                let file_size = part_a.parse::<usize>().unwrap();
                let file_name = parts.next().unwrap().to_string();
                // print!("{} {} {}\n", cur_dir, file_size, file_name);
                // print!("paths: {:?}\n", paths);
                // check if path exists
                if paths
                    .iter()
                    .find(|x| x.0 == cur_dir.clone() + &file_name)
                    .is_none()
                {
                    paths.push((cur_dir.clone() + &file_name, file_size, Vec::new()));
                }
                let dir = paths.iter_mut().find(|x| x.0 == cur_dir).unwrap();
                dir.1 += file_size;
                dir.2.push(file_name);
                let mut dir_parent = get_parent_dir(cur_dir.clone());
                while dir_parent != "" {
                    let dir = paths.iter_mut().find(|x| x.0 == dir_parent).unwrap();
                    dir.1 += file_size;
                    dir_parent = get_parent_dir(dir_parent);
                }
            }
        }
    }
    // let root = paths.iter().find(|x| x.0 == root_dir).unwrap().1;
    let cumulative_path_size = calculate_cumulative_dir_sizes(&mut paths, 100000);
    let to_delete_dir = find_smallest_dir_to_delete(&mut paths, 70000000, 30000000);

    print!("Day 7: {}\n", cumulative_path_size);
    print!("Day 7 p2: {}\n", to_delete_dir);
}