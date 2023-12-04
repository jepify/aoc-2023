advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let calibation_value = input
        .trim()
        .split("\n")
        .map(|line| {
            line.chars()
                .find(char::is_ascii_digit)
                .expect("Did not find first digit")
                .to_digit(10)
                .unwrap()
                * 10
                + line
                    .chars()
                    .rev()
                    .find(char::is_ascii_digit)
                    .expect("Did not find last digit")
                    .to_digit(10)
                    .unwrap()
        })
        .sum::<u32>();
    Some(calibation_value)
}

pub fn part_two(input: &str) -> Option<u32> {
    let calibration_value = input
        .trim()
        .lines()
        .map(|line| {
            let mut start: Option<u32> = None;
            let mut end: Option<u32> = None;
            for i in 0..line.len() {
                let start_line = line[i..].to_string();
                let end_line = line[..line.len() - i].to_string();
                if start.is_none() {
                    if start_line.starts_with("one") || start_line.starts_with('1') {
                        start = Some(1);
                    }
                    if start_line.starts_with("two") || start_line.starts_with('2') {
                        start = Some(2);
                    }
                    if start_line.starts_with("three") || start_line.starts_with('3') {
                        start = Some(3);
                    }
                    if start_line.starts_with("four") || start_line.starts_with('4') {
                        start = Some(4);
                    }
                    if start_line.starts_with("five") || start_line.starts_with('5') {
                        start = Some(5);
                    }
                    if start_line.starts_with("six") || start_line.starts_with('6') {
                        start = Some(6);
                    }
                    if start_line.starts_with("seven") || start_line.starts_with('7') {
                        start = Some(7);
                    }
                    if start_line.starts_with("eight") || start_line.starts_with('8') {
                        start = Some(8);
                    }
                    if start_line.starts_with("nine") || start_line.starts_with('9') {
                        start = Some(9);
                    }
                }
                if end.is_none() {
                    if end_line.ends_with("one") || end_line.ends_with('1') {
                        end = Some(1);
                    }
                    if end_line.ends_with("two") || end_line.ends_with('2') {
                        end = Some(2);
                    }
                    if end_line.ends_with("three") || end_line.ends_with('3') {
                        end = Some(3);
                    }
                    if end_line.ends_with("four") || end_line.ends_with('4') {
                        end = Some(4);
                    }
                    if end_line.ends_with("five") || end_line.ends_with('5') {
                        end = Some(5);
                    }
                    if end_line.ends_with("six") || end_line.ends_with('6') {
                        end = Some(6);
                    }
                    if end_line.ends_with("seven") || end_line.ends_with('7') {
                        end = Some(7);
                    }
                    if end_line.ends_with("eight") || end_line.ends_with('8') {
                        end = Some(8);
                    }
                    if end_line.ends_with("nine") || end_line.ends_with('9') {
                        end = Some(9);
                    }
                }
                if start.is_some() && end.is_some() {
                    break;
                }
            }
            start.unwrap() * 10 + end.unwrap()
        })
        .sum::<u32>();

    Some(calibration_value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
