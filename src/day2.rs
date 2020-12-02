mod prelude;
use prelude::*;

type Validator = fn(Range<usize>, char, &str) -> Option<bool>;

fn main() {
    let passwords = std::io::stdin().lock()
        .lines()
        .filter_map(Result::ok)
        .collect_vec();

    println!("{}", passwords.iter().filter(|p| validate(&p, policy_1) == Some(true)).count());
    println!("{}", passwords.iter().filter(|p| validate(&p, policy_2) == Some(true)).count());
}

fn policy_1(range: Range<usize>, letter: char, value: &str) -> Option<bool> {
    let count = value.matches(letter).count();
    Some(range.contains(&count))
}

fn policy_2(range: Range<usize>, letter: char, value: &str) -> Option<bool> {
    let first = value.chars().nth(range.start - 1)?;
    let second = value.chars().nth(range.end - 2)?;
    Some((first == letter || second == letter) && first != second)
}

fn validate(line: &str, predicate: Validator) -> Option<bool> {
    let mut parts = line.split(' ');
    let mut range = parts.next()?.split('-');
    let min = range.next()?.parse::<usize>().ok()?;
    let max = range.next()?.parse::<usize>().ok()?;
    let letter = parts.next()?.chars().nth(0)?;
    let password = parts.next()?;
    predicate(min..max + 1, letter, password)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Some(true), validate("1-3 a: abcde", policy_1));
        assert_eq!(Some(false), validate("1-3 b: cdefg", policy_1));
        assert_eq!(Some(true), validate("2-9 c: ccccccccc", policy_1));
    }

    #[test]
    fn example_2() {
        assert_eq!(Some(true), validate("1-3 a: abcde", policy_2));
        assert_eq!(Some(false), validate("1-3 b: cdefg", policy_2));
        assert_eq!(Some(false), validate("2-9 c: ccccccccc", policy_2));
    }
}
