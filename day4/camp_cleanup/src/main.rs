use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::RangeInclusive,
};

fn main() {
    let input_file = File::open("./input.txt").expect("Input file not found!");
    let reader = BufReader::new(input_file);

    let mut vector: Vec<(RangeInclusive<usize>, RangeInclusive<usize>)> = Vec::new();

    for line in reader.lines() {
        let Ok(line) = line else {
            continue;
        };
        let mut a_b = line.split(",");
        let mut a_range = a_b.next().unwrap().split("-");
        let mut b_range = a_b.next().unwrap().split("-");
        vector.push((
            (a_range.next().unwrap().parse().unwrap()..=a_range.next().unwrap().parse().unwrap()),
            (b_range.next().unwrap().parse().unwrap()..=b_range.next().unwrap().parse().unwrap()),
        ))
    }

    let overlapping = vector.into_iter().fold(0, |mut acc, (range_a, range_b)| {
        if (range_a.contains(range_b.start()) || range_a.contains(range_b.end()))
            || (range_b.contains(range_a.start()) || range_b.contains(range_a.end()))
        {
            acc += 1;
        }
        acc
    });

    println!("{overlapping}")
}
