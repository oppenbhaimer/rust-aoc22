use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("input/input_6")?;
    let mut reader = io::BufReader::new(file);

    let mut l = String::new();
    let _ = reader.read_line(&mut l);

    let mut ans = 3;
    for w in l.chars().collect::<Vec<_>>().windows(4) {
        let mut s: HashSet<_> = HashSet::new();

        s.insert(w[0]);
        s.insert(w[1]);
        s.insert(w[2]);
        s.insert(w[3]);

        ans += 1;
        if s.len() == 4 {
            break;
        }
    }

    println!("{}", ans);

    Ok(())
}
