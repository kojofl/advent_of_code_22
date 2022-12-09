use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::Index,
};

fn main() {
    let file = File::open("./input.txt").expect("File not found");
    let reader = BufReader::new(file);

    let mut matrix: Vec<Vec<u32>> = Vec::new();

    for line in reader.lines() {
        let Ok(line) = line else {
            continue;
        };
        matrix.push(
            line.chars()
                .map(|c| char::to_digit(c, 10).unwrap())
                .collect(),
        )
    }

    let max_scenic_score = calculate_max_scenic_score(&matrix);

    println!("{max_scenic_score}")
}

fn calculate_max_scenic_score(height_matrix: &Vec<Vec<u32>>) -> usize {
    let width = height_matrix[0].len() - 1;
    let height = height_matrix.len() - 1;

    let mut max_score = 0;

    for (i, row) in height_matrix.iter().enumerate() {
        for (j, &el) in row.iter().enumerate() {
            if i == 0 || i == height || j == 0 || j == width {
                continue;
            }

            let view_left = height_matrix[i][..j]
                .iter()
                .rev()
                .enumerate()
                .find(|&(_, e)| *e >= el)
                .map(|(i, r)| i + 1)
                .unwrap_or(height_matrix[i][..j].len());

            let view_right = height_matrix[i][j + 1..]
                .iter()
                .enumerate()
                .find(|&(_, e)| *e >= el)
                .map(|(i, r)| i + 1)
                .unwrap_or(height_matrix[i][j + 1..].len());

            let mut i_up = i - 1;
            let view_up: usize = {
                let mut view = 1;

                while i_up != 0 && height_matrix[i_up][j] < el {
                    view += 1;
                    i_up -= 1;
                }
                view
            };

            let mut i_down = i + 1;
            let view_down: usize = {
                let mut view = 1;

                while i_down != height && height_matrix[i_down][j] < el {
                    view += 1;
                    i_down += 1;
                }
                view
            };

            max_score = max_score.max(view_down * view_left * view_right * view_up)
        }
    }
    max_score
}

fn calculate_visibility(height_matrix: &Vec<Vec<u32>>) -> Vec<Vec<bool>> {
    let width = height_matrix[0].len() - 1;
    let height = height_matrix.len() - 1;
    let mut bool_matrix = vec![vec![true; width + 1]; height + 1];

    for (i, row) in height_matrix.iter().enumerate() {
        for (j, &el) in row.iter().enumerate() {
            if i == 0 || i == height || j == 0 || j == width {
                continue;
            }

            let max_left: u32 = *height_matrix[i][..j].iter().max().unwrap();

            if el > max_left {
                continue;
            }
            let max_right: u32 = *height_matrix[i][j + 1..].iter().max().unwrap();
            if el > max_right {
                continue;
            }
            let mut i_up = i - 1;
            let max_up: u32 = {
                let mut max = height_matrix[i_up][j];
                while i_up != 0 {
                    i_up -= 1;
                    max = max.max(height_matrix[i_up][j]);
                }
                max
            };
            if el > max_up {
                continue;
            }
            let mut i_down = i + 1;
            let max_down: u32 = {
                let mut max = height_matrix[i_down][j];
                while i_down != height {
                    i_down += 1;
                    max = max.max(height_matrix[i_down][j]);
                }
                max
            };
            if el > max_down {
                continue;
            }
            bool_matrix[i][j] = false
        }
    }
    bool_matrix
}
