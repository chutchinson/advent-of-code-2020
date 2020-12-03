mod prelude;
use prelude::*;

struct Grid  {
    width: usize,
    height: usize,
    cells: Vec<char>
}

fn parse<T: BufRead + Read>(read: &mut T) -> Option<Grid> {
    let mut cells = Vec::new();
    let mut height = 0;
    for line in read.lines() {
        cells.extend(line.ok()?.chars());
        height += 1;
    }
    Some(Grid {
        width: cells.len() / height,
        height,
        cells
    })
}

fn count(grid: &Grid, dx: usize, dy: usize) -> usize {
    let mut count = 0;
    let mut x = dx;
    let mut y = dy;
    while y < grid.height {
        count += if grid.cells[y * grid.width + x] == '#' { 1 } else { 0 };
        y += dy;
        x += dx;
        x %= grid.width;
    }
    count
}

fn solve(grid: &Grid, slopes: &[(usize, usize)]) -> usize {
    slopes.iter()
        .map(|&(dx, dy)| count(&grid, dx, dy))
        .product()
}

fn main() {
    let grid = parse(&mut std::io::stdin().lock()).expect("invalid grid");

    println!("{}", solve(&grid, &[(3,1)]));
    println!("{}", solve(&grid, &[(1,1), (3,1), (5,1), (7,1), (1,2)]));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn grid() -> Grid {
        let input = "\
            ..##.......\n\
            #...#...#..\n\
            .#....#..#.\n\
            ..#.#...#.#\n\
            .#...##..#.\n\
            ..#.##.....\n\
            .#.#.#....#\n\
            .#........#\n\
            #.##...#...\n\
            #...##....#\n\
            .#..#...#.#\n";
        parse(&mut input.as_bytes()).unwrap()
    }

    #[test]
    fn example_1() {
        let g = grid();
        let count = count(&g, 3, 1);
        assert_eq!(7, count);
    }

    #[test]
    fn example_2() {
        let g = grid();
        let slopes = [(1,1),(3,1),(5,1),(7,1),(1,2)];
        let counts = slopes.iter().map(|&(dx, dy)| count(&g, dx, dy)).collect_vec();
        assert_eq!(counts, [2,7,3,4,2]);
        assert_eq!(counts.iter().product::<usize>(), 336);
    }
}
