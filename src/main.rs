mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day23;
fn main() {
    day01::find_max(read_input(1));
    day02::calculate_scores(read_input(2));
    day02::calculate_scores_part2(read_input(2));
    day03::get_rucksacks_total_priority(read_input(3));
    day03::get_three_groupings_total_priority(read_input(3));
    day04::get_range_containing_number(read_input(4));
    day05::reorder_crates(read_input(5));
    day06::do_both(read_input(6));
    day07::get_dir_sizes(read_input(7));
    day08::count_visible_trees(read_input(8));
    day08::get_most_scenic_tree(read_input(8));
    day09::calculate_tail_moves(read_input(9));
    day09::calculate_multiple_knots_moves(read_input(9));
    day10::simulate_cpu(read_input(10));
    day11::play_keep_away(read_input(11));
    day12::hill_climb_racing(read_input(12));
    day13::do_brackets(read_input(13));
    day14::do_sand(read_input(14));
    day15::beacon_the_sensors(read_input(151), 10, 20);
    day15::beacon_the_sensors(read_input(15), 2000000, 4000000);
    day23::game_of_elves(read_input(23));
}

fn read_input(day: usize) -> String {
    std::fs::read_to_string(format!("./data/day{:0>2}.txt", day)).unwrap()
}
