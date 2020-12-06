mod prelude;
use prelude::*;

fn bsp(spec: &str) -> (usize, usize, usize) {
     let n = spec.chars().fold(0, |x, ch| {
         x << 1 | match ch {
            'F' | 'L' => 0,
             _ => 1
         }
     });
     (n >> 3, n & 7, n)
}

fn main() {
    let assignments = stdin().lock().lines()
        .filter_map(Result::ok)
        .map(|line| bsp(&line).2)
        .sorted()
        .collect_vec();

    let max_seat_id = assignments.last().unwrap();

    println!("{}", max_seat_id);

    let min_seat_id = assignments.first().unwrap();
    let seat_id = (min_seat_id+1..max_seat_id-1)
        .find(|id| assignments.binary_search(&id).is_err())
        .unwrap();

    println!("{}", seat_id);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(bsp("BFFFBBFRRR"), (70, 7, 567));
        assert_eq!(bsp("FFFBBBFRRR"), (14, 7, 119));
        assert_eq!(bsp("BBFFBBFRLL"), (102, 4, 820));
    }
}