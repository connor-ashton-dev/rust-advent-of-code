use std::{cmp, collections::HashMap};

static NUM_RED: i32 = 12;
static NUM_GREEN: i32 = 13;
static NUM_BLUE: i32 = 14;

#[derive(Debug)]
struct Game<'a> {
    number: i32,
    scores: HashMap<&'a str, i32>,
}

pub fn run(contents: &str) -> i32 {
    let mut score = 0;
    let lines = contents.lines();
    for line in lines {
        let parsed_game = match parse(line) {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        };
        score += is_game_valid(&parsed_game);
    }
    score
}

fn is_game_valid(g: &Game) -> i32 {
    for (k, v) in &g.scores {
        let color = k.chars().nth(0).expect("Invalid color");
        if (color == 'b' && v > &NUM_BLUE)
            || (color == 'r' && v > &NUM_RED)
            || (color == 'g' && v > &NUM_GREEN)
        {
            return 0;
        }
    }
    g.number
}

fn parse(line: &str) -> Result<Game, &str> {
    // split up to the ':'
    let parts: Vec<&str> = line.split(':').collect();
    if parts.len() != 2 {
        return Err("Invalid line");
    }

    let game_number = match get_game_number(parts[0]) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    let game_map = match scores_map(parts[1]) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    let my_game = Game {
        number: game_number,
        scores: game_map,
    };

    Ok(my_game)
}

fn get_game_number(s: &str) -> Result<i32, &str> {
    let parts: Vec<&str> = s.split(' ').collect();
    if parts.len() != 2 {
        return Err("Invalid line in [GET_GAME_NUMBER]");
    }
    let number = parts[1];
    match number.parse::<i32>() {
        Ok(v) => Ok(v),
        Err(_) => Err("Error parsing number in [GET_GAME_NUMBER]"),
    }
}

fn scores_map(s: &str) -> Result<HashMap<&str, i32>, &str> {
    let mut my_map: HashMap<&str, i32> = HashMap::new();
    let rounds_seperated: Vec<&str> = s.split(';').collect();
    for round in rounds_seperated {
        let turns_seperated: Vec<&str> = round.split(',').collect();
        for turn in turns_seperated {
            let parts: Vec<&str> = turn.trim().split(' ').collect();
            let num = parts[0].parse::<i32>().expect("Error parsing number");
            let color = parts[1];
            let entry = my_map.entry(color).or_insert(0);
            *entry = cmp::max(*entry, num)
        }
    }

    Ok(my_map)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case() {
        let contents = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(run(&contents), 8)
    }
}
