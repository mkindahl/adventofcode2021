use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/bin/aoc1.dat");
    let file = File::open(path)?;
    let input = BufReader::new(file);
    let nums: Vec<u32> = input
        .lines()
        .map(|line| line.unwrap().trim().parse().unwrap())
        .collect();

    // Part 1
    let increases: usize = nums
        .iter()
        .zip(nums.iter().skip(1))
        .map(|(x, y)| if x < y { 1 } else { 0 })
        .sum();
    println!("Part 1: {}", increases);

    // Part 2
    let sums: Vec<_> = (0..nums.len() - 2)
        .map(|i| nums[i] + nums[i + 1] + nums[i + 2])
        .collect();
    let result2: usize = sums
        .iter()
        .zip(sums.iter().skip(1))
        .map(|(x, y)| if x < y { 1 } else { 0 })
        .sum();

    println!("Part 2: {}", result2);

    Ok(())
}
