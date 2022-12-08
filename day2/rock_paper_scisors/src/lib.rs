use std::{
    cmp::Ordering::{self, Equal, Greater, Less},
    collections::HashMap,
    error::Error,
    fmt,
};

#[derive(Debug)]
pub enum GameError {
    MoveError,
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameError::MoveError => {
                write!(f, "Error occured trying to parse move to game Decision!")
            }
        }
    }
}

impl Error for GameError {}

#[derive(Eq, PartialEq, PartialOrd, Clone, Copy, Debug)]
pub enum Decisions {
    Rock = 1,
    Paper = 2,
    Scisors = 3,
}

#[derive(Eq, PartialEq, PartialOrd, Clone, Copy, Debug)]
pub enum DesiredOutcome {
    Win,
    Loss,
    Draw,
}

#[derive(Eq, PartialEq, PartialOrd, Clone, Copy, Debug)]
pub enum Controler {
    Decision(Decisions),
    DesiredOutcome(DesiredOutcome),
}

impl Ord for Decisions {
    fn cmp(&self, other: &Self) -> Ordering {
        match *self {
            Decisions::Rock => match other {
                Decisions::Rock => Equal,
                Decisions::Paper => Less,
                Decisions::Scisors => Greater,
            },
            Decisions::Paper => match other {
                Decisions::Rock => Greater,
                Decisions::Paper => Equal,
                Decisions::Scisors => Less,
            },
            Decisions::Scisors => match other {
                Decisions::Rock => Less,
                Decisions::Paper => Greater,
                Decisions::Scisors => Equal,
            },
        }
    }
}

pub struct GamePoints {
    pub win: i32,
    pub draw: i32,
    pub loss: i32,
}

pub struct Game {
    player_a_map: HashMap<char, Decisions>,
    pub player_b_map: HashMap<char, Controler>,
    game_flow: Vec<(char, char)>,
    points: GamePoints,
}

impl Game {
    pub fn new(
        player_a_map: HashMap<char, Decisions>,
        player_b_map: HashMap<char, Controler>,
        game_flow: Vec<(char, char)>,
        points: GamePoints,
    ) -> Self {
        Self {
            player_a_map,
            player_b_map,
            game_flow,
            points,
        }
    }

    pub fn start(&self) -> Result<(i32, i32), GameError> {
        let mut points_a = 0;
        let mut points_b = 0;
        for (move_a, move_b) in self.game_flow.iter() {
            let Some(&decision_a) = self.player_a_map.get(move_a) else {
                return Err(GameError::MoveError);
            };
            let Some(&controler) = self.player_b_map.get(move_b) else {
                return Err(GameError::MoveError);
            };

            let decision_b = match controler {
                Controler::Decision(d) => d,
                Controler::DesiredOutcome(outcome) => match outcome {
                    DesiredOutcome::Win => match decision_a {
                        Decisions::Rock => Decisions::Paper,
                        Decisions::Paper => Decisions::Scisors,
                        Decisions::Scisors => Decisions::Rock,
                    },
                    DesiredOutcome::Loss => match decision_a {
                        Decisions::Rock => Decisions::Scisors,
                        Decisions::Paper => Decisions::Rock,
                        Decisions::Scisors => Decisions::Paper,
                    },
                    DesiredOutcome::Draw => decision_a,
                },
            };

            points_a += decision_a as i32;
            points_b += decision_b as i32;
            match decision_a.cmp(&decision_b) {
                Less => {
                    points_a += self.points.loss;
                    points_b += self.points.win;
                }
                Equal => {
                    points_a += self.points.draw;
                    points_b += self.points.draw;
                }
                Greater => {
                    points_a += self.points.win;
                    points_b += self.points.loss;
                }
            }
        }

        Ok((points_a, points_b))
    }
}
