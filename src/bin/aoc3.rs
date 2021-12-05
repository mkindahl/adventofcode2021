use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

fn part1(vec: &Vec<Vec<u8>>, i: usize, v: usize) -> usize {
    let n = vec.len();
    if i < vec[0].len() {
        let c = vec.iter().filter(|v| v[i] == b'1').count();
        let b = if 2 * c > n { 1 } else { 0 };
        part1(vec, i + 1, (v << 1) | b)
    } else {
        v * (!v & ((1usize << i) - 1))
    }
}

fn to_num(vec: &Vec<u8>) -> usize {
    vec.iter()
        .fold(0, |a, b| (a << 1) | if *b == b'1' { 1 } else { 0 })
}

fn process(vec: Vec<Vec<u8>>, i: usize, o: u8, z: u8) -> usize {
    if vec.len() == 1 {
        to_num(&vec[0])
    } else {
        let n = vec.len();
        let c = vec.iter().filter(|v| v[i] == b'1').count();
        let b = if c >= (n - c) { o } else { z };
        let nvec: Vec<_> = vec.into_iter().filter(|v| v[i] == b).collect();
        process(nvec, i + 1, o, z)
    }
}

fn part2(vec: &Vec<Vec<u8>>) -> usize {
    let ox = process(vec.clone(), 0, b'1', b'0');
    let sc = process(vec.clone(), 0, b'0', b'1');
    ox * sc
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/bin/aoc3.dat");
    let file = File::open(path)?;
    let input = BufReader::new(file);
    let report: Result<Vec<_>, _> = input.lines().collect();
    if let Ok(lines) = report {
        let lines: Vec<_> = lines.into_iter().map(|line| line.into_bytes()).collect();
        println!("Part 1: {}", part1(&lines, 0, 0));
        println!("Part 2: {}", part2(&lines));
    }
    Ok(())
}
