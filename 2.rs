use std::env;
use std::fs;

struct Sample {
    red: u32,
    green: u32,
    blue: u32,
}

struct Game {
    id: u32,
    samples: Vec<Sample>,
}

fn parse_game(line: &str) -> Game {
    let mut split = line.split(": ");
    let id = split
        .next()
        .expect("expected game prefix")
        .split(" ")
        .nth(1)
        .expect("expected game number")
        .parse()
        .expect("expected integer");

    let mut samples: Vec<Sample> = Vec::new();
    let line = split.next().expect("expected :");
    for sample in line.split("; ") {
        for cube in sample.split(", ") {
            let mut split = cube.split(" ");
            let num: u32 = split.next().expect("expected field").parse().expect("expected number");
            let color = split.next().expect("expected field");
            let sample = Sample {
                red: if color == "red" {num} else {0},
                green: if color == "green" {num} else {0},
                blue: if color == "blue" {num} else {0},
            };
            samples.push(sample);
        }
    }
    Game {
        id: id,
        samples: samples,
    }
}

fn valid_game(game: &Game) -> bool {
    for sample in &game.samples {
        if sample.red > 12 || sample.green > 13 || sample.blue > 14 {
            return false;
        }
    }
    true
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut sum = 0;
    for line in contents.lines() {
        let game = parse_game(line);
        if valid_game(&game) {
            sum += game.id;
        }
    }
    println!("{}", sum);
}
