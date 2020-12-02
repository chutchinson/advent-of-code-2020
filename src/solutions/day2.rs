use crate::prelude::*;

type Validator = fn(usize, usize, char, &str) -> bool;

fn check_1(a: usize, b: usize, letter: char, value: &str) -> bool {
    let count = value.matches(letter).count();
    count >= a && count <= b
}

fn check_2(a: usize, b: usize, letter: char, value: &str) -> bool {
    let first = value.as_bytes()[a - 1] as char;
    let second = value.as_bytes()[b - 1] as char;
    (first == letter || second == letter) && first != second
}

fn validate(line: &str, predicate: Validator) -> Option<bool> {
    let mut parts = line.split(' ');
    let mut range = parts.next()?.split('-');
    let min = range.next()?.parse::<usize>().ok()?;
    let max = range.next()?.parse::<usize>().ok()?;
    let letter = parts.next()?.as_bytes()[0] as char;
    let password = parts.next()?;
    Some(predicate(min, max, letter, password))
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