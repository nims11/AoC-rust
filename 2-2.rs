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

fn power(game: &Game) -> u32 {
    let mut red: Option<u32> = None;
    let mut green: Option<u32> = None;
    let mut blue: Option<u32> = None;
    for sample in &game.samples {
        if !red.is_some_and(|x| x > sample.red) {
            red = Some(sample.red);
        }
        if !green.is_some_and(|x| x > sample.green) {
            green = Some(sample.green);
        }
        if !blue.is_some_and(|x| x > sample.blue) {
            blue = Some(sample.blue);
        }
    }
    return red.expect("err") * green.expect("err") * blue.expect("err");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut sum = 0;
    for line in contents.lines() {
        let game = parse_game(line);
        sum += power(&game);
    }
    println!("{}", sum);
}
