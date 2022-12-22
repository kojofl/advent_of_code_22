use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point((i32, i32));

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self((x, y))
    }

    fn calculate_coverage_of_line(&self, line: i32, range: i32) -> (i32, i32) {
        let (line_with_coverage_zero, add) = if line >= self.0 .1 {
            if self.0 .1 + range < line {
                return (0, 0);
            }
            (self.0 .1 + range, false)
        } else {
            if self.0 .1 - range > line {
                return (0, 0);
            }
            (self.0 .1 - range, true)
        };

        let mut current_line = line_with_coverage_zero;
        let mut current_coverage = 1;

        while current_line != line {
            match add {
                true => {
                    current_line += 1;
                    current_coverage += 2;
                }
                false => {
                    current_line -= 1;
                    current_coverage += 2;
                }
            }
        }

        return (
            self.0 .0 - current_coverage / 2,
            self.0 .0 + current_coverage / 2,
        );
    }
}

fn main() {
    let mut point_vec: Vec<(Point, Point)> = Vec::new();

    let file = File::open("./input.txt").unwrap();

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let Ok(line) = line else {
            continue;
        };

        let mut split = line.split(":");

        let mut sensor = split.next().unwrap().split(",");

        let s_x = sensor
            .next()
            .unwrap()
            .split("=")
            .last()
            .unwrap()
            .parse::<i32>()
            .unwrap();

        let s_y = sensor
            .next()
            .unwrap()
            .split("=")
            .last()
            .unwrap()
            .parse::<i32>()
            .unwrap();

        let mut beacon = split.next().unwrap().split(",");

        let b_x = beacon
            .next()
            .unwrap()
            .split("=")
            .last()
            .unwrap()
            .parse::<i32>()
            .unwrap();

        let b_y = beacon
            .next()
            .unwrap()
            .split("=")
            .last()
            .unwrap()
            .parse::<i32>()
            .unwrap();

        point_vec.push((Point::new(s_x, s_y), Point::new(b_x, b_y)));
    }

    find_beacon(&point_vec);
}

fn find_beacon(point_vec: &Vec<(Point, Point)>) {
    let mut coverage_map: HashMap<i32, Vec<(i32, i32)>> = HashMap::with_capacity(4000000);
    for (sensor, beacon) in point_vec {
        let range = manhatten_dist(*sensor, *beacon);

        let mut line = sensor.0 .1 - range;

        let mut current_coverage = 1;

        while line < 0 {
            line += 1;
            if line <= sensor.0 .0 {
                current_coverage += 2
            } else {
                current_coverage -= 2
            }
        }

        while current_coverage > 0 {
            coverage_map
                .entry(line)
                .and_modify(|el| {
                    el.push((
                        sensor.0 .0 - current_coverage / 2,
                        sensor.0 .0 + current_coverage / 2,
                    ));

                    el.sort_by(|a, b| a.0.cmp(&b.0));

                    let tmp = merge_ranges(el);

                    *el = tmp;
                })
                .or_insert(vec![(
                    sensor.0 .0 - current_coverage / 2,
                    sensor.0 .0 + current_coverage / 2,
                )]);
            line += 1;
            if line <= sensor.0 .1 {
                current_coverage += 2;
            } else {
                current_coverage -= 2;
            }
        }
    }

    coverage_map
        .iter()
        .filter(|(el, v)| v.len() != 1 && **el <= 4000000 && **el >= 0)
        .for_each(|(el, v)| println!("{el}: {v:?}"))
}

fn calc_coverage_of_line(point_vec: &Vec<(Point, Point)>, line: i32) -> usize {
    let mut covered_ranges_in_line: Vec<(i32, i32)> = point_vec
        .iter()
        .copied()
        .map(|(sensor, beacon)| {
            let manhatten = manhatten_dist(sensor, beacon);
            sensor.calculate_coverage_of_line(line, manhatten)
        })
        .filter(|coverage| coverage.0 != 0 && coverage.1 != 0)
        .collect();

    covered_ranges_in_line.sort_by(|a, b| a.0.cmp(&b.0));

    let merged = merge_ranges(&covered_ranges_in_line);

    println!("{merged:?}");

    let mut s_b_in_line = HashSet::new();

    for (s, b) in point_vec {
        if s.0 .1 == line {
            s_b_in_line.insert(s);
        }
        if b.0 .1 == line {
            s_b_in_line.insert(b);
        }
    }
    let c = merged
        .iter()
        .fold(0, |acc, &range| acc + (range.0..=range.1).count());

    c - s_b_in_line.len()
}

fn merge_ranges(range_vec: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut merged_vec = Vec::new();
    let mut merge_item: Option<(i32, i32)> = None;

    for r in range_vec {
        match merge_item {
            Some(range) => {
                if range.1 >= r.0 || range.1.abs_diff(r.0) <= 1 {
                    if range.1 < r.1 {
                        merge_item = Some((range.0, r.1));
                    }
                } else {
                    merged_vec.push(merge_item.take().unwrap());
                    merge_item = Some(*r)
                }
            }
            None => merge_item = Some(*r),
        }
    }

    if merge_item.is_some() {
        merged_vec.push(merge_item.take().unwrap())
    }

    merged_vec
}

// calc the Manhatten distance of two points
fn manhatten_dist(a: Point, b: Point) -> i32 {
    (a.0 .0 - b.0 .0).abs() + (a.0 .1 - b.0 .1).abs()
}
