use std::{collections::VecDeque, fs::File, io::{BufReader, BufRead}};

struct Labyrith {
    inner: Vec<Vec<char>>,
    visited: Vec<Vec<bool>>,
    priority_que: VecDeque<Field>,
    start: (usize, usize),
    goal: (usize, usize),
}

struct Field {
    row: usize,
    col: usize,
    value: char,
    steps_from_start: i32,
}

impl Field {
    pub fn new(row: usize, col: usize, value: char, steps: i32) -> Self {
        Self {
            row,
            col,
            value,
            steps_from_start: steps,
        }
    }
}

impl Labyrith {
    pub fn new(lab: Vec<Vec<char>>, start: (usize, usize), goal: (usize, usize)) -> Self {
        Self {
            visited: vec![vec![false; lab[0].len()]; lab.len()],
            inner: lab,
            priority_que: VecDeque::new(),
            start,
            goal,
        }
    }

    pub fn start_pathfinding(&mut self) -> Option<i32> {
        let start = *self
            .inner
            .get(self.start.0)
            .unwrap()
            .get(self.start.1)
            .unwrap();
        let start_field = Field::new(self.start.0, self.start.1, start, 0);
        self.priority_que.push_back(start_field);
        while let Some(field) = self.priority_que.pop_front() {
            print!("\x1B[2J");
            print_matrix(&self.inner, &self.visited);
            let mut possible_next = self.check_surounding(field);
            while let Some(n) = possible_next.pop() {
                if n.row == self.goal.0 && n.col == self.goal.1 {
                    return Some(n.steps_from_start);
                }
                self.priority_que.push_back(n)
            }
        }
        None
    }

    fn check_surounding(&mut self, field_of_interest: Field) -> Vec<Field> {
        let mut v = Vec::new();
        // check up
        if field_of_interest.row != 0 {
            if let Some(up) = self.inner.get(field_of_interest.row - 1) {
                if !self.visited[field_of_interest.row - 1][field_of_interest.col] {
                    let c = *up.get(field_of_interest.col).unwrap();
                    if (field_of_interest.value as u8).abs_diff(c as u8) <= 1 {
                        v.push(Field::new(
                            field_of_interest.row - 1,
                            field_of_interest.col,
                            c,
                            field_of_interest.steps_from_start + 1,
                        ));
                        self.visited[field_of_interest.row - 1][field_of_interest.col] = true;
                    }
                }
            }
        }
        // check down
        if let Some(down) = self.inner.get(field_of_interest.row + 1) {
            if !self.visited[field_of_interest.row + 1][field_of_interest.col] {
                let c = *down.get(field_of_interest.col).unwrap();
                if (field_of_interest.value as u8).abs_diff(c as u8) <= 1 {
                    v.push(Field::new(
                        field_of_interest.row + 1,
                        field_of_interest.col,
                        c,
                        field_of_interest.steps_from_start + 1,
                    ));
                    self.visited[field_of_interest.row + 1][field_of_interest.col] = true;
                }
            }
        }

        // check left
        if field_of_interest.col != 0
            && !self.visited[field_of_interest.row][field_of_interest.col - 1]
        {
            if let Some(left) = self
                .inner
                .get(field_of_interest.row)
                .unwrap()
                .get(field_of_interest.col - 1)
            {
                if (field_of_interest.value as u8).abs_diff(*left as u8) <= 1 {
                    v.push(Field::new(
                        field_of_interest.row,
                        field_of_interest.col - 1,
                        *left,
                        field_of_interest.steps_from_start + 1,
                    ));
                    self.visited[field_of_interest.row][field_of_interest.col - 1] = true;
                }
            }
        }
        // check right
        if let Some(right) = self
            .inner
            .get(field_of_interest.row)
            .unwrap()
            .get(field_of_interest.col + 1)
        {
            if !self.visited[field_of_interest.row][field_of_interest.col + 1] {
                if (field_of_interest.value as u8).abs_diff(*right as u8) <= 1 {
                    v.push(Field::new(
                        field_of_interest.row,
                        field_of_interest.col + 1,
                        *right,
                        field_of_interest.steps_from_start + 1,
                    ));
                    self.visited[field_of_interest.row][field_of_interest.col + 1] = true;
                }
            }
        }

        v
    }
}

fn print_matrix(m: &Vec<Vec<char>>, v: &Vec<Vec<bool>>) {
    for (i, row) in m.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            match v[i][j] {
                true => print!("\x1b[93m{}\x1b[0m", c),
                false => print!("{}", *c),
            }
            
        }
        println!()
    }
}

fn main() {
    let (field, start, goal) = parse_file("./input.txt");
    let mut finder = Labyrith::new(field,  goal.unwrap(), start.unwrap());
    let result = finder.start_pathfinding();
    print!("{result:?}");
    
}

fn parse_file(filename: &str) -> (Vec<Vec<char>>, Option<(usize, usize)>, Option<(usize, usize)>) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut v: Vec<Vec<char>> = Vec::new();
    let mut start = None;
    let mut end = None;
    for (i, line) in reader.lines().enumerate() {
        let Ok(line ) = line else {
            continue;
        };
        let mut row = Vec::new();
        for (j, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    row.push('a');
                    start = Some((i, j))
                },
                'E' => {
                    row.push('z');
                    end = Some((i, j))
                },
                x => row.push(x)
             }
        }
        v.push(row);
    }
    (v, start, end)
}
