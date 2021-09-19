use std::collections::VecDeque;

const INPUT_A: u64 = 873;
const INPUT_B: u64 = 583;
const FACTOR_A: u64 = 16807;
const FACTOR_B: u64 = 48271;
const DIVISOR: u64 = 2147483647;

fn main() {
    let count = part1();
    println!("Part 1: count={}", count);

    let count = part2();
    println!("Part 2: count={}", count);
}

fn part1() -> u64 {
    let mut count = 0;
    let mut a = INPUT_A;
    let mut b = INPUT_B;

    for _ in 0..40_000_000 {
        a = a * FACTOR_A;
        a = a % DIVISOR;
        b = b * FACTOR_B;
        b = b % DIVISOR;
        if a & 0xFFFF == b & 0xFFFF {
            count += 1;
        }
    }

    count
}

fn part2() -> u64 {
    let mut queue_a = VecDeque::new();
    let mut queue_b = VecDeque::new();
    let mut judged_count = 0;
    let mut ok_count = 0;
    let mut a = INPUT_A;
    let mut b = INPUT_B;

    while judged_count < 5_000_000 {
        a = a * FACTOR_A;
        a = a % DIVISOR;
        b = b * FACTOR_B;
        b = b % DIVISOR;

        if a % 4 == 0 {
            queue_a.push_back(a);
        }
        if b % 8 == 0 {
            queue_b.push_back(b);
        }

        if queue_a.len() > 0 && queue_b.len() > 0 {
            judged_count += 1;
            let judge_a = queue_a.pop_front().unwrap();
            let judge_b = queue_b.pop_front().unwrap();
            if judge_a & 0xFFFF == judge_b & 0xFFFF {
                ok_count += 1;
            }
        }
    }

    ok_count
}