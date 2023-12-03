use std::{cmp, collections::HashMap};

#[derive(Debug)]
struct Game<'a> {
    scores: HashMap<&'a str, i32>,
}

impl Game<'_> {
    fn multiply_scores(&self) -> i32 {
        let mut score = 1;
        for (_, v) in &self.scores {
            score *= v;
        }
        score
    }
}

pub fn run(contents: &str) -> i32 {
    let mut score = 0;
    let lines = contents.lines();
    for line in lines {
        let parsed_game = match parse(line) {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        };
        score += parsed_game.multiply_scores();
    }
    score
}

fn parse(line: &str) -> Result<Game, &str> {
    // split up to the ':'
    let parts: Vec<&str> = line.split(':').collect();
    if parts.len() != 2 {
        return Err("Invalid line");
    }

    let game_map = match scores_map(parts[1]) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    let my_game = Game { scores: game_map };

    Ok(my_game)
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
