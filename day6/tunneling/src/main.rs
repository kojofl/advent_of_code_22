use std::{
    fs::File,
    io::{BufReader, Read},
};

fn main() {
    let input = File::open("./input.txt").expect("Cannot find file");

    let mut reader = BufReader::new(input);

    let mut buffer: String = String::new();

    reader.read_to_string(&mut buffer).unwrap();

    let mut result = 14;

    for window in buffer.as_bytes().windows(14) {
        if (1..window.len()).any(|i| window[i..].contains(&window[i - 1])) {
            result += 1;
            continue;
        }
        println!("{result}");
        break;
    }
}
