use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone, Copy, Debug)]
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

    let file = File::open("./test.txt").unwrap();

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

    let mut covered_ranges_in_line: Vec<(i32, i32)> = point_vec
        .iter()
        .map(|(sensor, beacon)| {
            let manhatten = manhatten_dist(*sensor, *beacon);
            sensor.calculate_coverage_of_line(10, manhatten)
        })
        .filter(|coverage| coverage.0 != 0 && coverage.1 != 0)
        .collect();

    covered_ranges_in_line.sort_by(|a, b| a.0.cmp(&b.0));

    println!("{covered_ranges_in_line:?}");

    let merged = merge_ranges(&covered_ranges_in_line);

    let correction = point_vec.iter().fold(0, |acc, &el| {
        println!("{el:?}");
        if el.1 .0 .1 == 10 || el.0 .0 .1 == 10 {
            acc + 1
        } else {
            acc
        }
    });

    println!("{correction}");

    println!("{merged:?}")
}

fn merge_ranges(range_vec: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut merged_vec = Vec::new();
    let mut merge_item: Option<(i32, i32)> = None;

    for r in range_vec {
        match merge_item {
            Some(range) => {
                if range.1 >= r.0 {
                    if range.1 < r.1 {
                        merge_item = Some((range.0, r.1));
                    }
                } else {
                    merged_vec.push(merge_item.take().unwrap())
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
