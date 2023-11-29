use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("input/input_2")?;
    let reader = io::BufReader::new(file);

    let mut score: i32 = 0;

    for line in reader.lines() {
        let l = line.unwrap();
        let parts: Vec<&str> = l.split_whitespace().collect();
        let mut i1 = parts[0].chars().next().unwrap() as i32;
        i1 -= 65;
        let mut i2 = parts[1].chars().next().unwrap() as i32;
        i2 -= 88;

        if i2 == (i1+1)%3 {
            score += 6 + i2 + 1;
        }
        else if i2 == i1 {
            score += 3 + i2 + 1;
        }
        else {
            score += i2 + 1;
        }
    }

    println!("{}", score);

    Ok(())
}
