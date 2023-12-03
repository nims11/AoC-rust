use std::fs;
use std::env;
use std::collections::HashMap;

fn get_calibration_value(line: &str) -> i32 {
    let mut digit_strings = HashMap::new();
    for i in 0..=9 {
        digit_strings.insert(i.to_string(), i);
    }
    digit_strings.insert("zero".to_string(), 0);
    digit_strings.insert("one".to_string(), 1);
    digit_strings.insert("two".to_string(), 2);
    digit_strings.insert("three".to_string(), 3);
    digit_strings.insert("four".to_string(), 4);
    digit_strings.insert("five".to_string(), 5);
    digit_strings.insert("six".to_string(), 6);
    digit_strings.insert("seven".to_string(), 7);
    digit_strings.insert("eight".to_string(), 8);
    digit_strings.insert("nine".to_string(), 9);
    let mut first_digit: Option<i32> = None;
    let mut last_digit: Option<i32> = None;

    let mut first_digit_idx = line.len();
    let mut last_digit_idx = 0;

    for (digit_str, digit) in digit_strings.into_iter() {
        match line.find(&digit_str) {
            Some(idx) => {
                if idx < first_digit_idx {
                    first_digit = Some(digit);
                    first_digit_idx = idx;
                }
            },
            None => {}
        }
        match line.rfind(&digit_str) {
            Some(idx) => {
                if idx >= last_digit_idx {
                    last_digit = Some(digit);
                    last_digit_idx = idx;
                }
            },
            None => {}
        }
    }
    first_digit.expect("first digit not found") * 10 + last_digit.expect("last digit not found")
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
