use std::fs::File;
use std::cmp;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("input/input_1")?;
    let reader = io::BufReader::new(file);

    let mut max_ctr = 0;
    let mut curr_ctr = 0;

    for line in reader.lines() {
        let l = line.unwrap();
        if l == "" {
            max_ctr = cmp::max(curr_ctr, max_ctr);
            curr_ctr = 0;
        }
        else {
            curr_ctr += l.parse::<i32>().unwrap();
        }
    }

    max_ctr = cmp::max(curr_ctr, max_ctr);

    println!("{}", max_ctr);

    Ok(())
}
