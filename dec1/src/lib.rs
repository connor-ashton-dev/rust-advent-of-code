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
mod tests {
    use super::*;

    // Unit Tests
    #[test]
    fn test_parse_num() {
        assert_eq!(parse_num("32"), Some(32));
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("a2defghi3"), Some(23));
    }

    #[test]
    fn test_parse_line_single_number() {
        assert_eq!(parse_line("a5x"), Some(55));
    }

    // E2E Tests
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
