mod prelude;
use prelude::*;

use std::f32::consts::PI;

const RADIANS: f32 = PI / 180.0;

fn parse_action(line: &str) -> (char, f32) {
    let direction = line.chars().next().unwrap();
    let units = line[1..].parse::<usize>().unwrap();
    (direction as char, units as f32)
}

fn rotate_around_origin(px: &mut f32, py: &mut f32, theta: f32) {
    let theta = theta * RADIANS;
    let cx = *px;
    let cy = *py;
    *px = cx * theta.cos() - cy * theta.sin();
    *py = cx * theta.sin() + cy * theta.cos();
}

fn distance(x: f32, y: f32) -> f32 {
    (x.abs() + y.abs()).ceil()
}

fn main() {
    let actions = stdin().lock().lines()
        .filter_map(Result::ok)
        .map(|line| parse_action(&line))
        .collect_vec();

    let mut ship_x = 0.0;
    let mut ship_y = 0.0;
    let mut heading = 90.0;

    for action in &actions {
        match action {
            ('N', v) => ship_y -= v,
            ('S', v) => ship_y += v,
            ('E', v) => ship_x += v,
            ('W', v) => ship_x -= v,
            ('R', v) => heading += v,
            ('L', v) => heading -= v,
            ('F', v) => {
                let theta: f32 = heading * RADIANS;
                ship_x += v * theta.sin();
                ship_y -= v * theta.cos();
            },
            _ => ()
        }
    }

    println!("{}", distance(ship_x, ship_y));

    ship_x = 0.0;
    ship_y = 0.0;

    let mut px = 10.0;
    let mut py = -1.0;

    for action in &actions {
        match action {
            ('N', v) => py -= v,
            ('S', v) => py += v,
            ('E', v) => px += v,
            ('W', v) => px -= v,
            ('R', v) => rotate_around_origin(&mut px, &mut py, *v),
            ('L', v) => rotate_around_origin(&mut px, &mut py, *v * -1.0),
            ('F', v) => {
                ship_x += px * v;
                ship_y += py * v;
            },
            _ => ()
        }
    }

    println!("{}", distance(ship_x, ship_y));
}