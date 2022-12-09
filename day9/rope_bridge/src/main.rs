use std::{
    cell::RefCell,
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    rc::Rc,
    str::FromStr,
};
#[derive(Debug, Clone, Copy)]
enum TailMove {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
    Nothing,
}
#[derive(Debug, Clone, Copy)]
enum HeadMove {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for HeadMove {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(HeadMove::Up),
            "D" => Ok(HeadMove::Down),
            "L" => Ok(HeadMove::Left),
            "R" => Ok(HeadMove::Right),
            _ => Err("Unknown Command"),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Tail {
    inner: Point,
    next: Option<Rc<RefCell<Tail>>>,
}

#[derive(Debug, Clone, Default)]
struct Rope {
    head: Point,
    tail: Tail,
}

impl Tail {
    pub fn make_move(&mut self, tail_move: TailMove) {
        match tail_move {
            TailMove::Up => {
                self.inner.y += 1;
            }
            TailMove::Down => {
                self.inner.y -= 1;
            }
            TailMove::Left => {
                self.inner.x -= 1;
            }
            TailMove::Right => {
                self.inner.x += 1;
            }
            TailMove::UpLeft => {
                self.inner.y += 1;
                self.inner.x -= 1;
            }
            TailMove::UpRight => {
                self.inner.y += 1;
                self.inner.x += 1;
            }
            TailMove::DownLeft => {
                self.inner.y -= 1;
                self.inner.x -= 1;
            }
            TailMove::DownRight => {
                self.inner.y -= 1;
                self.inner.x += 1;
            }
            TailMove::Nothing => {}
        }
        if let Some(next) = &self.next {
            let next_response = Rope::compute_tail_response(self.inner, next.borrow().inner);
            (*next).borrow_mut().make_move(next_response);
        }
    }

    fn get_last(&self) -> Point {
        if let Some(next) = &self.next {
            next.as_ref().borrow().get_last()
        } else {
            self.inner
        }
    }

    fn add_rope(&mut self) {
        if let Some(next) = &self.next {
            (*next).borrow_mut().add_rope()
        } else {
            self.next = Some(Rc::new(RefCell::new(Tail::default())))
        }
    }
}

impl Rope {
    pub fn make_move(&mut self, m: HeadMove) {
        match m {
            HeadMove::Up => {
                self.head.y += 1;
            }
            HeadMove::Down => {
                self.head.y -= 1;
            }
            HeadMove::Left => {
                self.head.x -= 1;
            }
            HeadMove::Right => {
                self.head.x += 1;
            }
        }
        let tail_move = Rope::compute_tail_response(self.head, self.tail.inner);
        self.tail.make_move(tail_move);
    }
    fn compute_tail_response(head: Point, tail: Point) -> TailMove {
        if head.y.abs_diff(tail.y) < 2 && head.x.abs_diff(tail.x) < 2 {
            return TailMove::Nothing;
        }

        if head.y == tail.y {
            if head.x > tail.x {
                return TailMove::Right;
            } else {
                return TailMove::Left;
            }
        }

        if head.x == tail.x {
            if head.y > tail.y {
                return TailMove::Up;
            } else {
                return TailMove::Down;
            }
        }
        match (head.y - tail.y, head.x - tail.x) {
            (y_d, x_d)
                if (y_d == 2 && x_d == 1) || (y_d == 1 && x_d == 2) || (y_d == 2 && x_d == 2) =>
            {
                TailMove::UpRight
            }
            (y_d, x_d)
                if (y_d == -2 && x_d == 1)
                    || (y_d == -1 && x_d == 2)
                    || (y_d == -2 && x_d == 2) =>
            {
                TailMove::DownRight
            }
            (y_d, x_d)
                if (y_d == -2 && x_d == -1)
                    || (y_d == -1 && x_d == -2)
                    || (y_d == -2 && x_d == -2) =>
            {
                TailMove::DownLeft
            }
            (y_d, x_d)
                if (y_d == 2 && x_d == -1)
                    || (y_d == 1 && x_d == -2)
                    || (y_d == 2 && x_d == -2) =>
            {
                TailMove::UpLeft
            }
            _ => panic!("Corrupted Rope State"),
        }
    }

    pub fn get_tail_position(&self) -> Point {
        self.tail.get_last()
    }

    pub fn add_rope(&mut self) {
        self.tail.add_rope();
    }
}

fn main() {
    let mut rope = Rope::default();

    for _ in 0..8 {
        rope.add_rope()
    }

    let file = File::open("./input.txt").expect("File not found!");

    let reader = BufReader::new(file);

    let mut rope_states: HashSet<Point> = HashSet::new();

    for lines in reader.lines() {
        let Ok(line) = lines else {
            continue;
        };
        let mut comand = line.split_ascii_whitespace();

        let m = comand.next().unwrap().parse::<HeadMove>().unwrap();

        let repetitions = comand.next().unwrap().parse::<usize>().unwrap();

        for _ in 0..repetitions {
            rope.make_move(m);
            rope_states.insert(rope.get_tail_position());
        }
    }
    println!("{}", rope_states.len())
}
