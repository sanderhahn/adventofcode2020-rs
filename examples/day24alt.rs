use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Point {
    x: isize,
    y: isize,
}

// Szudzik Elegant Pairing (not used)
// http://www.szudzik.com/ElegantPairing.pdf

impl Point {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }
    fn move_ne(&mut self) {
        self.y += 1;
    }
    fn move_e(&mut self) {
        self.x += 1;
        self.y += 1;
    }
    fn move_se(&mut self) {
        self.x += 1;
    }
    fn move_sw(&mut self) {
        self.y -= 1
    }
    fn move_w(&mut self) {
        self.x -= 1;
        self.y -= 1;
    }
    fn move_nw(&mut self) {
        self.x -= 1;
    }
    fn neighbors(&self) -> Neighbors {
        Neighbors::new(self)
    }
}

struct Neighbors {
    point: Point,
    direction: usize,
}

impl Neighbors {
    fn new(point: &Point) -> Self {
        Self {
            point: point.clone(),
            direction: 0
        }
    }
}

impl Iterator for Neighbors {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        match self.direction {
            0 => {
                self.point.move_ne()
            }
            1 => {
                self.point.move_se()
            }
            2 => {
                self.point.move_sw()
            }
            3 => {
                self.point.move_w()
            }
            4 => {
                self.point.move_nw()
            }
            5 => {
                self.point.move_ne()
            }
            _ => {
                return None
            }
        }
        self.direction += 1;
        Some(self.point)
    }
}

type Grid = HashSet<Point>;

struct PrefixParser<'s> {
    rest: &'s str,
}

impl<'s> PrefixParser<'s> {
    fn new(rest: &'s str) -> Self {
        Self { rest }
    }
    fn parse(&mut self, prefix: &str) -> bool {
        if self.rest.starts_with(prefix) {
            self.rest = &self.rest[prefix.len()..];
            true
        } else {
            false
        }
    }
    fn done(&self) -> bool {
        self.rest == ""
    }
}

fn grid(input: &str) -> Grid {
    let mut grid = Grid::new();
    for line in input.lines() {
        let mut point = Point::new();
        let mut pp = PrefixParser::new(line);

        while !pp.done() {
            if pp.parse("ne") {
                point.move_ne();
            } else if pp.parse("e") {
                point.move_e();
            } else if pp.parse("se") {
                point.move_se();
            } else if pp.parse("sw") {
                point.move_sw();
            } else if pp.parse("w") {
                point.move_w();
            } else if pp.parse("nw") {
                point.move_nw();
            } else {
                panic!("error {}", pp.rest);
            }
        }
        if grid.contains(&point) {
            grid.remove(&point);
        } else {
            grid.insert(point);
        }
    }
    grid
}

fn day24a(input: &str) -> usize {
    grid(input).len()
}

fn count(grid: &Grid, point: &Point) -> usize {
    let mut count = 0;
    for neighbor in point.neighbors() {
        if grid.contains(&neighbor) {
            count += 1;
        }
    }
    count
}

fn all_whites(grid: &Grid) -> Grid {
    let mut whites = Grid::new();
    for point in grid {
        for neighbor in point.neighbors() {
            if !grid.contains(&neighbor) {
                whites.insert(neighbor);
            }
        }
    }
    whites
}

fn flip(grid: Grid) -> Grid {
    let mut new_grid = Grid::new();
    for point in grid.iter() {
        let count = count(&grid, &point);
        if count == 0 || count > 2 {
            new_grid.remove(&point);
        } else {
            new_grid.insert(*point);
        }
    }
    for point in all_whites(&grid) {
        if count(&grid, &point) == 2 {
            new_grid.insert(point);
        }
    }
    new_grid
}

fn day24b(input: &str) -> usize {
    let mut grid = grid(input);
    for _ in 0..100 {
        grid = flip(grid);
    }
    grid.len()
}

fn main() {
    let example = "\
sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";

    assert_eq!(0, day24a("ew\nswenw"));
    assert_eq!(10, day24a(example));

    let input = include_str!("../inputs/day24.txt");
    println!("{}", day24a(input));

    assert_eq!(6, all_whites(&grid("ew")).len());
    assert_eq!(8, all_whites(&grid("ew\nse")).len());
    assert_eq!(11, all_whites(&grid("ew\nsese")).len());

    assert_eq!(2208, day24b(example));
    println!("{}", day24b(input));
}
