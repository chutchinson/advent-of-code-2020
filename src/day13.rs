mod prelude;
use prelude::*;

fn parse(input: &str) -> Option<(usize, Vec<usize>)> {
    let mut lines = input.lines();
    let timestamp = lines.next().and_then(|v| v.parse::<usize>().ok())?;
    let ids = lines.next().map(|v| {
        v.split(",")
            .map(|text| { match text { "x" => 0, _ => text.parse::<usize>().unwrap() }})
            .collect_vec()
    })?;
    Some((timestamp, ids))
}

fn find_earliest_schedule(schedule: &Vec<(usize, usize)>, start: usize) -> usize {
    let mut time = start;
    loop {
        for (_, id) in schedule.iter() {
            if time % id == 0 {
                return id * (time - start);
            }
        }
        time += 1;
    }
}

fn find_earliest_timestamp(schedule: &Vec<(usize, usize)>) -> usize {
    let mut gap = schedule[0].1;
    let mut time = 0;

    for (i, id) in schedule.iter().skip(1) {
        while (time + i) % id != 0 {
            time += gap;
        }
        gap *= id;
    }

    time
}

fn main() {
    let mut input = String::new();
    let _ = stdin().lock().read_to_string(&mut input);

    let (timestamp, schedule) = parse(&input).unwrap();

    let mut busses = schedule.into_iter()
        .enumerate()
        .filter(|(_, id)| *id != 0)
        .collect_vec();

    let n = find_earliest_schedule(&busses, timestamp);
    println!("{:?}", n);

    let time = find_earliest_timestamp(&busses);
    println!("{}", time);
}