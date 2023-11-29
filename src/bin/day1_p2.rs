use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("input/input_1")?;
    let reader = io::BufReader::new(file);

    let mut curr_ctr = 0;

    let mut v = vec![];

    for line in reader.lines() {
        let l = line.unwrap();
        if l == "" {
            v.push(curr_ctr);
            curr_ctr = 0;
        }
        else {
            curr_ctr += l.parse::<i32>().unwrap();
        }
    }

    v.sort_by(|a,b| b.cmp(a));

    println!("{}", v[0]+v[1]+v[2]);

    Ok(())
}
