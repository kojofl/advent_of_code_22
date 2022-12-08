use rock_paper_scisors::{Controler, Decisions, DesiredOutcome, Game, GamePoints};
use std::{
    collections::HashMap,
    fs::File,
    hash::Hash,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("./input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut game_flow = Vec::new();
    for line in reader.lines() {
        let Ok(line) = line else {
            continue;
        };
        let mut c = line
            .split(" ")
            .map(|s| {
                let b = s.as_bytes()[0];
                char::from_u32(b as u32).unwrap()
            })
            .take(2);

        let move_a = c.next().expect("Player_a has to make a move");
        let move_b = c.next().expect("Player_b has to make a move");
        game_flow.push((move_a, move_b))
    }
    let player_a_map = HashMap::from([
        ('A', Decisions::Rock),
        ('B', Decisions::Paper),
        ('C', Decisions::Scisors),
    ]);
    let player_b_map = HashMap::from([
        ('X', Controler::Decision(Decisions::Rock)),
        ('Y', Controler::Decision(Decisions::Paper)),
        ('Z', Controler::Decision(Decisions::Scisors)),
    ]);

    let mut game = Game::new(
        player_a_map,
        player_b_map,
        game_flow,
        GamePoints {
            win: 6,
            draw: 3,
            loss: 0,
        },
    );

    let results = game.start().unwrap();
    println!("{results:?}");

    let player_b_desires = HashMap::from([
        ('X', Controler::DesiredOutcome(DesiredOutcome::Loss)),
        ('Y', Controler::DesiredOutcome(DesiredOutcome::Draw)),
        ('Z', Controler::DesiredOutcome(DesiredOutcome::Win)),
    ]);

    game.player_b_map = player_b_desires;

    let new_results = game.start().unwrap();

    println!("{new_results:?}");
}
