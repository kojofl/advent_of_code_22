use std::{
    cell::{RefCell, RefMut},
    collections::{HashSet, VecDeque},
};

struct Monkey {
    items: VecDeque<u128>,
    operation: Box<dyn Fn(u128) -> u128>,
    test: u128,
    pos: usize,
    neg: usize,
    inspected_count: u128,
}

impl Monkey {
    pub fn new(
        items: VecDeque<u128>,
        operation: Box<dyn Fn(u128) -> u128>,
        test: u128,
        pos: usize,
        neg: usize,
        inspected_count: u128,
    ) -> Self {
        Self {
            items,
            operation,
            test,
            pos,
            neg,
            inspected_count,
        }
    }
}

struct MonkeyZoo(Vec<RefCell<Monkey>>);

impl MonkeyZoo {
    pub fn play_round(&mut self) {
        // Rounds
        for i in 0..self.0.len() {
            let player = self.0.get(i).unwrap();
            let mut pl = player.borrow_mut();
            while let Some(inspect) = pl.items.pop_front() {
                pl.inspected_count += 1;
                let afte_op = (*pl.operation)(inspect) % 9699690;

                if (afte_op % pl.test) == 0 {
                    self.0
                        .get(pl.pos)
                        .unwrap()
                        .borrow_mut()
                        .items
                        .push_back(afte_op)
                } else {
                    self.0
                        .get(pl.neg)
                        .unwrap()
                        .borrow_mut()
                        .items
                        .push_back(afte_op)
                }
            }
        }
    }
}

fn main() {
    let monkey_0 = Monkey::new(
        VecDeque::from([50, 70, 89, 75, 66, 66]),
        Box::new(|x| x * 5),
        2,
        2,
        1,
        0,
    );
    let monkey_1 = Monkey::new(VecDeque::from([85]), Box::new(|x| x * x), 7, 3, 6, 0);
    let monkey_2 = Monkey::new(
        VecDeque::from([66, 51, 71, 76, 58, 55, 58, 60]),
        Box::new(|x| x + 1),
        13,
        1,
        3,
        0,
    );
    let monkey_3 = Monkey::new(
        VecDeque::from([79, 52, 55, 51]),
        Box::new(|x| x + 6),
        3,
        6,
        4,
        0,
    );
    let monkey_4 = Monkey::new(VecDeque::from([69, 92]), Box::new(|x| x * 17), 19, 7, 5, 0);
    let monkey_5 = Monkey::new(
        VecDeque::from([71, 76, 73, 98, 67, 79, 99]),
        Box::new(|x| x + 8),
        5,
        0,
        2,
        0,
    );
    let monkey_6 = Monkey::new(
        VecDeque::from([82, 76, 69, 69, 57]),
        Box::new(|x| x + 7),
        11,
        7,
        4,
        0,
    );
    let monkey_7 = Monkey::new(
        VecDeque::from([65, 79, 86]),
        Box::new(|x| x + 5),
        17,
        5,
        0,
        0,
    );

    let mut monkeys = MonkeyZoo(vec![
        RefCell::new(monkey_0),
        RefCell::new(monkey_1),
        RefCell::new(monkey_2),
        RefCell::new(monkey_3),
        RefCell::new(monkey_4),
        RefCell::new(monkey_5),
        RefCell::new(monkey_6),
        RefCell::new(monkey_7),
    ]);

    let super_mod = monkeys.0.iter().fold(1, |sum, m| sum * m.borrow().test);

    println!("{}", super_mod);

    for _ in 0..10000 {
        monkeys.play_round()
    }

    for x in monkeys.0 {
        println!("{}", x.borrow().inspected_count)
    }
}
