use std::fs;

#[derive(Debug, PartialEq, Eq)]
struct Range {
    begin: i32,
    end: i32
}


fn compute_invalid_id_sum(path: &str) -> i32 {
    let ranges = import_ranges(path);

    // step: create actual range of values for each range
     ranges.iter()
         .map(|range: &Range| { range.begin ..=range.end })
         .map(|range| {  })

    // step: remove all odd length stuff

    // step: keep all palindromic stuff

    // step: add all values

    todo!()
}

fn import_ranges(path: &str) -> Vec<Range> {
    let ranges_as_string = read_ranges_file(path);

    ranges_as_string.split(",").map(|range_str| {
        let range_parts: Vec<&str> = range_str.split("-").collect();
        Range {begin: range_parts[0].parse().unwrap(), end: range_parts[1].parse().unwrap()}
    }).collect()
}

fn read_ranges_file(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}

#[cfg(test)]
mod tests_part_one {
    use crate::day2::{compute_invalid_id_sum, import_ranges, read_ranges_file, Range};
    use spectral::assert_that;

    #[test]
    fn should_import_ranges_string() {
        // Given
        let path = "resources/day2/input_test";

        // When Then
        assert_that!(read_ranges_file(path))
            .is_equal_to("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124".to_string());
    }

    #[test]
    fn should_import_ranges() {
        // Given
        let path = "resources/day2/input_test_short";

        // When
        let ranges = import_ranges(path);

        // Then
        let expected = vec![
            Range{ begin: 11, end: 22 },
            Range{ begin: 95, end: 115 },
            Range{ begin: 998, end: 1012 },
            Range{ begin: 1188511880, end: 1188511890 },
            Range{ begin: 222220, end: 222224 },
            Range{ begin: 1698522, end: 1698528 },
            Range{ begin: 446443, end: 446449 },
            Range{ begin: 38593856, end: 38593862 }
        ];

        assert_that!(&ranges).is_equal_to(&expected);
    }

    #[test]
    fn should_compute_invalid_id_sum_for_short_test_input() {
        // Given
        let path = "resources/day2/input_test_short";

        // When
        let invalid_id_sum = compute_invalid_id_sum(path);

        // Then
        assert_that!(invalid_id_sum).is_equal_to(1227775554);
    }
}