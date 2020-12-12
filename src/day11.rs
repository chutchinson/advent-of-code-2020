mod prelude;
use prelude::*;

type Predicate = fn(&Grid, px: isize, py: isize, dx: isize, dy: isize) -> bool;

fn index2d(x: isize, y: isize, width: isize) -> usize {
    let index = y * width + x;
    index as usize
}

fn neighbors(grid: &Grid, px: isize, py: isize, dx: isize, dy: isize) -> bool {
    grid.get(px + dx, py + dy) == Some('#')
}

fn raycast(grid: &Grid, x: isize, y: isize, dx: isize, dy: isize) -> bool {
    let mut px = x + dx;
    let mut py = y + dy;
    loop {
        match grid.get(px, py) {
            None => return false,
            Some('L') => return false,
            Some('#') => return true,
            _ => ()
        }
        px += dx;
        py += dy;
    }
}

#[derive(Clone, PartialEq)]
struct Grid {
    data: Vec<char>,
    width: isize,
    height: isize,
}

impl Grid {
    pub fn contains(&self, x: isize, y: isize) -> bool {
        x >= 0 && y >= 0 && x < self.width && y < self.height
    }
    pub fn get(&self, x: isize, y: isize) -> Option<char> {
        if self.contains(x, y) { Some(self.data[index2d(x, y, self.width)]) } else { None }
    }
    pub fn get_mut(&mut self, x: isize, y: isize) -> Option<&mut char> {
        if self.contains(x, y) { Some(&mut self.data[index2d(x, y, self.width)]) } else { None }
    }
    pub fn adjacent(&self, x: isize, y: isize, f: Predicate) -> usize {
        let directions: [(isize, isize); 8] = [
            (-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)
        ];
        directions
            .iter()
            .filter(|(dx, dy)| f(&self, x, y, *dx, *dy))
            .count()
    }
}

fn solve(grid: &Grid, f: Predicate, n: usize) -> usize {
    let mut last_grid = grid.clone();
    let mut count = 0;

    loop {
        let grid = last_grid.clone();

        for y in 0..grid.height {
            for x in 0..grid.width {
                let seat = last_grid.get_mut(x, y).unwrap();
                match *seat {
                    'L' => if grid.adjacent(x, y, f) == 0 { *seat = '#'; count += 1 },
                    '#' => if grid.adjacent(x, y, f) >= n { *seat = 'L'; count -= 1 },
                    _ => ()
                }
            }
        }

        if grid == last_grid {
            break count
        }
    }
}

fn main() {
    let mut grid = Grid {
        data: Vec::new(),
        width: 0,
        height: 0
    };

    for line in stdin().lock().lines().filter_map(Result::ok) {
        grid.data.extend(line.chars());
        grid.width = line.len() as isize;
        grid.height += 1;
    }

    let count = solve(&grid, neighbors, 4);
    println!("{}", count);

    let count = solve(&grid, raycast, 5);
    println!("{}", count);
}