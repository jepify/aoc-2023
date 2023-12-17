advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let sum_of_ids = input
        .trim()
        .lines()
        .map(|line| line.split_once(':').unwrap())
        .filter(|&(_, sets)| {
            sets.split([';', ','])
                .all(|d| match d.trim().split_once(' ') {
                    Some((x, "red")) => x.parse::<u32>().unwrap() <= 12,
                    Some((x, "green")) => x.parse::<u32>().unwrap() <= 13,
                    Some((x, "blue")) => x.parse::<u32>().unwrap() <= 14,
                    _ => false,
                })
        })
        .map(|(game_id, _)| game_id.split_once(' ').unwrap().1.parse::<u32>().unwrap())
        .sum::<u32>();

    Some(sum_of_ids)
}

pub fn part_two(input: &str) -> Option<u32> {
    let sum_of_powers = input
        .trim()
        .lines()
        .map(|line| line.split_once(':').unwrap())
        .map(|(_, sets)| {
            sets.split([';', ','])
                .fold((0, 0, 0), |acc, d| match d.trim().split_once(' ') {
                    Some((x, "red")) if x.parse::<u32>().unwrap() > acc.0 => {
                        (x.parse::<u32>().unwrap(), acc.1, acc.2)
                    }
                    Some((x, "green")) if x.parse::<u32>().unwrap() > acc.1 => {
                        (acc.0, x.parse::<u32>().unwrap(), acc.2)
                    }
                    Some((x, "blue")) if x.parse::<u32>().unwrap() > acc.2 => {
                        (acc.0, acc.1, x.parse::<u32>().unwrap())
                    }
                    _ => acc,
                })
        })
        .map(|(red, green, blue)| red * green * blue)
        .sum::<u32>();

    Some(sum_of_powers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
