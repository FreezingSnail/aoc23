use std::fs;

const NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn is_written_digit(value: &str, index: usize) -> bool {
    for n in NUMBERS {
        if value[index..].starts_with(n) {
            print!("word: {} ", n);
            return true;
        }
    }

    return false;
}

fn parse_number(value: &str, index: usize) -> u32 {
    if value
        .chars()
        .nth(index)
        .expect(&format!("panic index {}, {}", index, value.len()))
        .is_digit(10)
    {
        return value
            .chars()
            .nth(index)
            .expect("parse number panic")
            .to_digit(10)
            .unwrap();
    }

    for (i, n) in NUMBERS.iter().enumerate() {
        if value[index..].starts_with(n) {
            return (i + 1) as u32;
        }
    }

    return 0;
}

fn parse_value(value: &str) -> u32 {
    let mut l = 0;
    let mut r = value.len() - 1;
    while l < r {
        print!(
            "{} ",
            value
                .chars()
                .nth(l)
                .expect(&format!("panic l {}, {}", l, value.len()))
        );
        if value
            .chars()
            .nth(l)
            .expect(&format!("panic l {}, {}", l, value.len()))
            .is_digit(10)
        {
            break;
        }
        if is_written_digit(value, l) {
            break;
        }
        l += 1;
    }
    while r > l {
        if value.chars().nth(r).expect("panic r").is_digit(10) {
            break;
        }
        if is_written_digit(value, r) {
            break;
        }
        r -= 1;
    }
    let val = parse_number(value, l).to_string() + &parse_number(value, r).to_string();

    return val.parse::<u32>().unwrap();
}

fn main() {
    let contents = fs::read_to_string("input").expect("Should have been able to read the file");

    let mut sum: u32 = 0;
    for line in contents.lines() {
        print!("{} = ", line);
        let val = parse_value(line);
        println!("{}", val);
        sum += val;
    }

    println!("{}", sum);
}
