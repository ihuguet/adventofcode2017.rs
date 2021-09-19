use day10;

const INPUT: &str = "xlqgujun";

fn main() {
    let mut grid = Vec::with_capacity(128);

    for row in 0..128 {
        let hash = calc_hash_from_input(&format!("{}-{}", INPUT, row));
        let hash = ascii_hash_to_u128(&hash).unwrap();
        grid.push(hash);
    }

    let used = count_used_squares(&grid);
    println!("Part 1: used={}", used);

    let regions = count_regions(grid);
    println!("Part 2: regions={}", regions);
}

fn calc_hash_from_input(hash_input: &str) -> String {
    let nums_list = (0..=255).collect();
    let lens = day10::input_to_ascii(hash_input);
    day10::apply_dense_knot_tie_hash(nums_list, &lens)
}

fn ascii_hash_to_u128(hash: &str) -> Result<u128,std::num::ParseIntError>{
    assert_eq!(hash.len(), 32);
    u128::from_str_radix(hash, 16)
}

fn count_used_squares(grid: &[u128]) -> usize {
    let mut count = 0;
    
    for mut hash in grid.iter().copied() {
        for _ in 0..128 {
            if hash & 1 == 1 {
                count += 1;
            }
            hash >>= 1;
        }
    }

    count
}

fn count_regions(mut grid: Vec<u128>) -> usize {
    let mut count = 0;

    for row in 0..grid.len() {
        for col in 0..128 {
            if bit_is_set(grid[row], col) {
                count += 1;
                clear_bits_in_region(&mut grid, row, col);
            }
        }
    }

    count
}

fn clear_bits_in_region(grid: &mut Vec<u128>, row: usize, col:usize) {
    if !bit_is_set(grid[row], col) {
        return;
    }
    
    bit_clear(&mut grid[row], col);
    let mut queue = vec![(row, col)];

    while queue.len() > 0 {
        let (row, col) = queue.pop().unwrap();
        
        if row > 0 && bit_is_set(grid[row-1], col) {
            bit_clear(&mut grid[row-1], col);
            queue.push((row-1, col));
        }
        if row < grid.len()-1 && bit_is_set(grid[row+1], col) {
            bit_clear(&mut grid[row+1], col);
            queue.push((row+1, col));
        }
        if col > 0 && bit_is_set(grid[row], col-1) {
            bit_clear(&mut grid[row], col-1);
            queue.push((row, col-1));
        }
        if col < 127 && bit_is_set(grid[row], col+1) {
            bit_clear(&mut grid[row], col+1);
            queue.push((row, col+1));
        }
    }
}

#[inline]
fn bit_is_set(hash: u128, bit: usize) -> bool{
    (hash >> bit) & 1 == 1
}

#[inline]
fn bit_clear(hash: &mut u128, bit: usize) {
    *hash &= !(1u128 << bit);
}