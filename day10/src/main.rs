use day10::*;

const INPUT: &str = "106,16,254,226,55,2,1,166,177,247,93,0,255,228,60,36";

fn main() {
    let mut list: Vec<u32> = (0..256).collect();
    let lens: Vec<usize> = input_to_ints(INPUT);
    apply_knot_tie_hash(&mut list, &lens, 1);
    println!("Part1: list[0]*list[1]={}", list[0]*list[1]);

    list = (0..256).collect();
    let lens = input_to_ascii(INPUT);
    let hex = apply_dense_knot_tie_hash(list, &lens);
    println!("Part2: hex={}", hex);
}
