use std::collections::HashMap;

fn parse_line(line: &str) -> Option<u32> {
    let parsed_line = turn_words_to_numbers(line);
    let mut num_word = String::new();
    let mut last: Option<char> = None;

    for char in parsed_line.chars() {
        if char.is_numeric() {
            if num_word.is_empty() {
                num_word.push(char);
            }
            last = Some(char);
        }
    }

    if last != None {
        num_word.push(last.unwrap());
    }

    if !num_word.is_empty() {
        parse_num(&num_word)
    } else {
        None
    }
}

fn turn_words_to_numbers(line: &str) -> String {
    let mut my_string = String::from(line);
    let my_map = HashMap::from([
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3ree"),
        ("four", "f4ur"),
        ("five", "f5ve"),
        ("six", "s6x"),
        ("seven", "s7ven"),
        ("eight", "e8ght"),
        ("nine", "n9ne"),
    ]);

    for (k, v) in my_map {
        my_string = my_string.replace(k, v);
    }

    my_string
}

fn parse_num(s: &str) -> Option<u32> {
    match s.parse::<u32>() {
        Ok(num) => Some(num),
        Err(e) => {
            println!("Error parsing number {:?}", e);
            None
        }
    }
}

pub fn run(contents: &str) -> u32 {
    contents
        .lines()
        .map(|line| parse_line(line).unwrap_or(0))
        .sum()
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    // parse_num tests
    #[test]
    fn test_parse_num() {
        assert_eq!(parse_num("32"), Some(32));
        assert_eq!(parse_num("3"), Some(3));
        assert_eq!(parse_num("a"), None);
        assert_eq!(parse_num(""), None);
    }

    // turn_words_to_numbers tests
    #[test]
    fn test_nums_to_words() {
        assert_eq!(turn_words_to_numbers("nineight"), "n9ne8ght");
        assert_eq!(turn_words_to_numbers("twone"), "t2o1e");
        assert_eq!(turn_words_to_numbers("21"), "21");
        assert_eq!(turn_words_to_numbers(""), "");
    }

    // parse_line tests
    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("a2defghi3"), Some(23));
        assert_eq!(parse_line("a5x"), Some(55));
        assert_eq!(parse_line("abcdef"), None);
        assert_eq!(parse_line(""), None);
        // including turn_words_to_numbers
        assert_eq!(parse_line("fiveight"), Some(58));
        assert_eq!(parse_line("sevenine"), Some(79));
    }
}

#[cfg(test)]
mod e2e_tests {
    use super::*;
    #[test]
    fn first_test_case() {
        let contents = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        assert_eq!(run(&contents), 142);
    }

    #[test]
    fn second_test_case() {
        let contents = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        assert_eq!(run(&contents), 281);
    }

    #[test]
    fn no_numbers() {
        let contents = "abc\ndef\nghi";
        assert_eq!(run(&contents), 0);
    }
}
