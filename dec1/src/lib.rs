fn parse_line(line: &str) -> Option<u32> {
    let mut num_word = String::new();
    let mut last: Option<char> = None;

    for char in line.chars() {
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
    }
    #[test]
    fn test_parse_single_num() {
        assert_eq!(parse_num("3"), Some(3));
    }
    #[test]
    fn test_pare_num_no_nums() {
        assert_eq!(parse_num("a"), None);
    }

    // parse_line tests
    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("a2defghi3"), Some(23));
    }
    #[test]
    fn test_parse_line_single_number() {
        assert_eq!(parse_line("a5x"), Some(55));
    }
    #[test]
    fn test_parse_line_no_numbers() {
        assert_eq!(parse_line("abcdef"), None);
    }
}

#[cfg(test)]
mod e2e_tests {
    use super::*;
    #[test]
    fn provided_test_case() {
        let contents = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        assert_eq!(run(&contents), 142);
    }

    #[test]
    fn no_numbers() {
        let contents = "abc\ndef\nghi";
        assert_eq!(run(&contents), 0);
    }
}
