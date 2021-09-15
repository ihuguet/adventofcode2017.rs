use std::fs::File;
use std::io::prelude::*;

enum Mode {
    Normal,
    Garbage,
}

enum Action {
    LvlUp,
    LvlDown,
    ChgMode(Mode),
    Escape,
    Ignore,
}

fn main() {
    let mut input = String::new();
    File::open("input.txt").unwrap().read_to_string(&mut input).unwrap();
    let chars_iter = input.chars();
    
    let mut mode = Mode::Normal;
    let mut level = 0u32;
    let mut normal_points = 0u32;
    let mut garbage_points = 0u32;
    let mut skip_next = false;

    for ch in chars_iter {
        if skip_next {
            skip_next = false;
            continue;
        }

        let action = match mode {
            Mode::Normal => parse_ch_mode_normal(ch),
            Mode::Garbage => parse_ch_mode_garbage(ch),
        };

        match action {
            Action::LvlUp => {level += 1;},
            Action::LvlDown => {normal_points += level; level -= 1;},
            Action::ChgMode(m) => {mode = m;},
            Action::Escape => {skip_next = true;},
            Action::Ignore => if let Mode::Garbage = mode {garbage_points += 1;},
        }
    }

    println!("Part1: points={}", normal_points);
    println!("Part2: count={}", garbage_points);
}

fn parse_ch_mode_normal(ch: char) -> Action {
    match ch {
        '{' => Action::LvlUp,
        '}' => Action::LvlDown,
        '<' => Action::ChgMode(Mode::Garbage),
        ',' => Action::Ignore,
        _ => panic!("Panic! char {} in normal mode", ch),
    }
}

fn parse_ch_mode_garbage(ch: char) -> Action {
    match ch {
        '>' => Action::ChgMode(Mode::Normal),
        '!' => Action::Escape,
        _ => Action::Ignore,
    }
}