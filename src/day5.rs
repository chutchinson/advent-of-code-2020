mod prelude;
use prelude::*;

fn bsp(range: Range<usize>, spec: &str) -> usize {
     spec.chars()
        .fold(range, |x, ch| {
            let n = (x.end - x.start) / 2;
            match ch {
                'F' | 'L' => x.start..x.end - n - 1,
                'B' | 'R' => x.start + n + 1..x.end,
                _ => x
            }
        })
        .start
}

fn parse(spec: &str) -> (usize, usize, usize) {
    let row = bsp(0..127, &spec[0..7]);
    let col = bsp(0..7, &spec[7..]);
    (row, col, row * 8 + col)
}

fn main() {
    let rows = stdin().lock()
        .lines()
        .filter_map(Result::ok)
        .collect_vec();

    let assignments = rows.iter()
        .map(|line| parse(line).2)
        .sorted()
        .collect_vec();

    let max_seat_id = *assignments.last().unwrap();

    println!("{}", max_seat_id);

    let min_seat_id = assignments.first().unwrap();
    let seat_id = (min_seat_id+1..max_seat_id-1)
        .find(|id| !assignments.contains(id))
        .unwrap();

    println!("{}", seat_id);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(parse("BFFFBBFRRR"), (70, 7, 567));
        assert_eq!(parse("FFFBBBFRRR"), (14, 7, 119));
        assert_eq!(parse("BBFFBBFRLL"), (102, 4, 820));
    }
}