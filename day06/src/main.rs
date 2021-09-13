use std::collections::HashMap;

const INPUT: &'static [u32] = &[4, 10, 4, 1, 8, 4, 9, 14, 5, 1, 14, 15, 0, 15, 3, 5];

fn main() {
    let mut banks = Vec::from(INPUT);
    let mut seen_states = HashMap::new();
    seen_states.insert(banks.clone(), 0);

    let mut steps = 0;
    let prev_steps: i32;

    loop {
        banks = redistribute_banks(banks);
        steps += 1;
        if let Some(v) = seen_states.insert(banks.clone(), steps) {
            prev_steps = v;
            break
        }
    }

    println!("Part 1: steps={}", steps);
    println!("Part 2: steps={}", steps - prev_steps);
}

fn redistribute_banks(mut banks: Vec<u32>) -> Vec<u32> {
    let mut idx = 0;
    let mut val = 0;

    for (i, v) in banks.iter().enumerate() {
        if *v > val {
            val = *v;
            idx = i;
        }
    }

    banks[idx] = 0;

    while val > 0 {
        idx += 1;
        if idx >= banks.len() {
            idx = 0
        }
        banks[idx] += 1;
        val -= 1;
    }

    banks
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn redistribute() {
        redistribute_test_one(vec![0, 2, 7, 0], vec![2, 4, 1, 2]);
        redistribute_test_one(vec![2, 4, 1, 2], vec![3, 1, 2, 3]);
        redistribute_test_one(vec![3, 1, 2, 3], vec![0, 2, 3, 4]);
        redistribute_test_one(vec![0, 2, 3, 4], vec![1, 3, 4, 1]);
        redistribute_test_one(vec![1, 3, 4, 1], vec![2, 4, 1, 2]);
    }

    fn redistribute_test_one(input: Vec<u32>, expects: Vec<u32>) {
        let result = redistribute_banks(input.clone());
        assert_eq!(result, expects, "Redistribution of bank {:?} expected {:?}, got {:?}",
            input, expects, result);
    }
}