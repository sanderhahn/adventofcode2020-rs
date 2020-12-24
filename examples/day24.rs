use std::collections::HashSet;

type Coord = (isize, isize);
type Grid = HashSet<Coord>;

fn grid(input: &str) -> Grid {
    let mut grid = Grid::new();
    for line in input.lines() {
        let mut row = 0isize;
        let mut col = 0isize;
        let mut partial = false;
        for char in line.chars() {
            match char {
                's' => {
                    row += 1;
                    partial = true;
                }
                'n' => {
                    row -= 1;
                    partial = true;
                }
                'e' => {
                    col -= if partial {
                        partial = false;
                        1
                    } else {
                        2
                    };
                }
                'w' => {
                    col += if partial {
                        partial = false;
                        1
                    } else {
                        2
                    };
                }
                _ => {
                    panic!("error");
                }
            }
        }
        let coord =  (col, row);
        if grid.contains(&coord) {
            grid.remove(&coord);
        } else {
            grid.insert(coord);
        }
    }
    grid
}

fn day24a(input: &str) -> usize {
    grid(input).len()
}

fn count(grid: &Grid, &(col, row): &Coord) -> usize {
    let mut count = 0;
    let w = (col+2, row);
    let e = (col-2, row);
    let se = (col+1, row+1);
    let sw = (col-1, row+1);
    let ne = (col+1, row-1);
    let nw = (col-1, row-1);
    if grid.contains(&w) {
        count += 1;
    }
    if grid.contains(&e) {
        count += 1;
    }
    if grid.contains(&se) {
        count += 1;
    }
    if grid.contains(&sw) {
        count += 1;
    }
    if grid.contains(&ne) {
        count += 1;
    }
    if grid.contains(&nw) {
        count += 1;
    }
    count
}

fn all_whites(grid: &Grid) -> Grid {
    let mut whites = Grid::new();
    for &(col, row) in grid.iter() {
        let w = (col+2, row);
        let e = (col-2, row);
        let se = (col+1, row+1);
        let sw = (col-1, row+1);
        let ne = (col+1, row-1);
        let nw = (col-1, row-1);
        if !grid.contains(&w) {
            whites.insert(w);
        }
        if !grid.contains(&e) {
            whites.insert(e);
        }
        if !grid.contains(&se) {
            whites.insert(se);
        }
        if !grid.contains(&sw) {
            whites.insert(sw);
        }
        if !grid.contains(&ne) {
            whites.insert(ne);
        }
        if !grid.contains(&nw) {
            whites.insert(nw);
        }
    }
    whites
}

fn flip(grid: Grid) -> Grid {
    let mut new_grid = grid.clone();
    for coord in grid.iter() {
        let count = count(&grid, &coord);
        if count == 0 || count > 2 {
            new_grid.remove(&coord);
        }
    }
    for coord in all_whites(&grid) {
        if count(&grid, &coord) == 2 {
            new_grid.insert(coord);
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

    assert_eq!(6, all_whites(&grid("ns")).len());
    assert_eq!(8, all_whites(&grid("ns\nnw")).len());
    assert_eq!(11, all_whites(&grid("ns\nnwnw")).len());

    assert_eq!(0, count(&grid("ns"), &(0, 0)));
    assert_eq!(1, count(&grid("ns\nnw"), &(0, 0)));
    assert_eq!(6, count(&grid("ns\nne\ne\nse\nsw\nw\nnw"), &(0, 0)));

    assert_eq!(2208, day24b(example));
    println!("{}", day24b(input));
}
