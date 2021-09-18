
fn main() {
    let layers_lens = parse_layers_lens("input.txt");

    let penalty = calc_penalty_crossing(&layers_lens);
    println!("Part 1: penalty points={}", penalty);
    
    let delay = calc_min_required_delay(&layers_lens);
    println!("Part 2: delay={}", delay);
}

fn calc_penalty_crossing(layers_lens: &[usize]) -> u32 {
    let mut penalty_points = 0;

    for pos in 0..layers_lens.len() {
        let layer_len = layers_lens[pos];
        let arrival_time = pos as u32;
        let interval = calc_interval(layer_len);
        if packet_caught(arrival_time, interval) {
            penalty_points += (pos * layer_len) as u32;
        }
    }

    penalty_points
}

fn calc_min_required_delay(layers_lens: &[usize]) -> u32 {
    let mut delay = 0u32;

    loop {
        let mut caught = false;

        for (pos, &len) in layers_lens.iter().enumerate() {
            let interval = calc_interval(len);
            let arrival_time = delay + pos as u32;
            if packet_caught(arrival_time, interval) {
                caught = true;
                break;
            }
        }

        match caught {
            true  => delay += 1,
            false => break delay,
        }
    }
}

#[inline]
fn calc_interval(len: usize) -> Option<u32> {
    match len {
        0 => None,
        l => Some(2 * (l as u32 - 1)), 
    }
}

#[inline]
fn packet_caught(arrival_time: u32, interval: Option<u32>) -> bool {
    match interval {
        None => false,
        Some(0) => panic!(),
        Some(interval) => is_even_divisible(arrival_time, interval),
    }
}

#[inline]
fn is_even_divisible(dividend: u32, divisor: u32) -> bool {
    dividend == (dividend / divisor) * divisor
}


fn parse_layers_lens(filename: &str) -> Vec<usize> {
    let file_content = std::fs::read_to_string(filename).unwrap();

    let mut lens = Vec::new();
    for line in file_content.lines() {
        let (layer_num, layer_len) = parse_line(line);
        lens.extend(vec![0; layer_num - lens.len()]); // fill hole with 0s
        lens.push(layer_len);
    }

    lens
}

fn parse_line(line: &str) -> (usize, usize) {
    let mut split = line.split(": ");
    let num = split.next().unwrap().parse().unwrap();
    let len = split.next().unwrap().parse().unwrap();
    (num, len)
}
