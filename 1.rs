use std::fs;
use std::env;

fn get_calibration_value(line: &str) -> u32 {
    let mut first_digit: Option<u32> = None;
    let mut last_digit: Option<u32> = None;
    println!("{}", line);
    for ch in line.chars() {
        if ch.is_digit(10) {
            if first_digit == None {
                first_digit = ch.to_digit(10);
            }
            last_digit = ch.to_digit(10);
        }
    }
    println!("{:?}{:?}", first_digit.expect(""), last_digit.expect(""));
    return first_digit.expect("missing first digit") * 10 + last_digit.expect("missing last digit")
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut sum = 0;
    for line in contents.lines() {
        sum += get_calibration_value(line);
    }
    println!("{}", sum);
}
