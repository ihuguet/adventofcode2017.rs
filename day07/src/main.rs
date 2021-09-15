use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{BTreeSet, HashMap};
use std::cell::Cell;

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let progs_info = parse_progs(reader);

    let lowest = build_graph(&progs_info);
    println!("Part1: lowest={}", lowest.name);

    let unbalanced = find_unbalanced(&lowest).unwrap();
    println!("Part2: unbalanced={}", unbalanced.name);
    for prog in &unbalanced.progs_above {
        println!("   {} ({}) -> {}", prog.name, prog.own_weight, prog.weight());
    }
    let (diff, wrong_prog) = unbalanced.diff_weight_above();
    let wrong_prog = wrong_prog.unwrap();
    println!("-- FIX: set \"{}\".own_weight={}", wrong_prog.name, wrong_prog.own_weight - diff);
}

type ProgsInfo = HashMap<String,(u32,Vec<String>)>;

struct Prog {
    name: String,
    own_weight: u32,
    total_weight: Cell<Option<u32>>,
    progs_above: Vec<Prog>,
}

impl Prog {
    fn new(name: &str, own_weight: u32) -> Prog {
        Prog {
            name: String::from(name), own_weight,
            total_weight: Cell::from(None), progs_above: Vec::new()
        }
    }

    fn weight(&self) -> u32 {
        if let Some(w) = self.total_weight.get() {
            return w;
        }

        let mut result = self.own_weight;
        for prog in &self.progs_above {
            result += prog.weight();
        }
        self.total_weight.set(Some(result));
        result
    }

    fn add_progs_above(&mut self, names: &Vec<String>, progs_info: &ProgsInfo) {
        for name in names {
            let (weight, names_above) = progs_info.get(name).unwrap();
            let mut prog = Prog::new(name, *weight);
            prog.add_progs_above(names_above, progs_info);
            self.progs_above.push(prog);
        }
    }
    
    fn diff_weight_above(&self) -> (u32, Option<&Prog>) {
        for p in &self.progs_above {
            let mut equals_count = -1; // -1 to compensabe being equal to itself
            let mut other = p;
    
            for p2 in &self.progs_above {
                if p.weight() == p2.weight() {
                    equals_count += 1;
                }
                else {
                    other = p2;
                }
            }

            if equals_count == 0 {
                return (p.weight() - other.weight(), Some(p));
            }
        }

        (0, None)
    }
}

fn find_unbalanced(prog: &Prog) -> Option<&Prog> {
    if let (_, Some(p)) = prog.diff_weight_above() {
        match find_unbalanced(p) { // recursively search
            Some(p_child) => return Some(p_child),
            None => return Some(prog),
        }
    }
    
    None
}

fn build_graph(progs_info: &ProgsInfo) -> Prog {
    let lowest_name = find_lowest(&progs_info);
    let (lowest_weight, lowest_children) = progs_info.get(&lowest_name).unwrap();
    
    let mut lowest = Prog::new(&lowest_name, *lowest_weight);
    lowest.add_progs_above(&lowest_children, progs_info);

    lowest
}

fn find_lowest(progs_info: &ProgsInfo) -> String {
    let mut maybe_list = BTreeSet::new();
    let mut deny_list  = BTreeSet::new();

    for (prog, (_, progs_above)) in progs_info {
        if !deny_list.remove(prog) {
            maybe_list.insert(String::from(prog));
        }

        for deny_prog in progs_above {
            if !maybe_list.remove(deny_prog) {
                deny_list.insert(String::from(deny_prog));
            }
        }
    }

    assert!(maybe_list.len() == 1);
    String::from(maybe_list.iter().next().unwrap())
}

fn parse_progs<T: BufRead>(reader: T) -> ProgsInfo {
    let mut progs_info = ProgsInfo::new();

    for line in reader.lines().map(|l| l.unwrap()) {
        let (prog, weight, progs_above) = parse_line(&line);
        let progs_above = progs_above.iter().map(|s| String::from(*s)).collect();
        progs_info.insert(String::from(prog), (weight, progs_above));
    }

    progs_info
}

fn parse_line(line: &str) -> (&str, u32, Vec<&str>) {
    let mut split = line.split(" -> ");

    let mut split_left = split.next().unwrap().split_whitespace();
    let name = split_left.next().unwrap();
    let weight = split_left.next().unwrap();
    let weight = weight[1..weight.len()-1].parse::<u32>().unwrap();
    
    let progs_above: Vec<&str> = 
        if let Some(above_str) = split.next() {
            above_str.split(", ").collect()
        } else {
            vec![]
        };
    
    (name, weight, progs_above)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn line_parser() {
        let (prog, weight, progs_above) = parse_line("fwft (72) -> ktlj, cntj, xhth");
        assert_eq!((prog, weight), ("fwft", 72));
        assert_eq!(progs_above, vec!["ktlj", "cntj", "xhth"]);

        let (prog, weight, progs_above) = parse_line("pbga (34)");
        assert_eq!((prog, weight), ("pbga", 34));
        assert_eq!(progs_above, Vec::<&str>::new());
    }

    #[test]
    fn progs_parser() {
        let file = File::open("input_test_parser.txt").unwrap();
        let reader = BufReader::new(file);

        let parsed_progs_info = parse_progs(reader);

        let mut expects = ProgsInfo::new();
        expects.insert(String::from("padx"),
                      (45, vec!(String::from("pbga"), String::from("ktlj"))));
        expects.insert(String::from("pbga"), (66, Vec::new()));
        expects.insert(String::from("ktlj"), (57, Vec::new()));

        assert_eq!(parsed_progs_info, expects);
    }

    #[test]
    fn lowest_search() {
        let file = File::open("input_test.txt").unwrap();
        let reader = BufReader::new(file);
        let progs_info = parse_progs(reader);

        let lowest = find_lowest(&progs_info);
        assert_eq!(lowest, "tknk");
    }

    #[test]
    fn unbalanced_search() {
        let file = File::open("input_test.txt").unwrap();
        let reader = BufReader::new(file);
        let progs_info = parse_progs(reader);
        let graph = build_graph(&progs_info);

        let unbalanced = find_unbalanced(&graph).unwrap();
        assert_eq!(unbalanced.name, "tknk");
    }
}