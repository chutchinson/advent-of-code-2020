mod prelude;
use prelude::*;

fn solve(v: &Vec<u32>, n: usize) -> u32 {
    v.iter().combinations(n)
        .filter(|x| x.iter().fold(0, |sum, &n| sum + n) == 2020)
        .map(|x| x.iter().fold(1, |res, &n| res * n))
        .next().expect("no solution")
}

fn main() {
    let expenses = std::io::stdin().lock()
        .lines()
        .filter_map(Result::ok)
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
        let v  = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(514579, solve(&v, 2).unwrap());
    }

    #[test]
    fn example_2() {
        let v  = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(241861950, solve(&v, 3).unwrap());
    }
}