use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Field {
    Empty,
    Occupied(Occupied),
}

impl Default for Field {
    fn default() -> Self {
        Self::Empty
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Occupied {
    Sand,
    Rock,
}

#[derive(Debug, Clone)]
struct Cave {
    inner: Vec<Vec<Field>>,
    x_offset: usize,
}

impl Cave {
    pub fn new(x_max: usize, y_max: usize, x_offset: usize) -> Self {
        let mut c = Cave {
            inner: vec![vec![Field::default(); 100000]; y_max + 1],
            x_offset: 0,
        };
        c.inner.push(vec![Field::Occupied(Occupied::Rock); 100000]);
        c
    }

    pub fn occupy_range(&mut self, x_start: usize, y_start: usize, x_end: usize, y_end: usize) {
        let (x_start, x_end) = match x_start.cmp(&x_end) {
            std::cmp::Ordering::Greater => (x_end - self.x_offset, x_start - self.x_offset),
            _ => (x_start - self.x_offset, x_end - self.x_offset),
        };
        let (y_start, y_end) = match y_start.cmp(&y_end) {
            std::cmp::Ordering::Greater => (y_end, y_start),
            _ => (y_start, y_end),
        };
        println!("{x_start} - {x_end} {}", self.x_offset);
        for y_index in y_start..=y_end {
            self.inner.get_mut(y_index).unwrap().splice(
                x_start..=x_end,
                (x_start..=x_end)
                    .into_iter()
                    .map(|_| Field::Occupied(Occupied::Rock)),
            );
        }
    }

    pub fn start_sand_sim(&mut self, sand_source: usize) -> i32 {
        let mut iterations = 0;

        loop {
            let Some(new_sand_pos) =
                Cave::find_landing_pos(&self.inner, (0, sand_source - self.x_offset)) else {
                    panic!("help")
                };

            if new_sand_pos == (0, sand_source) {
                return iterations;
            }

            //println!("Setting Sand at: {new_sand_pos:?}");

            self.inner[new_sand_pos.0][new_sand_pos.1] = Field::Occupied(Occupied::Sand);
            iterations += 1
        }
    }

    fn find_landing_pos(
        cave: &Vec<Vec<Field>>,
        position: (usize, usize),
    ) -> Option<(usize, usize)> {
        let Some(down) = cave.get(position.0 + 1) else {
            return None
        };

        // Check down
        match down.get(position.1) {
            Some(field_state) => {
                if *field_state == Field::Empty {
                    return Cave::find_landing_pos(cave, (position.0 + 1, position.1));
                }
            }
            None => return None,
        }

        let Some(checked_left) = position.1.checked_sub(1) else {
            return None
        };

        // Check down left
        match down.get(checked_left) {
            Some(field_state) => {
                if *field_state == Field::Empty {
                    return Cave::find_landing_pos(cave, (position.0 + 1, checked_left));
                }
            }
            None => return None,
        }
        // Check down right
        match down.get(position.1 + 1) {
            Some(field_state) => {
                if *field_state == Field::Empty {
                    return Cave::find_landing_pos(cave, (position.0 + 1, position.1 + 1));
                }
            }
            None => return None,
        }
        return Some(position);
    }

    pub fn print(&self) {
        for el in &self.inner {
            for f in el {
                match f {
                    Field::Empty => print!("."),
                    Field::Occupied(t) => match t {
                        Occupied::Sand => print!("o"),
                        Occupied::Rock => print!("#"),
                    },
                }
            }
            println!()
        }
    }
}

fn parse_file(file_path: &str) -> Cave {
    let file_for_meta_inf = File::open(file_path).unwrap();
    let reader = BufReader::new(file_for_meta_inf);
    let mut min_x = usize::MAX;
    let mut max_x = 0;
    let mut max_y = 0;
    for l in reader.lines() {
        let Ok(line) = l else {
            continue;
        };
        let split = line.split("->");
        for pair in split {
            let mut it = pair.split(",");

            let x_con = it.next().unwrap().trim().parse::<usize>().unwrap();
            if min_x > x_con {
                min_x = x_con
            }
            if max_x < x_con {
                max_x = x_con
            }
            let y_con = it.next().unwrap().trim().parse::<usize>().unwrap();
            if max_y < y_con {
                max_y = y_con
            }
        }
    }

    let mut cave = Cave::new(max_x - min_x + 1, max_y + 1, min_x);

    print!("got here");
    //cave.print();

    let file_to_parse = File::open(file_path).unwrap();

    let reader = BufReader::new(file_to_parse);

    for l in reader.lines() {
        let Ok(line) = l else {
            continue;
        };
        let it: Vec<&str> = line.split("->").collect();
        for win in it.windows(2) {
            let mut a = win[0].split(",");
            let mut b = win[1].split(",");
            cave.occupy_range(
                a.next().unwrap().trim().parse().unwrap(),
                a.next().unwrap().trim().parse().unwrap(),
                b.next().unwrap().trim().parse().unwrap(),
                b.next().unwrap().trim().parse().unwrap(),
            )
        }
    }

    cave
}

fn main() {
    let mut cave = parse_file("./input.txt");
    let it = cave.start_sand_sim(500);
    println!("{it}");
    //cave.print()
}
