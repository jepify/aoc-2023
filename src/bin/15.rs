advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u32> {
    let hash_value = input
        .trim()
        .split(',')
        .map(|word| {
            word.chars()
                .map(|c| c as u32)
                .fold(0, |acc, cur| ((acc + cur) * 17) % 256)
        })
        .sum::<u32>();
    Some(hash_value)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
