use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("input/input_5")?;
    let reader = io::BufReader::new(file);

    // easier than parsing the file by hand
    let mut dock: Vec<Vec<char>> = vec![];
    dock.push(vec!['W', 'M', 'L', 'F']);
    dock.push(vec!['B', 'Z', 'V', 'M', 'F']);
    dock.push(vec!['H', 'V', 'R', 'S', 'L', 'Q']);
    dock.push(vec!['F', 'S', 'V', 'Q', 'P', 'M', 'T', 'J']);
    dock.push(vec!['L', 'S', 'W']);
    dock.push(vec!['F', 'V', 'P', 'M', 'R', 'J', 'W']);
    dock.push(vec!['J', 'Q', 'C', 'P', 'N', 'R', 'F']);
    dock.push(vec!['V', 'H', 'P', 'S', 'Z', 'W', 'R', 'B']);
    dock.push(vec!['B', 'M', 'J', 'C', 'G', 'H', 'Z', 'W']);

    let pattern = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    let mut idx = 0;
    for line in reader.lines() {
        idx += 1;
        if idx <= 10 {
            continue;
        }

        let l = line.unwrap();
        let cap = pattern.captures(&l).unwrap();
        let a : usize = cap.get(1).unwrap().as_str().parse().unwrap();
        let b : usize = cap.get(2).unwrap().as_str().parse().unwrap();
        let c : usize = cap.get(3).unwrap().as_str().parse().unwrap();

        let last_a : usize = dock[b-1].len() - a;
        let elems = dock[b-1][last_a..].to_vec();
        for elem in elems {
            dock[c-1].push(elem);
        }

        dock[b-1].drain(last_a..);
    }

    for i in 0..9 {
        print!("{}", dock[i].last().unwrap());
    }
    println!("");

    Ok(())
}
