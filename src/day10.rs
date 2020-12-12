mod prelude;
use prelude::*;

fn differences(n: &[usize]) -> impl Iterator<Item=usize> + '_ {
    n[1..].iter().zip(n.iter()).map(|(a, b)| a - b)
}

fn main() {
    let mut data = String::new();
    let _ = stdin().lock().read_to_string(&mut data);
    let mut n = data.lines()
        .map(|line| line.parse::<usize>().unwrap())
        .sorted()
        .collect_vec();

    n.insert(0, 0);
    n.push(n[n.len() - 1] + 3);

    let ones = differences(&n).filter(|&v| v == 1).count();
    let threes = differences(&n).filter(|&v| v == 3).count();
    println!("{}", ones * threes);

    let mut counts = Vec::new();
    counts.push(1usize);

    for i in 1..n.len() {
        let mut ans = 0;
        for j in 0..i {
            if n[j] + 3 >= n[i] {
                ans += counts[j]
            }
        }
        counts.push(ans);
    }

    println!("{:?}", counts[counts.len()-1]);
}