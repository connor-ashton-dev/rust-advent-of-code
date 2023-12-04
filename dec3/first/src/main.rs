use std::fs;

pub type CharMatrix = Vec<Vec<char>>;

fn main() {
    let mut char_matrix: CharMatrix = Vec::new();
    let content = fs::read_to_string("input.txt").expect("incorrect file name");
    generate_matrix(&content, &mut char_matrix);

    let nums = find_valid_numbers(&char_matrix);

    let sum: i32 = nums.iter().sum();

    println!("{}", sum)
}

fn find_valid_numbers(m: &CharMatrix) -> Vec<i32> {
    let mut nums: Vec<i32> = Vec::new();
    for (i, row) in m.iter().enumerate() {
        let mut j = 0;
        while j < row.len() {
            if row[j].is_digit(10) {
                let start = j;
                while j < row.len() && row[j].is_digit(10) {
                    j += 1;
                }
                let end = j - 1;

                // Check if the number is a valid part number
                if is_valid((i, start), m) || is_valid((i, end), m) {
                    if let Ok(num) = row[start..=end].iter().collect::<String>().parse::<i32>() {
                        nums.push(num);
                    }
                }
            } else {
                j += 1;
            }
        }
    }
    nums
}

fn is_valid(point: (usize, usize), m: &CharMatrix) -> bool {
    let (i, j) = point;
    let rows = m.len();
    let cols = m[0].len();

    // Check left of the number
    if j > 0 && !m[i][j - 1].is_digit(10) && m[i][j - 1] != '.' {
        return true;
    }

    // Check right of the number
    if j < cols - 1 && !m[i][j + 1].is_digit(10) && m[i][j + 1] != '.' {
        return true;
    }

    // Check above and below the number
    for dj in [j as i32 - 1, j as i32, j as i32 + 1].iter() {
        if *dj >= 0 && (*dj as usize) < cols {
            // Check above if not in the first row
            if i > 0 && !m[i - 1][*dj as usize].is_digit(10) && m[i - 1][*dj as usize] != '.' {
                return true;
            }
            // Check below if not in the last row
            if i < rows - 1 && !m[i + 1][*dj as usize].is_digit(10) && m[i + 1][*dj as usize] != '.'
            {
                return true;
            }
        }
    }

    false
}

fn generate_matrix(c: &str, m: &mut CharMatrix) {
    let lines = c.lines();
    for line in lines {
        let vec: Vec<char> = line.chars().collect();
        m.push(vec);
    }
}
