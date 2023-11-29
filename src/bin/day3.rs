use std::fs::File;
use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("input/input_3")?;
    let reader = io::BufReader::new(file);

    let mut ans: u32 = 0;

    for line in reader.lines() {
        let l = line.unwrap();
        let (s1, s2) = l.split_at(l.len()/2);

        let h1: HashSet<_> = s1.chars().collect();
        let h2: HashSet<_> = s2.chars().collect();

        let i: Vec<_> = h1.intersection(&h2).cloned().collect();

        for c in &i {
            let c_i = *c as u32;
            ans += match c_i {
                65 ..= 90 => 26+c_i-64,
                97 ..= 122 => c_i-96,
                0_u32..=64_u32 | 91_u32..=96_u32 | 123_u32..=u32::MAX => todo!()
            }
        }
    }

    println!("{}", ans);

    Ok(())
}
