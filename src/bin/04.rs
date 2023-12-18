use std::collections::{HashMap, HashSet};

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let card_score_sum = input
        .lines()
        .map(|line| line.split_once(": ").unwrap().1.split_once(" | ").unwrap())
        .map(|(winning_numbers, my_numbers)| {
            let overlapping_numbers = winning_numbers
                .split_whitespace()
                .collect::<HashSet<&str>>()
                .intersection(&my_numbers.split_whitespace().collect::<HashSet<&str>>())
                .count();
            if overlapping_numbers == 0 {
                0
            } else {
                u32::pow(2, (overlapping_numbers - 1) as u32)
            }
        })
        .sum::<u32>();
    Some(card_score_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let card_score_sum = input
        .lines()
        .map(|line| line.split_once(": ").unwrap().1.split_once(" | ").unwrap())
        .map(|(winning_numbers, my_numbers)| {
            let winning_numbers_parsed = winning_numbers.split_whitespace().collect::<Vec<&str>>();
            let overlapping_numbers = my_numbers
                .split_whitespace()
                .filter(|n| winning_numbers_parsed.contains(n))
                .count();
            overlapping_numbers as u32
        })
        .zip(1_u32..)
        .fold(HashMap::new(), |mut acc, (winning_entries, card_number)| {
            let associated_cards = *acc.entry(card_number).and_modify(|c| *c += 1).or_insert(1);

            for i in 1..=winning_entries {
                acc.entry(card_number + i)
                    .and_modify(|c| *c += associated_cards)
                    .or_insert(associated_cards);
            }
            acc
        })
        .values()
        .sum::<u32>();
    Some(card_score_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
