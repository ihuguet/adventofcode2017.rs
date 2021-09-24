use std::collections::VecDeque;

const INPUT: usize = 329;

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut nums = VecDeque::with_capacity(2018);

    let mut pos = 0usize;
    nums.push_back(0);

    for n in 1..=2017 {
        pos = (pos + INPUT) % nums.len() + 1;
        nums.insert(pos, n);
    }

    let next = if pos < nums.len() - 1 {
                    nums[pos + 1]
               } else {
                    nums[0]
               };
               
    println!("Part 1: next={}", next);
}

// 0 will always stay at position 0, so we don't need to know all the values,
// but only the last one that is inserted after position 0
fn part2() {
    let mut nums_count = 1;
    let mut pos = 0;
    let mut num_after_0 = 0;

    while nums_count < 50000000 {
        pos += INPUT;

        if pos >= nums_count {
            pos = pos % nums_count;
        }

        if pos == 0 {
            num_after_0 = nums_count;
        }

        pos += 1;
        nums_count += 1;
    }

    println!("Part 2: next={}", num_after_0);
}
