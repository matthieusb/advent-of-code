use std::fs::File;
use std::io::{self, BufRead};


#[derive(Debug, PartialEq, Eq)]
enum Side {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq)]
struct Direction {
    side: Side,
    distance: i32,
}

impl Direction {
    fn distance(&self) -> i32 {
        match self.side {
            Side::Left => {
                -1 * self.distance
            },
            Side::Right => {
                self.distance
            },
        }
    }
}

#[allow(dead_code)]
fn deduce_password_part_one(path: &str) -> i32 {
    let directions = parse_directions(path);
    let mut position = 50;
    let mut times_pointed_at_zero = 0;

    for direction in directions {
        let offset = position + (direction.distance() % 100);
        let mut new_position = 0;

        if offset < 0 {
            new_position = 100 + offset
        } else {
            if offset >= 100 {
                new_position = offset - 100
            } else {
                new_position = offset
            }
        }

        position = new_position;

        if position == 0 {
            times_pointed_at_zero += 1;
        }
    }

    times_pointed_at_zero
}

#[allow(dead_code)]
fn deduce_password_part_two(path: &str) -> i32 {
    let directions = parse_directions(path);
    let mut position = 50;
    let mut times_pointed_at_zero = 0;

    for direction in directions {
        let offset = position + (direction.distance() % 100);
        let mut new_position = 0;

        if offset < 0 {
            new_position = 100 + offset
        } else {
            if offset >= 100 {
                new_position = offset - 100
            } else {
                new_position = offset
            }
        }

        if (offset < 0 || offset > 100) && new_position != 0 && position != 0 {
            times_pointed_at_zero += 1;
        }

        position = new_position;

        if position == 0 {
            times_pointed_at_zero += 1;
        }
    }

    times_pointed_at_zero
}

fn parse_directions(path: &str) -> Vec<Direction> {
    let directions_str = read_lines(path).unwrap();

    directions_str.iter().map(|s| {
        let side_str = &s[0..1];
        let side = if side_str == "L" { Side::Left } else { Side::Right };

        Direction { side, distance: s[1..s.len()].parse::<i32>().unwrap_or(0) }
    }).collect()
}

fn read_lines(path: &str) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    Ok(lines)
}

#[cfg(test)]
mod tests_part_one {
    use spectral::assert_that;
    use crate::day1::{deduce_password_part_one, parse_directions, read_lines, Direction, Side};

    #[test]
    fn should_load_test_input() {
        // Given
        let path = "resources/day1/input_test";

        // When
        let lines = read_lines(path).unwrap();

        // Then
        let expected: Vec<String> = vec![
            "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
        ]
            .iter()
            .map(|&s| s.into())
            .collect();

        assert_eq!(lines.len(), 10);
        assert_eq!(lines, expected);
    }

    #[test]
    fn should_parse_directions() {
        // Given
        let path = "resources/day1/input_test";

        // When
        let directions = parse_directions(path);

        // Then
        let expected: Vec<Direction> = vec![
            Direction {
                side: Side::Left,
                distance: 68,
            },
            Direction {
                side: Side::Left,
                distance: 30,
            },
            Direction {
                side: Side::Right,
                distance: 48,
            },
            Direction {
                side: Side::Left,
                distance: 5,
            },
            Direction {
                side: Side::Right,
                distance: 60,
            },
            Direction {
                side: Side::Left,
                distance: 55,
            },
            Direction {
                side: Side::Left,
                distance: 1,
            },
            Direction {
                side: Side::Left,
                distance: 99,
            },
            Direction {
                side: Side::Right,
                distance: 14,
            },
            Direction {
                side: Side::Left,
                distance: 82,
            },
        ];

        assert_eq!(directions.len(), 10);
        assert_eq!(directions, expected);
    }

    #[test]
    fn should_deduce_password_for_test_input() {
        // Given
        let path = "resources/day1/input_test";

        // When
        let password: i32 = deduce_password_part_one(path);

        // Then
        assert_that(&password).is_equal_to(&3);
    }

    #[test]
    fn should_deduce_password_for_test_input_with_larger_numbers() {
        // Given
        let path = "resources/day1/input_test_larger_numbers";

        // When
        let password: i32 = deduce_password_part_one(path);

        // Then
        assert_that(&password).is_equal_to(&5);
    }

    #[test]
    fn should_deduce_password_for_real_input() {
        // Given
        let path = "resources/day1/input_real";

        // When
        let password: i32 = deduce_password_part_one(path);

        // Then
        assert_that(&password).is_equal_to(&1165);
    }
}

#[cfg(test)]
mod tests_part_two {
    use spectral::assert_that;
    use crate::day1::deduce_password_part_two;

    #[test]
    fn should_deduce_password_for_test_input() {
        // Given
        let path = "resources/day1/input_test";

        // When
        let password = deduce_password_part_two(path);

        // Then
        assert_that(&password).is_equal_to(&6);
    }

    #[test]
    fn should_deduce_password_for_test_input_with_larger_numbers() {
        // Given
        let path = "resources/day1/input_test_larger_numbers";

        // When
        let password = deduce_password_part_two(path);

        // Then
        assert_that(&password).is_equal_to(&13);
    }

    #[test]
    fn should_deduce_password_for_real_input() {
        // Given
        let path = "resources/day1/input_real";

        // When
        let password = deduce_password_part_two(path);

        // Then
        assert_that(&password).is_equal_to(&0);
    }
}
