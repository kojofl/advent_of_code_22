use std::{
    cell::RefCell,
    cmp::Reverse,
    collections::BinaryHeap,
    fs::File,
    io::{BufRead, BufReader},
    option,
    rc::Rc,
};

use disc_space::*;

fn main() {
    let file = File::open("./input.txt").expect("File not found!");

    let reader = BufReader::new(file);

    let mut current_dir = Rc::new(RefCell::new(Dict::new("/".into())));

    for line in reader.lines() {
        let Ok(line) = line else {
            continue;
        };
        let line_input: Vec<&str> = line.split_ascii_whitespace().collect();
        if line_input[0] == "$" {
            match line_input[1] {
                "cd" => match line_input[2] {
                    "/" => continue,
                    ".." => current_dir = Dict::cd(current_dir, ChangeCommand::MoveOut).unwrap(),
                    x => {
                        current_dir =
                            Dict::cd(current_dir, ChangeCommand::MoveIn(x.into())).unwrap()
                    }
                },
                "ls" => {
                    continue;
                }
                _ => panic!("No such command supported"),
            }
        } else {
            match line_input[0] {
                "dir" => {
                    current_dir
                        .as_ref()
                        .borrow_mut()
                        .content
                        .push(DiscElement::Dict(Rc::new(RefCell::new(Dict::with_parent(
                            line_input[1].into(),
                            Rc::clone(&current_dir),
                        )))));
                }
                size => current_dir
                    .as_ref()
                    .borrow_mut()
                    .content
                    .push(DiscElement::File(disc_space::File::new(
                        line_input[1],
                        size.parse().unwrap(),
                    ))),
            }
        }
    }

    let home = Dict::cd(current_dir, ChangeCommand::Home).unwrap();

    home.as_ref().borrow_mut().calculate_size();

    let mut total_size = 0;

    sum_dict_with_less_than(Rc::clone(&home), &mut total_size, 100001);

    println!("{}", total_size);

    let space_to_clear = home.as_ref().borrow().size.unwrap() + 30000000 - 70000000;

    let mut del_options = BinaryHeap::new();

    find_suitable_deletes(Rc::clone(&home), &mut del_options, space_to_clear);

    println!("{}", del_options.pop().unwrap().0)
}

fn find_suitable_deletes(
    dict: Rc<RefCell<Dict>>,
    options: &mut BinaryHeap<Reverse<u64>>,
    space_to_clear: u64,
) {
    if let Some(size) = dict.as_ref().borrow().size {
        if size >= space_to_clear {
            options.push(Reverse(size));
        }
    }
    for element in &dict.as_ref().borrow().content {
        match element {
            DiscElement::Dict(d) => find_suitable_deletes(Rc::clone(d), options, space_to_clear),
            DiscElement::File(_) => continue,
        }
    }
}

fn sum_dict_with_less_than(dict: Rc<RefCell<Dict>>, total_size: &mut u64, boundry: u64) {
    if let Some(size) = dict.as_ref().borrow().size {
        if size < boundry {
            *total_size += size
        }
    }
    for element in &dict.as_ref().borrow().content {
        match element {
            DiscElement::Dict(d) => sum_dict_with_less_than(Rc::clone(d), total_size, boundry),
            DiscElement::File(_) => continue,
        }
    }
}
