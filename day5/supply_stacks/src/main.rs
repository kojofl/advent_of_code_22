use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Default, Clone)]
pub struct Stack<T> {
    inner: Vec<T>,
}

impl<T> Stack<T> {
    pub const fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.inner.pop()
    }

    pub fn take(&mut self, amount: usize) -> Vec<T> {
        self.inner.split_off(self.inner.len() - amount)
    }

    pub fn push(&mut self, val: T) {
        self.inner.push(val)
    }

    pub fn concat(&mut self, mut other: Vec<T>) {
        self.inner.append(&mut other)
    }

    pub fn reverse(&mut self) {
        self.inner.reverse()
    }
}

#[derive(Debug, Clone)]
pub struct SupplyStack<T, const N: usize> {
    inner: [Stack<T>; N],
}

impl<T, const N: usize> SupplyStack<T, N> {
    pub fn new() -> Self {
        Self {
            inner: [(); N].map(|_| Stack::new()),
        }
    }

    pub fn insert(&mut self, index: usize, value: T) {
        self.inner[index].push(value)
    }
}

fn main() {
    let file = File::open("./input.txt").expect("File not found");

    let reader = BufReader::new(file);

    let mut supply: SupplyStack<char, 9> = SupplyStack::new();

    let mut building_stack = true;

    for line in reader.lines() {
        let Ok(line) = line else {
            continue;
        };
        if building_stack {
            for (i, &byte) in line.as_bytes().iter().enumerate() {
                if !byte.is_ascii_alphabetic() {
                    continue;
                }
                supply.insert(i / 4, char::from(byte));
            }
            if !line.as_bytes().contains(&91) {
                supply.inner.iter_mut().for_each(|s| s.reverse());
                building_stack = false
            }
        } else {
            // Execute commands
            let commands: Vec<i32> = line
                .split_ascii_whitespace()
                .into_iter()
                .filter_map(|s| s.parse::<i32>().ok())
                .collect();
            if commands.len() != 3 {
                continue;
            }
            let (count, from, to) = (
                commands[0] as usize,
                commands[1] as usize,
                commands[2] as usize,
            );

            let from = supply.inner[from - 1].take(count);
            supply.inner[to - 1].concat(from);
        }
    }
    supply.inner.iter_mut().for_each(|stack| {
        print!("{:?}", stack.pop());
    });
}
