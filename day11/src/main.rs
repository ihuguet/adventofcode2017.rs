use std::fs;
use std::error::Error;
use std::cmp;

/// Position in hexagonal grid can be indicated by 2 axes coordinates, but with
/// one of the axes in angle. Then, of the 6 posibles direction of one hexagon,
/// 4 are aligned with the axes and 2 are not. A movement in one of those 2 not
/// aligned directions modify both X and Y coordinates (see function
/// hexagonal_move)
/// 
///       Y       X
///       |      /
///       |     /
///      ---   /
///    /     \ 
///    \     /
///      ---
/// 
fn main() -> Result<(),Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    let movs: Vec<&str> =  input.split(",").collect();
    
    let mut pos = (0, 0);
    let mut max_dist = 0;

    for mov in movs {
        pos = hexagonal_move(&pos, mov);
        let dist = calc_distance(&pos);
        if dist > max_dist { max_dist = dist; }
    }

    let final_dist = calc_distance(&pos);

    println!("Part 1: final distance={}", final_dist);
    println!("Part 2: max. distance={}", max_dist);

    Ok(())
}

fn hexagonal_move(pos: &(i32, i32), mov: &str) -> (i32, i32) {
    match mov {
        "n"  => (pos.0,     pos.1 + 1),
        "nw" => (pos.0 - 1, pos.1 + 1),
        "sw" => (pos.0 - 1, pos.1),
        "s"  => (pos.0,     pos.1 - 1),
        "se" => (pos.0 + 1, pos.1 - 1),
        "ne" => (pos.0 + 1, pos.1),
        _ => panic!(),
    }
}

fn calc_distance(pos: &(i32, i32)) -> u32 {
    let (mut x, mut y) = pos;
    let mut steps = 0;

    if x < 0 && y > 0 { // nw
        steps += cmp::min(i32::abs(x), i32::abs(y));
        x += steps;
        y -= steps;
    } else if x > 0 && y < 0 { // se
        steps += cmp::min(i32::abs(x), i32::abs(y));
        x -= steps;
        y += steps;
    }
    steps += i32::abs(x) + i32::abs(y); // n, s, ne, sw

    steps as u32
}
