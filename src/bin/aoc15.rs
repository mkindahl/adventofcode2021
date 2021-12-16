use std::{
    cmp::Ordering,
    collections::BinaryHeap,
    fs::File,
    io::{self, BufRead, BufReader},
    path::{Path, PathBuf},
};

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/bin/aoc15.dat");
    let mut map = Map::read(&path)?;
    let goal = (map.width - 1, map.height - 1);
    println!("Part 1: {:?}", least_risky_path(&mut map, (0, 0), goal));

    let mut big_map = Map::new();
    for r in 0..map.height {
        let mut row = Vec::new();
        for i in 0..5 {
            for c in 0..map.width {
                row.push((inc(map.map[r][c].0, i), usize::MAX));
            }
        }
        big_map.map.push(row);
    }

    for i in 1..5 {
        for r in 0..map.height {
            let row = big_map.map[r].iter().map(|r| (inc(r.0, i), r.1)).collect();
            big_map.map.push(row);
        }
    }
    big_map.height = 5 * map.height;
    big_map.width = 5 * map.width;

    let goal = (big_map.width - 1, big_map.height - 1);
    println!("Part 2: {:?}", least_risky_path(&mut big_map, (0, 0), goal));
    Ok(())
}

fn inc(r: usize, i: usize) -> usize {
    let mut ii = r + i;
    if ii > 9 {
        ii -= 9;
    }
    ii
}

fn least_risky_path(map: &mut Map, start: (usize, usize), goal: (usize, usize)) -> Option<usize> {
    let mut heap = BinaryHeap::new();

    heap.push(State {
        risk: 0,
        coord: start,
    });

    while let Some(State { risk, coord }) = heap.pop() {
        if coord == goal {
            return Some(risk);
        }

        if risk > map.map[coord.0][coord.1].1 {
            continue;
        }

        for (r, c) in map.adjecent(coord) {
            let next = State {
                risk: risk + r,
                coord: c,
            };
            if next.risk < map.map[c.0][c.1].1 {
                map.map[c.0][c.1].1 = next.risk;
                heap.push(next);
            }
        }
    }
    None
}

struct Map {
    width: usize,
    height: usize,
    map: Vec<Vec<(usize, usize)>>,
}

#[derive(Eq, PartialEq)]
struct State {
    risk: usize,
    coord: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .risk
            .cmp(&self.risk)
            .then_with(|| self.coord.cmp(&other.coord))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Map {
    fn new() -> Map {
        Map {
            width: 0,
            height: 0,
            map: Vec::new(),
        }
    }

    fn read(path: &Path) -> Result<Map, io::Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let map: Result<Vec<_>, _> = reader.split(0xA).collect();
        let map = map?;
        Ok(Map {
            width: map.iter().map(|v| v.len()).max().unwrap(),
            height: map.len(),
            map: map
                .iter()
                .map(|v| {
                    v.iter()
                        .map(|r| ((r - b'0') as usize, usize::MAX))
                        .collect()
                })
                .collect(),
        })
    }

    fn adjecent(&self, coord: (usize, usize)) -> Vec<(usize, (usize, usize))> {
        let mut adj = vec![];
        if coord.0 > 0 {
            adj.push((self.map[coord.0 - 1][coord.1].0, (coord.0 - 1, coord.1)));
        }
        if coord.1 > 0 {
            adj.push((self.map[coord.0][coord.1 - 1].0, (coord.0, coord.1 - 1)));
        }
        if coord.0 < self.height - 1 {
            adj.push((self.map[coord.0 + 1][coord.1].0, (coord.0 + 1, coord.1)));
        }
        if coord.1 < self.width - 1 {
            adj.push((self.map[coord.0][coord.1 + 1].0, (coord.0, coord.1 + 1)));
        }
        adj
    }
}
