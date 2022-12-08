use std::{
    collections::BinaryHeap,
    fs,
    io::{BufRead, BufReader},
};

fn main() {
    let file = fs::File::open("./input.txt").expect("File not found!");
    let mut contestant = 0;
    let reader = BufReader::new(file);
    let mut bin_heap = BinaryHeap::new();
    for line in reader.lines() {
        let Ok(line) = line else {
            continue;
        };
        if line.is_empty() {
            bin_heap.push(contestant);
            contestant = 0;
        } else {
            contestant += line.parse::<i32>().expect("Could not parse line to int")
        }
    }
    let max = bin_heap.peek().unwrap();
    println!("Max calories on a single Elf: {}", max);

    let top_3 = take::<3, i32>(&mut bin_heap);
    println!("{:?} => {}", top_3, top_3.iter().sum::<i32>())
}

fn take<const S: usize, T: Ord>(bin_heap: &mut BinaryHeap<T>) -> Vec<T> {
    (0..S)
        .into_iter()
        .map(|_| bin_heap.pop().unwrap())
        .collect()
}
