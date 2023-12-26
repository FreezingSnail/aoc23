use std::fs;

const RED: u8 = 12;
const GREEN: u8 = 13;
const BLUE: u8 = 14;

struct Game {
    blue: u8,
    red: u8,
    green: u8,
    //yellow: u8,
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

fn validate_games(games: &Vec<Game>) -> bool {
    for game in games {
        if !validate_game(game) {
            return false;
        }
    }
    return true;
}

fn validate_game(game: &Game) -> bool {
    if game.blue > BLUE || game.red > RED || game.green > GREEN {
        return false;
    }
    return true;
}

fn main() {
    let contents = fs::read_to_string("input").expect("Should have been able to read the file");
    let mut sum = 0;
    for (i, line) in contents.lines().enumerate() {
        //print!("{} ", line);
        let games = parse_line_to_games(line);
        if validate_games(&games) {
            sum += 1 + i;
        }
    }

    println!("Valid games: {}", sum);
}
