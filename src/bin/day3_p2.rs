use std::fs::File;
use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("input/input_3")?;
    let reader = io::BufReader::new(file);

    let mut ans: u32 = 0;
    let mut group: Vec<String> = vec![];

    for line in reader.lines() {
        let l = line.unwrap();
        group.push(l);
        if group.len() < 3 {
            continue;
        }

        let h1: HashSet<_> = group[0].chars().collect();
        let h2: HashSet<_> = group[1].chars().collect();
        let h3: HashSet<_> = group[2].chars().collect();

        let h12: HashSet<_> = h1.intersection(&h2).collect();
        let h23: HashSet<_> = h2.intersection(&h3).collect();
        let h123: HashSet<_> = h12.intersection(&h23).collect();

        for c in h123 {
            let c_i = **c as u32; // Why is this **?
            ans += match c_i {
                65 ..= 90 => 26+c_i-64,
                97 ..= 122 => c_i-96,
                0_u32..=64_u32 | 91_u32..=96_u32 | 123_u32..=u32::MAX => todo!()
            }
        }
        group.clear();
    }

    println!("{}", ans);

    Ok(())
}
