use std::cmp;
use std::env;
use std::fs;
use std::collections::HashMap;

struct Number {
    val: u32,
    r: u32,
    c_begin: u32,
    c_end: u32,
}

fn is_symbol(c: char) -> bool {
    c == '*'
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents: Vec<&str> = contents.lines().collect();
    let mut numbers: Vec<Number> = Vec::new();

    for (row, line) in contents.iter().enumerate() {
        let mut cur: Option<u32> = None;
        let mut c_begin: u32 = 0;
        for (col, ch) in line.chars().enumerate() {
            if ch.is_digit(10) {
                let ch = ch.to_digit(10).expect("expected digit");
                cur = match cur {
                    None => {c_begin = col as u32; Some(ch)},
                    Some(cur_val) => {Some(cur_val * 10 + ch)},
                }
            } else {
                match cur {
                    None => {},
                    Some(val) => {
                        numbers.push(Number{
                            val: val,
                            r: row as u32,
                            c_begin: c_begin,
                            c_end: (col - 1) as u32,
                        });
                    },
                };
                cur = None;
            }
        }
        match cur {
            None => {},
            Some(val) => {
                numbers.push(Number{
                    val: val,
                    r: row as u32,
                    c_begin: c_begin,
                    c_end: (contents[0].len() - 1) as u32,
                });
            },
        };
    }

    let mut gear_to_numbers = HashMap::new();

    let mut sum = 0;
    for number in numbers {
        let start_row = (cmp::max(0, (number.r as i32) - 1)) as usize;
        let end_row = cmp::min(contents.len() - 1, (number.r + 1) as usize);
        let start_col = (cmp::max(0, (number.c_begin as i32) - 1)) as usize;
        let end_col = cmp::min(contents[0].len() - 1, (number.c_end + 1) as usize);

        for r in start_row..=end_row {
            for c in start_col..=end_col {
                if is_symbol(contents[r].chars().nth(c).unwrap()) {
                    if !gear_to_numbers.contains_key(&(r, c)) {
                        gear_to_numbers.insert((r, c), Vec::new());
                    }
                    gear_to_numbers.get_mut(&(r, c)).unwrap().push(number.val);
                }
            }
        }
    }

    for (gear, numbers) in gear_to_numbers.into_iter() {
        if numbers.len() == 2 {
            sum += numbers[0] * numbers[1];
        }
    }

    println!("{}", sum);
}
