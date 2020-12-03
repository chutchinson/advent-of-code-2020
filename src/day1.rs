mod prelude;
use prelude::*;

fn solve(v: &Vec<u32>, n: usize) -> u32 {
    v.iter().cloned()
        .combinations(n)
        .filter(|x| x.iter().sum::<u32>() == 2020)
        .map(|x| x.iter().product())
        .next()
        .expect("no solution")
}

fn main() {
    let expenses = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.expect("failed to read line"))
        .map(|line| line.parse::<u32>().expect("invalid input"))
        .collect_vec();

    println!("{}", solve(&expenses, 2));
    println!("{}", solve(&expenses, 3));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let v = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(514579, solve(&v, 2));
    }

    #[test]
    fn example_2() {
        let v = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(241861950, solve(&v, 3));
    }
}
