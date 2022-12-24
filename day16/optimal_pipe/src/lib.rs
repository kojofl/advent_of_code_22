use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Valve {
    pub flow: i32,
    pub connections: Vec<String>,
}

pub type ValveManager = HashMap<String, Valve>;

pub fn parse_file(file_path: &str) -> ValveManager {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut manager = ValveManager::new();
    for line in reader.lines() {
        let Ok(line) = line else {
            continue;
        };
        let id = line[6..=7].to_owned();
        let delimeter_position = line.find(";").unwrap();
        let flow = line[23..delimeter_position].parse::<i32>().unwrap();
        let connections_str = line.split("valves").last().unwrap();
        let connections: Vec<String> = connections_str
            .split(",")
            .map(|c| c.trim().to_owned())
            .collect();
        let valve = Valve { flow, connections };
        manager.insert(id, valve);
    }
    manager
}
