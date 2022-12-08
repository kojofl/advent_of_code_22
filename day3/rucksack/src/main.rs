use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

struct Rucksack(Vec<(HashSet<char>, HashSet<char>)>);

impl Rucksack {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn insert(&mut self, compartment_a: HashSet<char>, compartment_b: HashSet<char>) {
        self.0.push((compartment_a, compartment_b))
    }

    fn calc_prio_sum(&self, prio_map: &HashMap<char, i32>) -> i32 {
        let mut prio_sum = 0;
        for (a, b) in &self.0 {
            for intersect in (*a).intersection(b) {
                prio_sum += prio_map.get(intersect).unwrap()
            }
        }
        prio_sum
    }

    fn calc_group_prio_sum(&self, prio_map: &HashMap<char, i32>) -> i32 {
        let mut prio_sum = 0;
        for window in self.0.windows(3).step_by(3) {
            let mut elf_a = unsafe { window.get_unchecked(0).clone() };
            let mut elf_b = unsafe { window.get_unchecked(1).clone() };
            let mut elf_c = unsafe { window.get_unchecked(2).clone() };
            elf_a.0.extend(elf_a.1.iter());
            elf_b.0.extend(elf_b.1.iter());
            elf_c.0.extend(elf_c.1.iter());
            for intersection_a_b in elf_a.0.intersection(&elf_b.0) {
                if let Some(intersect_c) = elf_c.0.get(intersection_a_b) {
                    prio_sum += prio_map.get(intersect_c).unwrap();
                    break;
                }
            }
        }
        prio_sum
    }
}

fn main() {
    let priority_map = HashMap::from([
        ('a', 1),
        ('b', 2),
        ('c', 3),
        ('d', 4),
        ('e', 5),
        ('f', 6),
        ('g', 7),
        ('h', 8),
        ('i', 9),
        ('j', 10),
        ('k', 11),
        ('l', 12),
        ('m', 13),
        ('n', 14),
        ('o', 15),
        ('p', 16),
        ('q', 17),
        ('r', 18),
        ('s', 19),
        ('t', 20),
        ('u', 21),
        ('v', 22),
        ('w', 23),
        ('x', 24),
        ('y', 25),
        ('z', 26),
        ('A', 27),
        ('B', 28),
        ('C', 29),
        ('D', 30),
        ('E', 31),
        ('F', 32),
        ('G', 33),
        ('H', 34),
        ('I', 35),
        ('J', 36),
        ('K', 37),
        ('L', 38),
        ('M', 39),
        ('N', 40),
        ('O', 41),
        ('P', 42),
        ('Q', 43),
        ('R', 44),
        ('S', 45),
        ('T', 46),
        ('U', 47),
        ('V', 48),
        ('W', 49),
        ('X', 50),
        ('Y', 51),
        ('Z', 52),
    ]);

    let file = File::open("./input.txt").expect("File not found!");
    let reader = BufReader::new(file);

    let mut rucksack = Rucksack::new();

    for line in reader.lines() {
        let Ok(line) = line else {
            continue;
        };
        let line_len = line.len();
        let (a, b) = line.split_at(line_len / 2);
        rucksack.insert(a.chars().collect(), b.chars().collect())
    }

    println!("{}", rucksack.calc_group_prio_sum(&priority_map))
}
