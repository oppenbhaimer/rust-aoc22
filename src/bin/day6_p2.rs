use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("input/input_6")?;
    let mut reader = io::BufReader::new(file);

    let mut l = String::new();
    let _ = reader.read_line(&mut l);

    let mut ans = 13;
    for w in l.chars().collect::<Vec<_>>().windows(14) {
        let mut s: HashSet<_> = HashSet::new();

        for c in w {
            s.insert(c);
        }

        ans += 1;
        if s.len() == 14 {
            break;
        }
    }

    println!("{}", ans);

    Ok(())
}
