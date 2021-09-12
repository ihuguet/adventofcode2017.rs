use std::fmt::{self, Display};
use std::collections::BTreeMap;

const INPUT: i32 = 347991;

enum Dir {
    Right, Up, Left, Down,
}

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut x_min = 0;
    let mut x_max = 0;
    let mut y_min = 0;
    let mut y_max = 0;
    let mut pos = Point{x: 0, y: 0};
    let mut num = 1;
    let mut dir = Dir::Right;

    while num < INPUT {
        let next_pos = match dir {
            Dir::Right => {
                x_max += 1;
                dir = Dir::Up;
                Point {x: x_max, y: pos.y}
            },
            Dir::Up => {
                y_max += 1;
                dir = Dir::Left;
                Point{x: pos.x, y: y_max }
            },
            Dir::Left => {
                x_min -= 1;
                dir = Dir::Down;
                Point{x: x_min, y: pos.y}
            },
            Dir::Down => {
                y_min -= 1;
                dir = Dir::Right;
                Point{x: pos.x, y: y_min}
            },
        };

        let next_sum = i32::abs(next_pos.x - pos.x + next_pos.y - pos.y);
        if num + next_sum >= INPUT {
            if next_pos.x > pos.x {pos.x += INPUT - num;}
            else if next_pos.x < pos.x {pos.x -= INPUT - num;}
            else if next_pos.y > pos.y {pos.y += INPUT - num;}
            else if next_pos.y < pos.y {pos.y -= INPUT - num;}
            num = INPUT;
        }
        else {
            pos = next_pos;
            num += next_sum;
        }
    }

    let dist = i32::abs(pos.x) + i32::abs(pos.y);
    println!("Part 1: pos={}, dist={}", pos, dist);
}

fn part2() {
    let mut grid = BTreeMap::new();
    grid.insert((0,0), 1);

    let mut x_min = 0;
    let mut x_max = 0;
    let mut y_min = 0;
    let mut y_max = 0;
    let mut pos = Point{x: 0, y: 0};
    let mut dir = Dir::Right;

    loop {
        match dir {
            Dir::Right => {
                pos.x += 1;
                if pos.x > x_max {
                    x_max = pos.x;
                    dir = Dir::Up;
                }
            },
            Dir::Up => {
                pos.y += 1;
                if pos.y > y_max {
                    y_max = pos.y;
                    dir = Dir::Left;
                }
            },
            Dir::Left => {
                pos.x -= 1;
                if pos.x < x_min {
                    x_min = pos.x;
                    dir = Dir::Down;
                }
            },
            Dir::Down => {
                pos.y -= 1;
                if pos.y < y_min {
                    y_min = pos.y;
                    dir = Dir::Right;
                }
            },
        };

        let val = fill_grid_cell(&mut grid, &pos);
        if val > INPUT {
            println!("Part2: pos={}, val={}", pos, val);
            break;
        }
    }
}

fn fill_grid_cell(grid: &mut BTreeMap<(i32,i32),i32>, pos: &Point) -> i32 {
    let mut sum = 0;
    for x in (pos.x-1)..(pos.x+2) {
        for y in (pos.y-1)..(pos.y+2) {
            if x == pos.x && y == pos.y {
                continue;
            }
            if let Some(val) = grid.get(&(x,y)) {
                sum += val;
            }
        }
    }
    grid.insert((pos.x, pos.y), sum);
    sum
}

impl Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}