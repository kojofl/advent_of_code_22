use std::{
    collections::VecDeque,
    fs::File,
    io::{BufReader, Read},
    usize,
};

#[derive(Clone, Copy)]
pub enum Rock {
    Horizontal([(usize, usize); 4]),
    Cross([(usize, usize); 5]),
    Corner([(usize, usize); 5]),
    Vertical([(usize, usize); 4]),
    Square([(usize, usize); 4]),
}

#[derive(Clone, Copy)]
pub enum AbstractRock {
    Horizontal,
    Cross,
    Corner,
    Vertical,
    Square,
}

static MAX_SIZE: usize = 1000;
static RESIZE_TARGET: usize = 100;

impl AbstractRock {
    // Vertical
    // ####
    // Cross
    // .#.
    // ###
    // .#.
    //
    // Corner
    // ..#
    // ..#
    // ###
    //
    // Horizontal
    // #
    // #
    // #
    // #
    //
    // Square
    // ##
    // ##
    fn into_rock<const S: usize>(self, room: &VecDeque<[bool; S]>) -> Rock {
        // Calc highest rock
        let highest = room.len()
            - room
                .iter()
                .rev()
                .enumerate()
                .find(|(_, row)| row.iter().any(|cell| *cell))
                .map(|r| r.0)
                .unwrap_or(S);
        match self {
            AbstractRock::Horizontal => Rock::Horizontal([
                (2, highest + 3),
                (3, highest + 3),
                (4, highest + 3),
                (5, highest + 3),
            ]),
            AbstractRock::Cross => Rock::Cross([
                (3, highest + 3),
                (2, highest + 4),
                (3, highest + 4),
                (4, highest + 4),
                (3, highest + 5),
            ]),
            AbstractRock::Corner => Rock::Corner([
                (2, highest + 3),
                (3, highest + 3),
                (4, highest + 3),
                (4, highest + 4),
                (4, highest + 5),
            ]),
            AbstractRock::Vertical => Rock::Vertical([
                (2, highest + 3),
                (2, highest + 4),
                (2, highest + 5),
                (2, highest + 6),
            ]),
            AbstractRock::Square => Rock::Square([
                (2, highest + 3),
                (3, highest + 3),
                (2, highest + 4),
                (3, highest + 4),
            ]),
        }
    }
}

#[derive(Clone, Copy)]
enum MoveSuccess {
    Moved,
    Rest,
}

impl Rock {
    fn inner_mut(&mut self) -> &mut [(usize, usize)] {
        match self {
            Rock::Horizontal(e) => &mut e[..],
            Rock::Cross(e) => &mut e[..],
            Rock::Corner(e) => &mut e[..],
            Rock::Vertical(e) => &mut e[..],
            Rock::Square(e) => &mut e[..],
        }
    }

    fn try_move<const S: usize>(
        &mut self,
        cave: &VecDeque<[bool; S]>,
        cmd: Command,
    ) -> Result<MoveSuccess, &'static str> {
        use MoveSuccess::*;
        let inner = self.inner_mut();
        match cmd {
            Command::Left => {
                for (x, y) in inner.iter() {
                    // Copy x since this should be a transaction and impossible to
                    // create wrong internal state of the rock
                    let mut x_clone = *x;
                    x_clone = x_clone.checked_sub(1).ok_or("Unable to move to left")?;
                    if cave[*y][x_clone] {
                        return Err("Cell occupied");
                    }
                }
                for (x, _) in inner.iter_mut() {
                    *x -= 1;
                }
                Ok(Moved)
            }
            Command::Right => {
                for (x, y) in inner.iter() {
                    // Copy x since this should be a transaction and impossible to
                    // create wrong internal state of the rock
                    let x_clone = *x + 1;
                    if x_clone >= S {
                        return Err("Unable to move to right");
                    }
                    if cave[*y][x_clone] {
                        return Err("Cell occupied");
                    }
                }
                for (x, _) in inner.iter_mut() {
                    *x += 1;
                }
                Ok(Moved)
            }
            Command::Down => {
                for (x, y) in inner.iter() {
                    // Copy y since this should be a transaction and impossible to
                    // create wrong internal state of the rock
                    let y_clone = *y;
                    if y_clone.checked_sub(1).is_none() {
                        return Ok(Rest);
                    }
                    if cave[y_clone - 1][*x] {
                        return Ok(Rest);
                    }
                }
                for (_, y) in inner.iter_mut() {
                    *y -= 1;
                }
                Ok(Moved)
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Command {
    Left,
    Right,
    Down,
}

impl TryFrom<u8> for Command {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'<' => Ok(Command::Left),
            b'>' => Ok(Command::Right),
            _ => Err("Failed to convert byte to Command"),
        }
    }
}

pub struct Cave<const S: usize> {
    rock_in_motion: Option<Rock>,
    commands: Option<Vec<Command>>,
    rock_order: Option<Vec<AbstractRock>>,
    room: VecDeque<[bool; S]>,
    rock_goal: usize,
    loss: usize,
}

impl<const S: usize> Cave<S> {
    pub fn new(commands: Vec<Command>, rock_order: Vec<AbstractRock>, rock_goal: usize) -> Self {
        Self {
            rock_in_motion: None,
            commands: Some(commands),
            rock_order: Some(rock_order),
            room: vec![[false; S]; 7].into(),
            rock_goal,
            loss: 0,
        }
    }

    pub fn start(mut self) -> usize {
        let mut infinite_rocks = self.rock_order.take().unwrap().into_iter().cycle();
        let mut infinite_cmds = self.commands.take().unwrap().into_iter().cycle();
        loop {
            if self.rock_in_motion.is_none() {
                let rock = infinite_rocks.next().unwrap();
                self.rock_in_motion = Some(rock.into_rock(&self.room));
            }
            self.calc_move(infinite_cmds.next().unwrap());
            self.calc_move(Command::Down);
            if self.rock_goal == 0 {
                break;
            }
            self.grow_cave();
            if self.room.len() > MAX_SIZE {
                self.loss += self.room.drain(0..MAX_SIZE - RESIZE_TARGET).len();
            }
        }
        self.print_cave();
        self.room.len()
            - self
                .room
                .iter()
                .rev()
                .enumerate()
                .find(|(_, row)| row.iter().any(|cell| *cell))
                .map(|r| r.0)
                .unwrap_or(S)
            + self.loss
    }

    fn calc_move(&mut self, m: Command) {
        let mut stone = self.rock_in_motion.take().unwrap();
        match stone.try_move(&self.room, m) {
            Ok(r) => match r {
                MoveSuccess::Rest => {
                    self.rock_goal -= 1;
                    for (x, y) in stone.inner_mut() {
                        self.room[*y][*x] = true;
                    }
                }
                _ => self.rock_in_motion = Some(stone),
            },
            Err(_e) => {
                self.rock_in_motion = Some(stone);
            }
        }
    }

    fn grow_cave(&mut self) {
        let highest = self.room.len()
            - self
                .room
                .iter()
                .rev()
                .enumerate()
                .find(|(_, row)| row.iter().any(|cell| *cell))
                .map(|r| r.0)
                .unwrap_or(S);
        for _ in 0..(7 - (self.room.len() - highest)) {
            self.room.push_back([false; S]);
        }
    }

    fn print_cave(&self) {
        for (y, r) in self.room.iter().rev().enumerate() {
            let y = self.room.len() - y;
            for (x, c) in r.iter().enumerate() {
                if *c {
                    print!("#");
                } else if let Some(mut rock) = self.rock_in_motion {
                    if rock.inner_mut().iter().any(|(a, b)| *b == y && *a == x) {
                        print!("@");
                    } else {
                        print!(".");
                    }
                } else {
                    print!(".");
                }
            }
            println!()
        }
    }
}

fn main() {
    use AbstractRock::*;

    let mut cmds = Vec::with_capacity(20);
    let cmd_file = File::open("input.txt").unwrap();
    let mut reader = BufReader::new(cmd_file);
    reader.read_to_end(&mut cmds).unwrap();
    let commands: Vec<Command> = cmds
        .into_iter()
        .map(TryInto::<Command>::try_into)
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .collect();
    let cave = Cave::<7>::new(
        commands,
        vec![Horizontal, Cross, Corner, Vertical, Square],
        1000000000000,
    );
    println!("{}", cave.start());
}

#[test]
fn test_sim() {
    use AbstractRock::*;

    let commands: Vec<Command> = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"
        .bytes()
        .map(TryInto::<Command>::try_into)
        .map(Result::unwrap)
        .collect();
    let cave = Cave::<7>::new(
        commands,
        vec![Horizontal, Cross, Corner, Vertical, Square],
        1,
    );
    let height = cave.start();
    println!("{height}")
}
