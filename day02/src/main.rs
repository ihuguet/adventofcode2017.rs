use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut csum = 0u32;
    let mut divsum = 0u32;
    
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines().map(|l| l.unwrap()) {
        let nums: Vec<u32> = line.split("\t")
                                 .map(|n| n.parse::<u32>().unwrap())
                                 .collect();
        
        let mut max = 0;
        let mut min = u32::MAX;
        let mut i = 0usize;
        for n in &nums {
            let n = *n;
            if n > max {max = n;}
            if n < min {min = n;}

            for m in &nums[(i+1)..] {
                if n % m == 0 {
                    divsum += n / m;
                } else if m % n == 0 {
                    divsum += m / n;
                }
            }
            i += 1;
        }

        csum += max - min;
    }

    println!{"Part1: csum={}", csum};
    println!{"Part2: divsum={}", divsum};
}
