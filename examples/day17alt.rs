use std::collections::HashSet;
use std::iter::FromIterator;

type Point = Vec<isize>;
type HashCube = HashSet<Point>;

fn parse(input: &str, ndim: usize) -> HashCube {
    let mut grid = HashSet::new();
    for (y, row) in input.lines().enumerate() {
        for (x, &b) in row.as_bytes().iter().enumerate() {
            if b == b'#' {
                let mut point: Point = vec![0; ndim];
                point[0] = x as isize;
                point[1] = y as isize;
                grid.insert(point);
            }
        }
    }
    grid
}

fn neighbors(point: &Point) -> HashCube {
    let mut points = vec![point.clone()];
    for n in 0..point.len() {
        for i in 0..points.len() {
            for d in 0..=2 {
                let mut new_point = points[i].clone();
                new_point[n] += d-1;
                points.push(new_point);
            }
        }
    }
    HashSet::from_iter(points)
}

fn count(point: &Point, grid: &HashCube) -> usize {
    let mut neighbors = neighbors(point);
    neighbors.remove(point);
    grid.intersection(&neighbors).collect::<HashSet<_>>().len()
}

fn eval(set: bool, neighbors: usize) -> bool {
    if set {
        neighbors >= 2 && neighbors <= 3
    } else {
        if neighbors == 3 {
            !set
        } else {
            false
        }
    }
}

fn all_neighbors(grid: &HashCube) -> HashCube {
    let mut all_neighbors: HashCube = grid.clone();
    for point in grid.into_iter() {
        all_neighbors = all_neighbors.union(&neighbors(&point)).cloned().collect();
    }
    all_neighbors
}

fn mutate(grid: &HashCube) -> HashCube {
    let mut new_grid = HashSet::new();

    for point in all_neighbors(grid).iter() {
        let n = count(point, grid);
        if eval(grid.contains(point), n) {
            new_grid.insert(point.clone());
        }
    }

    new_grid
}

fn day17(input: &str, ndim: usize) -> usize {
    let mut grid = parse(input, ndim);
    for _cycle in 0..6 {
        grid = mutate(&grid);
    }
    grid.len()
}

fn main() {
    let example = "\
.#.
..#
###";
    assert_eq!(112, day17(example, 3));

    let input = include_str!("../inputs/day17.txt");
    println!("{}", day17(input, 3));

    assert_eq!(848, day17(example, 4));
    println!("{}", day17(input, 4));
}
