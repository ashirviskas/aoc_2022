pub fn range_fully_containing(
    range_a_start: usize,
    range_a_end: usize,
    range_b_start: usize,
    range_b_end: usize,
) -> bool {
    (range_a_start >= range_b_start && range_a_end <= range_b_end)
        || (range_b_start >= range_a_start && range_b_end <= range_a_end)
}
pub fn range_partially_overlapping(
    range_a_start: usize,
    range_a_end: usize,
    range_b_start: usize,
    range_b_end: usize,
) -> bool {
    (range_a_start <= range_b_end && range_a_start >= range_b_start)
        || (range_a_end <= range_b_end && range_a_end >= range_b_start)
        || (range_b_start <= range_a_end && range_b_start >= range_a_start)
        || (range_b_end <= range_a_end && range_b_end >= range_a_start)
}

pub fn get_range_containing_number(data: String) {
    let split = data.lines();
    let mut fully_contained = 0;
    let mut partially_overlapping = 0;

    for s in split {
        let mut ranges = s.split(",");
        let mut range_a = ranges.next().unwrap().split("-");
        let mut range_b = ranges.next().unwrap().split("-");
        let range_a_start = range_a.next().unwrap().parse::<usize>().unwrap();
        let range_a_end = range_a.next().unwrap().parse::<usize>().unwrap();
        let range_b_start = range_b.next().unwrap().parse::<usize>().unwrap();
        let range_b_end = range_b.next().unwrap().parse::<usize>().unwrap();
        if range_fully_containing(range_a_start, range_a_end, range_b_start, range_b_end) {
            fully_contained += 1;
        }
        if range_partially_overlapping(range_a_start, range_a_end, range_b_start, range_b_end) {
            partially_overlapping += 1;
        }
    }
    print!("Day 4: {}\n", fully_contained);
    print!("Day 4 p2: {}\n", partially_overlapping);
}