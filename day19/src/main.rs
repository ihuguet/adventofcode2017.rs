use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone,Copy)]
enum Dir {
    Up, Right, Down, Left
}

fn main() {
    let grid = parse_input("input.txt");

    let y0 = grid[0].iter().position(|&v| v == b'|').unwrap();
    let mut pos = (0, y0);
    let mut dir = Dir::Down;
    let mut path = String::new();
    let mut steps = 1; // entering to first position counts as step

    loop {
        pos = get_next_pos(pos, dir);
        match grid[pos.0][pos.1] {
            letter @ b'A'..=b'Z' => path.push(letter as char),
            b'+' => dir = get_next_dir(&grid, pos, dir),
            b' ' => break,
            _ => (),
        }
        steps += 1;
    }

    println!("Part 1: path={}", path);
    println!("Part 2: steps={}", steps);
}

fn get_next_pos(pos: (usize,usize), dir: Dir) -> (usize,usize) {
    match dir {
        Dir::Up    => (pos.0 - 1, pos.1),
        Dir::Right => (pos.0, pos.1 + 1),
        Dir::Down  => (pos.0 + 1, pos.1),
        Dir::Left  => (pos.0, pos.1 - 1),
    }
}

fn get_next_dir(grid: &Vec<Vec<u8>>, pos: (usize,usize), dir: Dir) -> Dir {
    match dir {
        Dir::Up | Dir::Down => {
            if pos.1 < grid[pos.0].len() - 1 && grid[pos.0][pos.1+1] != b' ' {
                Dir::Right
            } else if pos.1 > 0 && grid[pos.0][pos.1-1] != b' ' {
                Dir::Left
            } else {
                panic!("Must change to horizontal @ {:?} but both sides are empty", pos)
            }
        }
        Dir::Left | Dir::Right => {
            if pos.0 < grid.len() - 1 && grid[pos.0+1][pos.1] != b' ' {
                Dir::Down
            } else if pos.0 > 0 && grid[pos.0-1][pos.1] != b' ' {
                Dir::Up
            } else {
                panic!("Must change to vertical @ {:?} but both sides are empty", pos)
            }
        }
    }
}

fn parse_input(filename: &str) -> Vec<Vec<u8>> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut grid = Vec::new();
    for line in reader.lines() {
        grid.push(line.unwrap().into_bytes());
    }

    grid
}