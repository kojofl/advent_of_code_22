use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::RangeInclusive,
};

trait ExtensiveRange {
    fn contains_range(&self, other: &Self) -> bool;
    fn overlap(&self, other: &Self) -> bool;
}

impl<T: PartialOrd> ExtensiveRange for RangeInclusive<T> {
    fn contains_range(&self, other: &Self) -> bool {
        self.contains(other.start()) && self.contains(other.end())
    }

    fn overlap(&self, other: &Self) -> bool {
        self.contains(other.start()) || self.contains(other.end())
    }
}

fn main() {
    let input_file = File::open("./input.txt").expect("Input file not found!");
    let reader = BufReader::new(input_file);

    let overlapping = reader.lines().fold((0, 0), |acc, line| {
        let Ok(line) = line else {
            return acc;
        };
        let mut a_b = line.split(",");
        let mut a_range = a_b.next().unwrap().split("-");
        let mut b_range = a_b.next().unwrap().split("-");
        let a_range = a_range.next().unwrap().parse::<i32>().unwrap()
            ..=a_range.next().unwrap().parse::<i32>().unwrap();
        let b_range = b_range.next().unwrap().parse::<i32>().unwrap()
            ..=b_range.next().unwrap().parse::<i32>().unwrap();
        if a_range.contains_range(&b_range) || b_range.contains_range(&a_range) {
            (acc.0 + 1, acc.1 + 1)
        } else if a_range.overlap(&b_range) || b_range.overlap(&a_range) {
            (acc.0 + 1, acc.1)
        } else {
            acc
        }
    });

    println!("{overlapping:?}")
}

#[test]
fn test() {
    let mut v = Vec::with_capacity(13);

    println!("{}", v.capacity());

    v.push(1);

    println!("{}", v.capacity());

    v.push(1);
    v.push(1);
    v.push(1);
    v.push(1);
    v.push(1);
    v.push(1);
    v.push(1);
    v.push(1);
    v.push(1);
    v.push(1);
    v.push(1);
    v.push(1);
    v.push(1);
    v.push(1);
    v.push(1);
    v.push(1);
    v.push(1);
    v.push(1);
    v.push(1);
    v.push(1);

    println!("{}", v.capacity());
}
