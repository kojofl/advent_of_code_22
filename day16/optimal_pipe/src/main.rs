use std::{
    clone,
    collections::HashMap,
    fs::File,
    hash::Hash,
    io::{BufRead, BufReader},
    sync::{
        atomic::{AtomicIsize, AtomicUsize, Ordering},
        Arc,
    },
    thread,
};

use optimal_pipe::{parse_file, Valve, ValveManager};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct State {
    current: Valve,
    open: Vec<Valve>,
    time: i32,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct State2 {
    current: Valve,
    to_find: Vec<Valve>,
    open: Vec<Valve>,
    time: i32,
}

static MAX_TIME: i32 = 26;

#[test]
fn test() {
    let m = parse_file("./input.txt");
    let to_find: Vec<Valve> = m.values().cloned().filter(|a| a.flow > 0).collect();
    let iterations = 2_i32.pow((to_find.len() - 1) as u32);
    println!("{iterations}");
    let start_a = m.get("AA").unwrap().clone();
    let mut mem = HashMap::new();

    let x = calc_optimal_pressure_only_turn_on_x(
        start_a.clone(),
        &m,
        to_find,
        &mut mem,
        1,
        Vec::new(),
        0,
    );

    print!("{x}")
}

static GLOBAL_THREAD_COUNT: AtomicUsize = AtomicUsize::new(0);

fn main() {
    let m = parse_file("./input.txt");

    let to_find: Vec<Valve> = m.values().cloned().filter(|a| a.flow > 0).collect();

    let max = Arc::new(AtomicIsize::new(0));
    let fin = 2_i32.pow((to_find.len() - 1) as u32);
    for i in 1..fin {
        println!("iteration {i} of {fin}");
        let c = m.clone();
        let start_a = m.get("AA").unwrap().clone();
        let mut set_a: Vec<Valve> = Vec::new();
        let mut bit_pos: i32 = 0;
        while set_a.len() != i.count_ones() as usize {
            let mask = i & (1 << bit_pos);
            if mask > 0 {
                set_a.push(to_find[bit_pos as usize].clone());
            }
            bit_pos = bit_pos + 1;
        }
        let set_b: Vec<Valve> = to_find
            .iter()
            .cloned()
            .filter(|v| !set_a.contains(v))
            .collect();
        while GLOBAL_THREAD_COUNT.load(Ordering::SeqCst) >= 8 {}
        GLOBAL_THREAD_COUNT.fetch_add(1, Ordering::SeqCst);
        let m = max.clone();
        thread::spawn(move || {
            let mem = HashMap::new();
            let x = calc_optimal_pressure_only_turn_on_x(
                start_a.clone(),
                &c,
                set_a,
                &mut mem.clone(),
                1,
                Vec::new(),
                0,
            );

            let y = calc_optimal_pressure_only_turn_on_x(
                start_a.clone(),
                &c,
                set_b,
                &mut mem.clone(),
                1,
                Vec::new(),
                0,
            );
            if m.load(Ordering::SeqCst) < (x + y).try_into().unwrap() {
                println!("Found new max: {}", x + y);
                m.store((x + y).try_into().unwrap(), Ordering::SeqCst)
            } else {
                println!("Proposed too low: {}", x + y)
            }
            GLOBAL_THREAD_COUNT.fetch_sub(1, Ordering::SeqCst);
        });
    }
    while GLOBAL_THREAD_COUNT.load(Ordering::SeqCst) != 0 {}
    let max = max.load(Ordering::SeqCst);
    println!("{max:?}")
    /*

        let max_me = calc_optimal_pressure_only_turn_on_x(
            start_a.clone(),
            &m,
            vec![
                m.get("JJ").unwrap().clone(),
                m.get("BB").unwrap().clone(),
                m.get("CC").unwrap().clone(),
            ],
            &mut mem.clone(),
            1,
            Vec::new(),
            0,
        );

        let max_el = calc_optimal_pressure_only_turn_on_x(
            start_a.clone(),
            &m,
            vec![
                m.get("DD").unwrap().clone(),
                m.get("HH").unwrap().clone(),
                m.get("EE").unwrap().clone(),
            ],
            &mut mem.clone(),
            1,
            Vec::new(),
            0,
        );
    */
}

fn calc_optimal_pressure_only_turn_on_x(
    current: Valve,
    all_valves: &ValveManager,
    to_find: Vec<Valve>,
    memory: &mut HashMap<State, i32>,
    time: i32,
    open: Vec<Valve>,
    acc_pressure: i32,
) -> i32 {
    if time == MAX_TIME {
        return acc_pressure;
    }

    if to_find.is_empty() {
        return acc_pressure + open.iter().fold(0, |acc, v| acc + v.flow) * (MAX_TIME - time);
    }

    let my_state = State {
        current: current.clone(),
        open,
        time,
    };

    if let Some(mem) = memory.get(&my_state) {
        //println!("memory found: {mem}");
        return *mem;
    }

    let best = current
        .connections
        .iter()
        .chain(["self".into()].iter())
        .map(|c| {
            if c == "self" {
                if current.flow > 0
                    && !my_state.open.contains(&current)
                    && to_find.contains(&current)
                {
                    //println!("found {current:?} in time: {time} current pressure: {acc_pressure}");
                    let mut new_open = my_state.open.clone();
                    new_open.push(current.clone());
                    let add = new_open.iter().fold(0, |acc, v| acc + v.flow);
                    let mut new_to_find = to_find.clone();
                    new_to_find.retain(|x| *x != current);
                    calc_optimal_pressure_only_turn_on_x(
                        current.clone(),
                        all_valves,
                        new_to_find,
                        memory,
                        time + 1,
                        new_open,
                        acc_pressure + add,
                    )
                } else {
                    0
                }
            } else {
                let new = all_valves.get(c).unwrap().clone();
                let add = my_state.open.iter().fold(0, |acc, v| acc + v.flow);
                calc_optimal_pressure_only_turn_on_x(
                    new,
                    all_valves,
                    to_find.clone(),
                    memory,
                    time + 1,
                    my_state.open.clone(),
                    acc_pressure + add,
                )
            }
        })
        .max()
        .unwrap();
    memory.insert(my_state.clone(), best);

    return best;
}

fn calc_optimal_pressure(
    current: Valve,
    all_valves: &ValveManager,
    memory: &mut HashMap<State, i32>,
    time: i32,
    open: Vec<Valve>,
    acc_pressure: i32,
) -> i32 {
    if time == MAX_TIME {
        return acc_pressure + open.iter().fold(0, |acc, v| acc + v.flow);
    }

    let my_state = State {
        current: current.clone(),
        open,
        time,
    };

    if let Some(mem) = memory.get(&my_state) {
        //println!("memory found: {mem}");
        return *mem;
    }

    let best = current
        .connections
        .iter()
        .chain(["self".into()].iter())
        .map(|c| {
            if c == "self" {
                if current.flow > 0 && !my_state.open.contains(&current) {
                    let add = my_state.open.iter().fold(0, |acc, v| acc + v.flow);
                    let mut new_open = my_state.open.clone();
                    new_open.push(current.clone());
                    calc_optimal_pressure(
                        current.clone(),
                        all_valves,
                        memory,
                        time + 1,
                        new_open,
                        acc_pressure + add,
                    )
                } else {
                    0
                }
            } else {
                let new = all_valves.get(c).unwrap().clone();
                let add = my_state.open.iter().fold(0, |acc, v| acc + v.flow);
                calc_optimal_pressure(
                    new,
                    all_valves,
                    memory,
                    time + 1,
                    my_state.open.clone(),
                    acc_pressure + add,
                )
            }
        })
        .max()
        .unwrap();

    memory.insert(my_state.clone(), best);

    return best;
}
