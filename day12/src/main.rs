use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::BTreeSet;

type Group = BTreeSet<u32>;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.unwrap());

    let mut groups: Vec<Group> = Vec::new();

    for line in lines {
        let procs = parse_line(&line);
        let procs_groups_idxs = find_existing_groups(&procs, &groups);
        let mut new_group: Group = procs.into_iter().collect();

        if let Some(idxs) = procs_groups_idxs {
            for &idx in idxs.iter().rev() {
                new_group.append(&mut groups[idx]);
                groups.remove(idx);
            }
        }

        groups.push(new_group);
    }

    let group0_len = groups.iter()
                           .find_map(|g| g.contains(&0).then(|| g.len()))
                           .unwrap_or(0);

    println!("Part 1: PID 0's group len={}", group0_len);
    println!("Part 2: num of groups={}", groups.len());
    Ok(())
}

fn parse_line(line: &str) -> Vec<u32> {
    let mut split = line.split(" <-> ");

    let proc: u32 = split.next().unwrap()
                        .parse().unwrap();
                    
    let mut procs: Vec<u32> = split.next().unwrap()
                                .split(", ")
                                .map(|s| s.parse().unwrap())
                                .collect();
    procs.push(proc);
    
    procs
}

fn find_existing_groups(procs: &Vec<u32>, groups: &Vec<Group>) -> Option<Vec<usize>> {
    let mut idxs = Vec::new();

    for proc in procs {
        for (i, group) in groups.iter().enumerate() {
            if group.contains(proc) {
                idxs.push(i);
            }
        }
    }
    idxs.sort();
    idxs.dedup();

    match idxs.len() {
        0 => None,
        _ => Some(idxs)
    }
}
