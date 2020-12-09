mod prelude;
use prelude::*;

static PREAMBLE: usize = 25;

fn valid(data: &Vec<usize>, offset: usize, x: usize) -> bool {
    data.iter().skip(offset).take(PREAMBLE)
        .combinations(2)
        .map(|v| *v[0] + *v[1])
        .any(|v| v == x)
}

fn find_invalid(data: &Vec<usize>) -> Option<usize> {
    data.iter().skip(PREAMBLE)
        .zip(0..data.len() - PREAMBLE)
        .filter(|(&x, offset)| !valid(&data, *offset, x))
        .map(|(&x, _)| x)
        .next()
}

fn find_encryption_weakness(data: &Vec<usize>, invalid: usize) -> Option<usize> {
    (2..data.len())
        .flat_map(|width| data
            .windows(width)
            .filter(|window| window.iter().sum::<usize>() == invalid))
        .map(|x| {
            let min = x.iter().min().unwrap();
            let max = x.iter().max().unwrap();
            min + max
        })
        .next()
}

fn main() {
    let mut data = String::new();
    let _ = stdin().lock().read_to_string(&mut data);

    let n = data.lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect_vec();

    let invalid = find_invalid(&n).unwrap();
    println!("{:?}", invalid);

    let weakness = find_encryption_weakness(&n, invalid).unwrap();
    println!("{:?}", weakness);
}