use crate::prelude::*;

fn find_expense(expenses: &Vec<i32>, size: usize) -> i32 {
    const SUM: i32 = 2020;
    let mut combinations = expenses.iter().combinations(size);
    let pair = combinations.find(|v| v.iter().fold(0, |sum, &v| sum + v) == SUM).unwrap();
    pair.iter().fold(1, |x, &v| x * v)
}

pub fn solve(ctx: &mut Context) -> Result {
    let expenses = ctx.load("1.txt")?
        .lines()
        .map(|line| line.unwrap())
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let n = find_expense(&expenses, 2);
    writeln!(ctx, "{}", n);

    let n = find_expense(&expenses, 3);
    writeln!(ctx, "{}", n);

    Ok(())
}

#[cfg(test)]
pub mod tests {
    use super::find_expense;

    #[test]
    pub fn sample_1() {
        let expenses = vec![1721, 979, 366, 299, 675, 1456];
        let expense = find_expense(&expenses, 2);
        assert_eq!(expense, 514579);
    }
}