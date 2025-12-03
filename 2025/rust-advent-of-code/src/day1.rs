use std::fs::File;
use std::io::{self, BufRead};

fn read_lines(path: &str) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    Ok(lines)
}

#[cfg(test)]
mod tests {
    use crate::day1::read_lines;

    #[test]
    fn should_load_test_input_correctly() {
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
}
