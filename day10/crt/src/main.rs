use std::{
    collections::VecDeque,
    error::Error,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

struct CRT {
    display: [[char; 40]; 6],
    cpu: CRTCpu,
}

impl CRT {
    pub fn new() -> Self {
        Self {
            display: [[' '; 40]; 6],
            cpu: CRTCpu::new(),
        }
    }

    pub fn run_crt(&mut self) {
        for rows in self.display {
            for (index, pixel) in rows.iter().enumerate() {
                self.print(index);
                self.cpu.run_cycle();
            }
            println!()
        }
    }

    pub fn print(&self, display_collumn: usize) {
        let sprinte_position = self.cpu.x;
        if sprinte_position.abs_diff(display_collumn as i64) <= 1 {
            print!("#");
        } else {
            print!(".");
        }
    }
}

#[derive(Debug)]
struct CRTCpu {
    pub x: i64,
    operation_queue: VecDeque<Operation>,
    cycle: u64,
}

impl CRTCpu {
    pub fn new() -> Self {
        Self {
            x: 1,
            operation_queue: VecDeque::new(),
            cycle: 0,
        }
    }

    pub fn run(&mut self) {
        while let Some(op) = self.operation_queue.pop_back() {
            match op {
                Operation::Noop => {}
                Operation::Add(v) => self.x += v,
            }
            self.cycle += 1;
        }
    }

    pub fn run_until(&mut self, until: u64) -> Result<(), u64> {
        while self.cycle < until {
            if let Some(op) = self.operation_queue.pop_back() {
                match op {
                    Operation::Noop => {}
                    Operation::Add(v) => self.x += v,
                }
                self.cycle += 1;
            } else {
                return Err(self.cycle);
            }
        }
        Ok(())
    }

    pub fn run_cycle(&mut self) -> Result<(), u64> {
        if let Some(op) = self.operation_queue.pop_back() {
            match op {
                Operation::Noop => {}
                Operation::Add(v) => self.x += v,
            }
            self.cycle += 1;
            Ok(())
        } else {
            return Err(1);
        }
    }

    pub fn add_operation(&mut self, operation: Operation) {
        match operation {
            Operation::Noop => self.operation_queue.push_front(operation),
            Operation::Add(_) => {
                self.operation_queue.push_front(Operation::Noop);
                self.operation_queue.push_front(operation)
            }
        }
    }
}

#[derive(Debug)]
enum Operation {
    Noop,
    Add(i64),
}

#[derive(Debug)]
struct ParseOperationError {
    s: &'static str,
}

impl Display for ParseOperationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.s)
    }
}

impl Error for ParseOperationError {}

impl FromStr for Operation {
    type Err = ParseOperationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" ");
        match split.next().unwrap() {
            "noop" => Ok(Operation::Noop),
            "addx" => {
                let v = split.next().unwrap().parse::<i64>().unwrap();
                Ok(Operation::Add(v))
            }
            _ => Err(ParseOperationError {
                s: "Help could not parse String",
            }),
        }
    }
}

fn main() {
    let file = File::open("./input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut crt = CRT::new();

    for line in reader.lines() {
        let Ok(line) = line else {
            continue;
        };
        crt.cpu.add_operation(line.parse().unwrap())
    }
    crt.run_crt()
}
