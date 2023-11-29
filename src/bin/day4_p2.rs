use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("input/input_4")?;
    let reader = io::BufReader::new(file);

    let mut ans = 0;

    for line in reader.lines() {
        let m: Vec<_> = line.unwrap().split(",").map(
            |s| s.split("-").map(
                |n| n.parse::<i32>().unwrap()
            ).collect::<Vec<_>>()
        ).collect::<Vec<_>>();

        if (m[0][1] >= m[1][0] && m[1][1] >= m[0][0]) || 
           (m[0][0] <= m[1][0] && m[0][1] >= m[1][1]) || 
           (m[0][0] >= m[1][0] && m[0][1] <= m[1][1]) {
            ans += 1;
        }
    }

    println!("{}", ans);

    Ok(())
}
