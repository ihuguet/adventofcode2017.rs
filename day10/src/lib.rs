const ROUNDS: usize = 64;

pub fn input_to_ints(input: &str) -> Vec<usize> {
    input.split(",").map(|s| s.parse::<usize>().unwrap()).collect()
}

pub fn input_to_ascii(input: &str) -> Vec<usize> {
    let mut lens: Vec<usize> = input.chars().map(|c| c as usize).collect();
    lens.extend(vec![17, 31, 73, 47, 23]);
    lens
}

pub fn apply_dense_knot_tie_hash(mut list: Vec<u32>, lens: &[usize]) -> String {
    apply_knot_tie_hash(&mut list, lens, ROUNDS);

    let mut list_out = Vec::with_capacity(16);

    let mut i = 0;
    while i < list.len() {
        let mut elem_out = 0;
        for &elem_in in &list[i..(i+16)] {
            elem_out ^= elem_in;
        }
        list_out.push(elem_out);
        i += 16;
    }

    list_out.iter()
            .map(|n| format!("{:02x}", n))
            .collect()
}

pub fn apply_knot_tie_hash(mut list: &mut Vec<u32>, lens: &[usize], mut rounds: usize) {
    let mut pos = 0;
    let mut skip = 0;

    while rounds > 0 {
        for &len in lens {
            reverse(&mut list, pos, len);
            pos = (pos + len + skip) % list.len();
            skip += 1;
        }
        rounds -= 1;
    }
}

fn reverse(list: &mut Vec<u32>, pos: usize, len: usize) {
    if len == 0 {return;}

    let mut i = pos;
    let mut j = pos + len - 1;
    let mut i_idx = i % list.len();
    let mut j_idx = j % list.len();

    while i < j {
        let tmp = list[i_idx];
        list[i_idx] = list[j_idx];
        list[j_idx] = tmp;

        i += 1;
        j -= 1;
        i_idx = if i_idx + 1 < list.len() { i_idx + 1 }
                else { 0 };
        j_idx = if j_idx > 0 { j_idx - 1 }
                else { list.len() - 1 };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse_normal() {
        reverse_test(vec![0,1,2,3,4], 0, 3, vec![2,1,0,3,4]);
    }

    #[test]
    fn reverse_cross_end_of_list() {
        reverse_test(vec![2,1,0,3,4], 3, 4, vec![4,3,0,1,2]);
    }

    #[test]
    fn reverse_len_0_or_1() {
        reverse_test(vec![0,1,2,3,4], 2, 1, vec![0,1,2,3,4]);
        reverse_test(vec![0,1,2,3,4], 2, 0, vec![0,1,2,3,4]);
    }

    fn reverse_test(mut list: Vec<u32>, pos: usize, len: usize, expects: Vec<u32>) {
        let original = list.clone();
        reverse(&mut list, pos, len);
        assert_eq!(&list, &expects, 
                   "Reverse of {:?} (pos={}, len={}) expected {:?}, got {:?}",
                    original, pos, len, expects, list);
    }

    #[test]
    fn hash_calculation_1_round() {
        let mut list = vec![0,1,2,3,4];
        let lens = [3,4,1,5];
        apply_knot_tie_hash(&mut list,  &lens, 1);
        assert_eq!(list, vec![3,4,2,1,0], "Hash calculation failed")
    }

    #[test]
    fn ints_input_parsing() {
        let len = input_to_ints("1,2,3");
        assert_eq!(len, vec![1,2,3]);
    }

    #[test]
    fn ascii_input_parsing() {
        let len = input_to_ascii("1,2,3");
        assert_eq!(len, vec![49,44,50,44,51,17,31,73,47,23]);
    }

    #[test]
    fn dense_hash() {
        dense_hash_test("", "a2582a3a0e66e6e86e3812dcb672a272");
        dense_hash_test("AoC 2017", "33efeb34ea91902bb2f59c9920caa6cd");
        dense_hash_test("1,2,3", "3efbe78a8d82f29979031a4aa0b16a9d");
        dense_hash_test("1,2,4", "63960835bcdc130f0b66d7ff4f6a5a8e");
    }

    fn dense_hash_test(ascii_input: &str, expects: &str) {
        let list = (0..256).collect();
        let lens = input_to_ascii(ascii_input);
        let hash = apply_dense_knot_tie_hash(list, &lens);
        assert_eq!(&hash, expects);
    }
}
