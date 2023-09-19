pub fn count_visible_trees(data: String) {
    let lines = data.lines();
    let mut tree_heights: Vec<Vec<usize>> = Vec::new();
    for line in lines {
        let chars = line.chars();
        let mut heights: Vec<usize> = chars.map(|x| x.to_digit(10).unwrap() as usize).collect();
        tree_heights.push(heights);
    }
    let mut visible_trees: Vec<Vec<bool>> = Vec::new();
    let rows_n = tree_heights.len();
    let cols_n = tree_heights[0].len();
    // visible from top
    // visible_trees.push(vec![true; tree_heights[0].len()]);
    let mut cur_max_heights: Vec<i32> = vec![-1; cols_n];
    let mut trees_iter = tree_heights.iter();
    for (i, heights) in trees_iter.enumerate() {
        let mut cur_visible: Vec<bool> = Vec::new();
        for j in 0..heights.len() {
            if cur_max_heights[j] >= heights[j] as i32 {
                cur_visible.push(false);
            } else {
                cur_visible.push(true);
                cur_max_heights[j] = heights[j] as i32;
            }
        }
        visible_trees.push(cur_visible);
    }
    // visible from bottom
    cur_max_heights = vec![-1; cols_n];
    let mut trees_iter = tree_heights.iter().rev();
    for (i, heights) in trees_iter.enumerate() {
        let mut cur_visible: Vec<bool> = Vec::new();
        for j in 0..heights.len() {
            if cur_max_heights[j] >= heights[j] as i32 {
                cur_visible.push(false);
            } else {
                // visible_trees[visible_trees.len() - i - 1][j] = true;
                cur_visible.push(true);
                cur_max_heights[j] = heights[j] as i32;
            }
        }
        // logical or
        visible_trees[rows_n - i - 1] = visible_trees[rows_n - i - 1]
            .iter()
            .zip(cur_visible.iter())
            .map(|(x, y)| *x || *y)
            .collect();
    }
    for (i, heights) in tree_heights.iter().enumerate() {
        let mut cur_max_height_left: i32 = -1;
        let mut cur_max_height_right: i32 = -1;
        for j in 0..heights.len() {
            if cur_max_height_left >= heights[j] as i32 {
                visible_trees[i][j] = visible_trees[i][j] || false;
            } else {
                visible_trees[i][j] = true;
                cur_max_height_left = heights[j] as i32;
            }

            if cur_max_height_right >= heights[heights.len() - j - 1] as i32 {
                visible_trees[i][heights.len() - j - 1] =
                    visible_trees[i][heights.len() - j - 1] || false;
            } else {
                visible_trees[i][heights.len() - j - 1] = true;
                cur_max_height_right = heights[heights.len() - j - 1] as i32;
            }
        }
    }

    let mut visible_trees_count = visible_trees
        .iter()
        .fold(0, |acc, x| acc + x.iter().filter(|x| **x).count());
    print!("Day 8: {}\n", visible_trees_count);
}

pub fn get_most_scenic_tree(data: String) {
    let lines = data.lines().collect::<Vec<_>>();
    let rows_n_soft = lines.len();
    let cols_n_soft = lines[0].len();
    const rows_n: usize = 100;
    const cols_n: usize = 100;
    let mut tree_heights = [[0; cols_n]; rows_n];
    let mut tree_scores = [[0; cols_n]; rows_n];
    for (i, line) in lines.iter().enumerate() {
        let chars = line.chars();
        let mut heights: Vec<usize> = chars.map(|x| x.to_digit(10).unwrap() as usize).collect();
        for (j, height) in heights.iter().enumerate() {
            tree_heights[i][j] = *height;
        }
    }
    // Calculating visible trees for each tree below, above, left and right of it
    for i in 0..rows_n_soft {
        for j in 0..cols_n_soft {
            let mut above = 0;
            let mut below = 0;
            let mut left = 0;
            let mut right = 0;

            if i == 0 || j == 0 || i == rows_n_soft - 1 || j == cols_n_soft - 1 {
                continue;
            }

            for ii in i + 1..rows_n_soft {
                if tree_heights[ii][j] < tree_heights[i][j] {
                    below += 1;
                } else {
                    below += 1;
                    break;
                }
            }

            for ii in (0..i).rev() {
                if tree_heights[ii][j] < tree_heights[i][j] {
                    above += 1;
                } else {
                    above += 1;
                    break;
                }
            }
            for jj in j + 1..cols_n_soft {
                if tree_heights[i][jj] < tree_heights[i][j] {
                    right += 1;
                } else {
                    right += 1;
                    break;
                }
            }
            for jj in (0..j).rev() {
                if tree_heights[i][jj] < tree_heights[i][j] {
                    left += 1;
                } else {
                    left += 1;
                    break;
                }
            }

            tree_scores[i][j] = above * below * left * right;
        }
    }
    print!(
        "Day 8: {}\n",
        tree_scores
            .iter()
            .fold(0, |acc, x| acc.max(*x.iter().max().unwrap()))
    )
}