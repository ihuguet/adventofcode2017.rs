use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut jumps: Vec<i32> = reader.lines()
                                .map(|l| l.unwrap().parse::<i32>().unwrap())
                                .collect();

    let steps1 = solve(&mut jumps.clone(), |n| n + 1);
    let steps2 = solve(&mut jumps, |n| if n < 3 {n + 1} else {n - 1});

    println!("Part 1: steps={}", steps1);
    println!("Part 2: steps={}", steps2);
}

fn solve<F>(jumps: &mut Vec<i32>, line_modif: F) -> u32
where F: Fn(i32) -> i32
{
    let mut pos = 0;
    let mut steps = 0;

    while pos < jumps.len() {
        let jump = jumps[pos];
        jumps[pos] = line_modif(jump);
        steps += 1;
        if jump >= 0 {
            pos += jump as usize;
        } else {
            let jump = -jump as usize;
            assert!(jump <= pos);
            pos -= jump;
        }
    }

    steps
}
