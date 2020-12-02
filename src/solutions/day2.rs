use crate::prelude::*;

type Validator = fn(Range<usize>, char, &str) -> bool;

fn check_1(range: Range<usize>, letter: char, value: &str) -> bool {
    let count = value.matches(letter).count();
    range.contains(&count)
}

fn check_2(range: Range<usize>, letter: char, value: &str) -> bool {
    let first = value.chars().nth(range.start - 1).unwrap();
    let second = value.chars().nth(range.end - 2).unwrap();
    (first == letter || second == letter) && first != second
}

fn validate(line: &str, predicate: Validator) -> Option<bool> {
    let mut parts = line.split(' ');
    let mut range = parts.next()?.split('-');
    let min = range.next()?.parse::<usize>().ok()?;
    let max = range.next()?.parse::<usize>().ok()?;
    let letter = parts.next()?.chars().nth(0)?;
    let password = parts.next()?;
    Some(predicate(min..max+1, letter, password))
}

pub fn solve(ctx: &mut Context) -> Result {

    let lines: Vec<_> = ctx.load("2.txt")?
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let count = lines.iter()
        .filter(|line| validate(line, check_1).unwrap_or(false))
        .count();
    
    writeln!(ctx, "{}", count);

    let count = lines.iter()
        .filter(|line| validate(line, check_2).unwrap_or(false))
        .count();
    
    writeln!(ctx, "{}", count);

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_1() {
        assert_eq!(Some(true), validate("1-3 a: abcde", check_1));
        assert_eq!(Some(false), validate("1-3 b: cdefg", check_1));
        assert_eq!(Some(true), validate("2-9 c: ccccccccc", check_1));
    }

    #[test]
    fn test_check_2() {
        assert_eq!(Some(true), validate("1-3 a: abcde", check_2));
        assert_eq!(Some(false), validate("1-3 b: cdefg", check_2));
        assert_eq!(Some(false), validate("2-9 c: ccccccccc", check_2));
    }
}