type Cubes = Vec<Vec<Vec<bool>>>;
type HyperCubes = Vec<Cubes>;

fn expand(cubes: &mut Cubes) {
    for plane in cubes.into_iter() {
        for row in plane.iter_mut() {
            row.insert(0, false);
            row.push(false);
        }
        plane.insert(0, vec![false; plane[0].len()]);
        plane.push(vec![false; plane[0].len()]);
    }
    let height = cubes[0].len();
    let width = cubes[0][0].len();
    let plane = vec![vec![false; width]; height];
    cubes.insert(0, plane.clone());
    cubes.push(plane);
}

pub fn debug(cycle: i32, cubes: &Cubes) {
    if cycle == 0 {
        println!("Before any cycles:\n")
    } else {
        let es = if cycle > 1 { "s" } else { "" };
        println!("After {} cycle{}:\n", cycle, es);
    }

    for z in 0..cubes.len() {
        let fake_z = (z as isize) - (cubes.len() >> 1) as isize;
        println!("z={}", fake_z);

        for line in &cubes[z] {
            for &b in line {
                if b {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
        println!("\n");
    }
}

fn neigbors(x: usize, y: usize, z: usize, cubes: &Cubes) -> usize {
    let width = cubes[0][0].len();
    let height = cubes[0].len();
    let depth = cubes.len();
    let mut neighbors = 0;
    for dx in 0..=2 {
        for dy in 0..=2 {
            for dz in 0..=2 {
                if dx == 1 && dy == 1 && dz == 1 {
                    continue;
                }
                let x = (x + dx + width - 1) % width;
                let y = (y + dy + height - 1) % height;
                let z = (z + dz + depth - 1) % depth;
                if cubes[z][y][x] {
                    neighbors += 1;
                }
            }
        }
    }
    neighbors
}

fn mutate(cubes: &Cubes) -> Cubes {
    let mut new_cubes = cubes.clone();
    for z in 0..cubes.len() {
        for y in 0..cubes[0].len() {
            for x in 0..cubes[0][0].len() {
                let n = neigbors(x, y, z, cubes);
                let r = cubes[z][y][x];
                let w = &mut new_cubes[z][y][x];
                if r {
                    *w = n >= 2 && n <= 3;
                } else {
                    if n == 3 {
                        *w = !*w;
                    }
                }
            }
        }
    }
    new_cubes
}

fn count(cubes: &Cubes) -> usize {
    cubes
        .iter()
        .map(|plane| {
            plane
                .iter()
                .map(|line| line.iter().map(|&b| if b { 1 } else { 0 }).sum::<usize>())
                .sum::<usize>()
        })
        .sum::<usize>()
}

fn day17(input: &str) -> usize {
    let mut cubes: Cubes = vec![input
        .lines()
        .map(|line| line.as_bytes().iter().map(|&c| c == b'#').collect())
        .collect()];

    for _cycle in 0..6 {
        // debug(_cycle, &cubes);
        expand(&mut cubes);
        cubes = mutate(&cubes);
    }
    count(&cubes)
}

fn hyper_expand(cubes: &mut HyperCubes) {
    for cubes in cubes.into_iter() {
        expand(cubes);
    }
    let depth = cubes[0].len();
    let height = cubes[0][0].len();
    let width = cubes[0][0][0].len();
    let cube = vec![vec![vec![false; width]; height]; depth];
    cubes.insert(0, cube.clone());
    cubes.push(cube);
}

fn hyper_neighbors(w: usize, x: usize, y: usize, z: usize, cubes: &HyperCubes) -> usize {
    let width = cubes[0][0][0].len();
    let height = cubes[0][0].len();
    let depth = cubes[0].len();
    let hyperdepth = cubes.len();
    let mut neighbors = 0;
    for dw in 0..=2 {
        for dx in 0..=2 {
            for dy in 0..=2 {
                for dz in 0..=2 {
                    if dw == 1 && dx == 1 && dy == 1 && dz == 1 {
                        continue;
                    }
                    let w = (w + dw + hyperdepth - 1) % hyperdepth;
                    let x = (x + dx + width - 1) % width;
                    let y = (y + dy + height - 1) % height;
                    let z = (z + dz + depth - 1) % depth;
                    if cubes[w][z][y][x] {
                        neighbors += 1;
                    }
                }
            }
        }
    }
    neighbors
}

fn hyper_mutate(cubes: &HyperCubes) -> HyperCubes {
    let mut new_cubes = cubes.clone();
    for w in 0..cubes.len() {
        for z in 0..cubes[0].len() {
            for y in 0..cubes[0][0].len() {
                for x in 0..cubes[0][0][0].len() {
                    let n = hyper_neighbors(w, x, y, z, cubes);
                    let r = cubes[w][z][y][x];
                    let w = &mut new_cubes[w][z][y][x];
                    if r {
                        *w = n >= 2 && n <= 3;
                    } else {
                        if n == 3 {
                            *w = !*w;
                        }
                    }
                }
            }
        }
    }
    new_cubes
}

fn hyper_count(cubes: &HyperCubes) -> usize {
    cubes.iter().map(|cubes| count(cubes)).sum()
}

fn day17b(input: &str) -> usize {
    let mut cubes: HyperCubes = vec![vec![input
        .lines()
        .map(|line| line.as_bytes().iter().map(|&c| c == b'#').collect())
        .collect()]];

    for _ in 0..6 {
        hyper_expand(&mut cubes);
        cubes = hyper_mutate(&cubes);
    }
    hyper_count(&cubes)
}

fn main() {
    let example = "\
.#.
..#
###";
    assert_eq!(112, day17(example));

    let input = include_str!("../inputs/day17.txt");
    println!("{}", day17(input));

    assert_eq!(848, day17b(example));
    println!("{}", day17b(input));
}
