use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

fn add(lhs: [u64; 256], rhs: [u64; 256]) -> [u64; 256] {
    let mut res = [0; 256];
    for i in 0..256 {
        res[i] = lhs[i] + rhs[i];
    }
    res
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/bin/aoc14.dat");
    let mut lines: Vec<String> = BufReader::new(File::open(path)?)
        .lines()
        .collect::<std::result::Result<_, _>>()?;
    let template = lines.remove(0).into_bytes();
    lines.remove(0);
    let rules: HashMap<[u8; 2], u8> = lines.into_iter().map(|line| parse_rule(&line)).collect();
    println!("Part 1: {}", compute(10, &template, &rules));
    println!("Part 2: {}", compute(40, &template, &rules));
    Ok(())
}

fn compute(levels: usize, template: &Vec<u8>, rules: &HashMap<[u8; 2], u8>) -> u64 {
    let mut stats = HashMap::new();
    let mut sum = (0..template.len() - 1)
        .map(|i| recurse([template[i], template[i + 1]], levels, rules, &mut stats))
        .fold([0; 256], add);

    for i in 1..template.len() - 1 {
        sum[template[i] as usize] -= 1;
    }

    sum.iter().max().unwrap() - sum.iter().filter(|&&n| n > 0).min().unwrap()
}

fn recurse(
    key: [u8; 2],
    level: usize,
    rules: &HashMap<[u8; 2], u8>,
    stats: &mut HashMap<([u8; 2], usize), [u64; 256]>,
) -> [u64; 256] {
    if level > 0 {
        match stats.get(&(key, level)) {
            Some(s) => *s,
            None => {
                let v = rules.get(&key).unwrap();
                let mut s = add(
                    recurse([key[0], *v], level - 1, rules, stats),
                    recurse([*v, key[1]], level - 1, rules, stats),
                );
                s[*v as usize] -= 1;
                stats.insert((key, level), s);
                s
            }
        }
    } else {
        let mut stats = [0; 256];
        for i in 0..2 {
            stats[key[i] as usize] += 1;
        }
        stats
    }
}

fn parse_rule(text: &str) -> ([u8; 2], u8) {
    let x: Vec<_> = text.split(" -> ").collect();
    let lhs = x[0].as_bytes();
    let rhs = x[1].as_bytes();
    (lhs[0..2].try_into().unwrap(), rhs[0])
}
