extern crate aoc_runner;
#[macro_use]
extern crate aoc_runner_derive;

use std::collections::HashSet;

#[aoc_generator(day1)]
fn frequency_generator(input: &str) -> Vec<i64> {
    input
        .lines()
        .into_iter()
        .map(str::parse::<i64>)
        .map(Result::unwrap)
        .collect()
}

#[aoc(day1, part1)]
fn frequency_calculator(input: &[i64]) -> i64 {
    input.iter().sum()
}

#[aoc(day1, part2)]
fn frequence_repeat(input: &[i64]) -> i64 {
    let mut seen = HashSet::with_capacity(input.len());
    let mut sum = 0;
    for n in input.iter().cycle() {
        if seen.contains(&sum) {
            return sum;
        } else {
            seen.insert(sum);
            sum += n;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

aoc_lib! { year = 2018 }
