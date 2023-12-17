advent_of_code::solution!(3);

#[derive(Debug)]
struct Number {
    number: u32,
    range: (u32, u32),
    line: u32,
}

fn parse_input(input: &str) -> Vec<Number> {
    let mut is_at_number = false;
    let mut start_index = 0;
    let mut end_index = 0;

    let mut numbers: Vec<Number> = Vec::new();

    for (line_number, line) in input.lines().enumerate() {
        is_at_number = false;
        for (col, char_at_index) in line.char_indices() {
            // let index = line * column_numbers + col;

            if let '0'..='9' = char_at_index {
                if !is_at_number {
                    start_index = col;
                    is_at_number = true;
                }
            } else {
                if is_at_number {
                    end_index = col - 1;
                    is_at_number = false;
                    numbers.push(Number {
                        number: line[start_index..=end_index]
                            .parse::<u32>()
                            .expect(&format!("{},{} - {}", start_index, end_index, line)),
                        line: line_number as u32,
                        range: match (start_index, end_index) {
                            (0, 0) => (0, 0),
                            (0, e) => {
                                if e == line.len() - 1 {
                                    (0, e as u32)
                                } else {
                                    (0, (e + 1) as u32)
                                }
                            }
                            (s, e) => {
                                if e == line.len() - 1 {
                                    ((s - 1) as u32, e as u32)
                                } else {
                                    ((s - 1) as u32, (e + 1) as u32)
                                }
                            }
                        },
                    });
                }
                if char_at_index != '.' {
                    continue;
                }
            }
        }
        if is_at_number {
            end_index = line.len() - 1;
            is_at_number = false;
            numbers.push(Number {
                number: line[start_index..=end_index]
                    .parse::<u32>()
                    .expect(&format!("{},{} - {}", start_index, end_index, line)),
                line: line_number as u32,
                range: match (start_index, end_index) {
                    (0, 0) => (0, 0),
                    (0, e) => {
                        if e == line.len() - 1 {
                            (0, e as u32)
                        } else {
                            (0, (e + 1) as u32)
                        }
                    }
                    (s, e) => {
                        if e == line.len() - 1 {
                            ((s - 1) as u32, e as u32)
                        } else {
                            ((s - 1) as u32, (e + 1) as u32)
                        }
                    }
                },
            });
        }
    }
    numbers
}

pub fn part_one(input: &str) -> Option<u32> {
    let jep = parse_input(input)
        .iter()
        .filter_map(
            |Number {
                 number,
                 line,
                 range: (start_range, end_range),
             }| {
                let middle_line = input.lines().nth((*line) as usize).unwrap();
                let middle_line_to_check =
                    middle_line[(*start_range as usize)..=(*end_range as usize)].to_string();
                if middle_line_to_check
                    .chars()
                    .any(|c| !c.is_digit(10) && c != '.')
                {
                    return Some(*number);
                }
                match (
                    input.lines().nth((*line as i32 - 1) as usize),
                    input.lines().nth((*line + 1) as usize),
                ) {
                    (None, None) => None,
                    (None, Some(line_below)) => {
                        let range_to_check =
                            line_below[(*start_range as usize)..=(*end_range as usize)].to_string();
                        if range_to_check.chars().any(|c| c != '.') {
                            Some(*number)
                        } else {
                            None
                        }
                    }
                    (Some(line_above), None) => {
                        let range_to_check =
                            line_above[(*start_range as usize)..=(*end_range as usize)].to_string();
                        if range_to_check.chars().any(|c| c != '.') {
                            Some(*number)
                        } else {
                            None
                        }
                    }
                    (Some(line_above), Some(line_below)) => {
                        let range_to_check_avove =
                            line_above[(*start_range as usize)..=(*end_range as usize)].to_string();
                        let range_to_check_below =
                            line_below[(*start_range as usize)..=(*end_range as usize)].to_string();

                        if range_to_check_avove.chars().any(|c| c != '.')
                            || range_to_check_below.chars().any(|c| c != '.')
                        {
                            Some(*number)
                        } else {
                            None
                        }
                    }
                }
            },
        )
        .sum::<u32>();

    Some(jep)
}

pub fn part_two(input: &str) -> Option<u32> {
    let numbers = parse_input(input);
    let gear_ratios = input
        .lines()
        .enumerate()
        .map(|(line_number, line)| {
            line.char_indices()
                .map(move |(col, chr)| (col, chr, line_number))
                .filter(|(_, chr, _)| *chr == '*')
                .map(|(col, _, l)| {
                    let gear_ratios = numbers
                        .iter()
                        .filter(|n| {
                            let line_diff = n.line as i32 - (l as i32);
                            -1 <= line_diff
                                && line_diff <= 1
                                && n.range.0 <= col as u32
                                && col as u32 <= n.range.1
                        })
                        .map(|n| n.number)
                        .collect::<Vec<u32>>();
                    if gear_ratios.len() == 2 {
                        gear_ratios.iter().product()
                    } else {
                        0
                    }
                })
                .sum::<u32>()
        })
        .sum::<u32>();
    Some(gear_ratios)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
