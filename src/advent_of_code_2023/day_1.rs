pub fn get_all_calibration_values(lines_of_text: &[&str]) -> Vec<i32> {
    lines_of_text
        .iter()
        .copied()
        .filter_map(get_calibration_value)
        .collect()
}

fn get_calibration_value(text: &str) -> Option<i32> {
    let mut digits = text.matches(char::is_numeric);
    let first = digits.next()?;
    let last = digits.last().unwrap_or(first);
    let number = first.to_string() + last;
    number.parse().ok()
}

pub fn combine_calibration_values(calibration_values: &[i32]) -> i32 {
    calibration_values.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::combine_calibration_values;
    use super::get_all_calibration_values;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_gets_all_the_calibration_values() {
        let input = ["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
        let expected = vec![12, 38, 15, 77];
        let actual = get_all_calibration_values(&input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_combines_the_calibration_values() {
        let input = [12, 38, 15, 77];
        let expected = 142;
        let actual = combine_calibration_values(&input);
        assert_eq!(expected, actual);
    }
}
