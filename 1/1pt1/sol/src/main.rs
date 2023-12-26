use std::fs;

fn parse_value(value: &str) -> u32 {
    let mut l = 0;
    let mut r = value.len() - 1;

    while !value.chars().nth(l).expect("value").is_digit(10) {
        l += 1;
    }
    while !value.chars().nth(r).expect("value").is_digit(10) {
        r -= 1;
    }
    let val = value.chars().nth(l).expect("value").to_string()
        + &value.chars().nth(r).expect("value").to_string();

    return val.parse::<u32>().unwrap();
}

fn main() {
    let contents = fs::read_to_string("input").expect("Should have been able to read the file");

    let mut sum: u32 = 0;
    for line in contents.lines() {
        let val = parse_value(line);
        sum += val;
    }

    println!("{}", sum);
}
