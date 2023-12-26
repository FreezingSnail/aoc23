use std::fmt;
use std::fs;

const RED: u8 = 12;
const GREEN: u8 = 13;
const BLUE: u8 = 14;

#[derive(Debug)]
struct Game {
    blue: u8,
    red: u8,
    green: u8,
    //yellow: u8,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Blue: {}, Red: {}, Green: {}",
            self.blue, self.red, self.green
        )
    }
}

fn parse_line_to_games(line: &str) -> Vec<Game> {
    let mut games = Vec::new();

    let trimmed = line.split(":").last().unwrap();
    for game in trimmed.split(';') {
        let mut ret = Game {
            blue: 0,
            red: 0,
            green: 0,
        };
        for subline in game.split(',') {
            let subline = subline.trim();

            let val = subline.split(' ').next().unwrap();
            let color = subline.split(' ').last().unwrap();

            match color {
                "blue" => ret.blue = val.parse::<u8>().unwrap(),
                "red" => ret.red = val.parse::<u8>().unwrap(),
                "green" => ret.green = val.parse::<u8>().unwrap(),
                //"yellow" => game.yellow = val.parse::<u8>().unwrap(),
                _ => println!("Unknown color {}", color),
            }
        }
        games.push(ret);
    }

    return games;
}

fn least_colors(games: &Vec<Game>) -> Game {
    let mut ret = Game {
        blue: 0,
        red: 0,
        green: 0,
    };

    for g in games {
        println!("Game: {}", g);
        if g.blue > ret.blue {
            ret.blue = g.blue;
        }
        if g.red > ret.red {
            ret.red = g.red;
        }
        if g.green > ret.green {
            ret.green = g.green;
        }
    }
    println!("Least colors: {}", ret);
    return ret;
}

fn sum_power_set(game: &Game) -> u32 {
    return game.blue as u32 * game.red as u32 * game.green as u32;
}

fn main() {
    let contents = fs::read_to_string("input").expect("Should have been able to read the file");
    let mut sum = 0;
    for (i, line) in contents.lines().enumerate() {
        //print!("{} ", line);
        let games = parse_line_to_games(line);
        sum += sum_power_set(&least_colors(&games));
    }

    println!("Valid games: {}", sum);
}
