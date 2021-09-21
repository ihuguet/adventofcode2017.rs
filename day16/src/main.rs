use std::collections::HashMap;

struct Row {
    row: Vec<u8>,
    index: Vec<usize>,
    start: usize,
}

impl Row {
    fn rotate_right(&mut self, n: usize) {
        let new_start = self.start as isize - n as isize;
        self.start = if new_start >= 0 {
                        new_start as usize
                    } else {
                        self.row.len() - (-new_start) as usize
                    };
    }

    fn swap_positions(&mut self, a: usize, b: usize) {
        let mut a = self.start + a;
        if a >= self.row.len() { a -= self.row.len(); }

        let mut b = self.start + b;
        if b >= self.row.len() { b -= self.row.len(); }

        self.swap_items(self.row[a], self.row[b]);
    }

    fn swap_items(&mut self, a: u8, b: u8) {
        let a = (a - b'a') as usize;
        let b = (b - b'a') as usize;
        self.row.swap(self.index[a], self.index[b]);
        self.index.swap(a, b);
    }

    fn to_string(&self) -> String {
        let mut s = String::from_utf8_lossy(&self.row[self.start..]).to_string();
        s.push_str(&String::from_utf8_lossy(&self.row[..self.start]).to_string());
        s
    }
}

impl From<Vec<u8>> for Row {
    fn from(v: Vec<u8>) -> Row {
        Row {
            index: (0..v.len()).collect(),
            row: v,
            start: 0,
        }
    }
}

#[derive(Debug,PartialEq)]
enum Action {
    Spin(usize),
    Exchange(usize,usize),
    Partner(u8,u8),
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let actions:Vec<Action> = input.split(",").map(|s| Action::from(s)).collect();
    
    let result = execute_actions(&actions, 1);
    println!("Part 1: row=\"{}\"", result);

    let result = execute_actions(&actions, 1000000000);
    println!("Part 2: row=\"{}\"", result);
}

fn execute_actions(actions: &[Action], mut iters: u32) -> String {
    let mut row = Row::from((b'a'..=b'p').collect::<Vec<u8>>());

    let mut seen = HashMap::new();

    while iters > 0 {
        for action in actions {
            match action {
                Action::Spin(n) => row.rotate_right(*n),
                Action::Exchange(a, b) => row.swap_positions(*a, *b),
                Action::Partner(a, b) => row.swap_items(*a, *b),
            }
        }

        if let Some(prev_iters) = seen.insert(row.to_string(), iters) {
            let interval = prev_iters - iters;
            while iters > interval {
                iters -= interval;
            }
        }

        iters -= 1;
    }

    row.to_string()
}

impl From<&str> for Action {
    fn from(s: &str) -> Action {
        let (code, data) = s.split_at(1);
        if code == "s" {
            Action::Spin(data.parse().unwrap())
        } else if code == "x" {
            let mut split = data.split("/");
            let a = split.next().unwrap();
            let b = split.next().unwrap();
            Action::Exchange(a.parse().unwrap(), b.parse().unwrap())
        } else if code == "p" {
            let mut split = data.split("/");
            let a = split.next().unwrap();
            let b = split.next().unwrap();
            let (a, b) = (a.chars().nth(0).unwrap(), b.chars().nth(0).unwrap());
            Action::Partner(a as u8, b as u8)
        } else {
            panic!();
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spin_test() {
        let mut s = Row::from(b"abcde".to_vec());
        s.rotate_right(3);
        assert_eq!(s.to_string(), "cdeab");
    }

    #[test]
    fn exchange_test() {
        let mut s = Row::from(b"abcde".to_vec());
        s.swap_positions(1, 3);
        assert_eq!(s.to_string(), "adcbe");
    }

    #[test]
    fn partner_test() {
        let mut s = Row::from(b"abcde".to_vec());
        s.swap_items(b'b', b'd');
        assert_eq!(s.to_string(), "adcbe");

        s.swap_items(b'a', b'b');
        assert_eq!(s.to_string(), "bdcae");
    }

    #[test]
    fn action_parsing() {
        let action = Action::from("s4");
        assert_eq!(action, Action::Spin(4));

        let action = Action::from("x12/4");
        assert_eq!(action, Action::Exchange(12,4));

        let action = Action::from("pa/b");
        assert_eq!(action, Action::Partner(b'a', b'b'));
    }
}