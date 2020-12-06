mod prelude;
use prelude::*;
use std::collections::{HashSet, HashMap};

type Group = Vec<u32>;

fn parse(line: &str) -> u32 {
    // convert sequence of letters a-z into a binary number
    line.chars().fold(0, |n, ch| n | 1 << (ch as usize - 97))
}

fn group<T>(it: &mut T) -> Option<Group>
    where T: Iterator<Item=String> {
    let batch = it
        .take_while(|line| line.len() != 0)
        .map(|line| parse(&line))
        .collect_vec();
    if batch.len() > 0 { Some(batch) } else { None }
}

fn solve<T>(group: &Group, transform: T) -> u32
    where T: Fn(u32, u32) -> u32 {
    group.into_iter()
        .skip(1)
        .fold(group[0], |n, &x| transform(n, x))
        .count_ones()
}

fn main() {
    let groups = stdin().lock().lines()
        .filter_map(Result::ok)
        .batching(group)
        .collect_vec();

    println!("{}", groups.iter().map(|g| solve(&g, |a, b| a | b)).sum::<u32>());
    println!("{}", groups.iter().map(|g| solve(&g, |a, b| a & b)).sum::<u32>());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn groups() -> Vec<Group> {
        let example = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb\n";
        example.lines()
            .map(|line| line.to_string())
            .batching(group)
            .collect_vec()
    }

    #[test]
    fn example_1() {
        assert_eq!(groups().iter().map(|g| solve(&g, |a, b| a | b)).sum::<u32>(), 11);
    }

    #[test]
    fn example_2() {
        assert_eq!(groups().iter().map(|g| solve(&g, |a, b| a & b)).sum::<u32>(), 6);
    }
}