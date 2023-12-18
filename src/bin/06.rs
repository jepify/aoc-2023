use std::u32;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let (times, distances) = input.split_once('\n').unwrap();
    let times = times[9..]
        .split_whitespace()
        .map(|s| s.parse::<f32>().unwrap())
        .zip(
            distances[9..]
                .split_whitespace()
                .map(|s| s.parse::<f32>().unwrap()),
        )
        .map(|(t, d)| {
            let square_root_part = f32::sqrt(t.powi(2) - 4.0 * (d + 1.0));
            let lower = f32::ceil((-t + square_root_part) / -2.0) as u32;
            let upper = f32::floor((-t - square_root_part) / -2.0) as u32;
            upper - lower + 1
        })
        .product::<u32>();

    Some(times)
}

fn part_one_simple(input: &str) -> Option<u32> {
    let (times, distances) = input.split_once('\n').unwrap();
    let times = times
        .strip_prefix("Time:")
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<f32>().unwrap())
        .collect::<Vec<_>>();
    let distances = distances
        .strip_prefix("Distance:")
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<f32>().unwrap())
        .collect::<Vec<_>>();
    let number_of_way_bounds = times
        .iter()
        .zip(distances.iter())
        .map(|(t, d)| {
            (
                // d = -x^2 + tx - r + 1
                // x = 0 => solutions where we are one more than r which is the record
                f32::ceil((-t + f32::sqrt(t.powi(2) - 4.0 * (d + 1.0))) / -2.0),
                f32::floor((-t - f32::sqrt(t.powi(2) - 4.0 * (d + 1.0))) / -2.0),
            )
        })
        .collect::<Vec<(f32, f32)>>();
    let number_of_ways = number_of_way_bounds
        .iter()
        .map(|(lower, upper)| upper - lower + 1.0)
        .collect::<Vec<f32>>();

    let product = number_of_ways.iter().product::<f32>();
    Some(product as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    // // Funny here where the values does not fit into a u32 and f64
    let (times, distances) = input.split_once('\n').unwrap();
    let time = times[9..].replace(" ", "").parse::<f64>().unwrap();
    let distance = distances[9..].replace(" ", "").parse::<f64>().unwrap();

    let square_root_part = f64::sqrt(time.powi(2) - 4.0 * (distance + 1.0));
    let lower = f64::ceil((-time + square_root_part) / -2.0) as u64;
    let upper = f64::floor((-time - square_root_part) / -2.0) as u64;

    Some(upper - lower + 1)
    // part_two_naive(input)
}

pub fn part_two_naive(input: &str) -> Option<u64> {
    let (times, distances) = input.split_once('\n').unwrap();
    let time = times[9..].replace(" ", "").parse::<u64>().unwrap();
    let distance = distances[9..].replace(" ", "").parse::<u64>().unwrap();

    let mut number_of_ways = 0;
    for holding_time in 1..=time {
        let potential_distance = (time - holding_time) * holding_time;
        if potential_distance > distance {
            number_of_ways += 1;
        }
    }
    Some(number_of_ways)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
