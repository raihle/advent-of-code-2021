#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn absolute_movement() {
        let test_input = "\
forward 5
down 5
forward 8
up 3
down 8
forward 2
";
      let abs_test = move_absolute(test_input);
      let abs_expected = PuzzleResults { x: 15, y: 10 };
      assert_eq!(abs_expected, abs_test);
    }

    #[test]
    fn relative_movement() {
        let test_input = "\
forward 5
down 5
forward 8
up 3
down 8
forward 2
";
      let rel_test = move_relative(test_input);
      let rel_expected = PuzzleResults { x: 15, y: 60 };
      assert_eq!(rel_expected, rel_test);
    }
}

#[derive(Debug, PartialEq)]
pub struct PuzzleResults {
    pub x: i32,
    pub y: i32,
}

pub fn move_absolute(input: &str) -> PuzzleResults {
    let mut x = 0;
    let mut y = 0;

    for command in input.lines() {
        let command_parts: Vec<&str> = command.split_whitespace().collect();
        match command_parts.as_slice() {
            [] => {},
            ["forward", distance] => x += distance.parse::<i32>().unwrap(),
            ["down", distance] => y += distance.parse::<i32>().unwrap(),
            ["up", distance] => y -= distance.parse::<i32>().unwrap(),
            _ => println!("Parse error {}", command),
        }
    }
    return PuzzleResults { x: x, y: y };
}

pub fn move_relative(input: &str) -> PuzzleResults {
    let mut x = 0;
    let mut y = 0;
    let mut dy = 0;

    for command in input.lines() {
        let command_parts: Vec<&str> = command.split_whitespace().collect();
        match command_parts.as_slice() {
            [] => {},
            ["forward", delta] => {
                let dx = delta.parse::<i32>().unwrap();
                x += dx;
                y += dx * dy;
            },
            ["down", delta] => dy += delta.parse::<i32>().unwrap(),
            ["up", delta] => dy -= delta.parse::<i32>().unwrap(),
            _ => println!("Parse error {}", command),
        }
    }
    return PuzzleResults { x, y };
}
