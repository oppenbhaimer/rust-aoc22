use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("input/input_7")?;
    let reader = io::BufReader::new(file);

    let mut lines: Vec<String> = vec![];
    let mut tree: HashMap<String, Vec<String>> = HashMap::new();
    let mut sizes: HashMap<String, i64> = HashMap::new();
    let mut parents: HashMap<String, Option<String>> = HashMap::new();
    parents.insert(String::from("/"), None);
    tree.insert(String::from("/"), vec![]);
    sizes.insert(String::from("/"), 0);

    for line in reader.lines() {
        lines.push(line?);
    }

    let mut curr_dir = String::from("/");

    let mut i = 0;
    while i < lines.len() {
        if lines[i].starts_with('$') {
            let tokens: Vec<_> = lines[i].split(" ").collect();

            if tokens[1] == "ls" {
                i += 1;
                while i < lines.len() && !lines[i].starts_with('$') {

                    let attribs: Vec<_> = lines[i].split_whitespace().collect();
                    let size = match attribs[0].parse::<i64>() {
                        Ok(d) => d,
                        Err(_error) => 0 // directory
                    };
                    let node_name = format!("{}/{}", curr_dir, attribs[1]);

                    sizes.insert(node_name.clone(), size);
                    tree.insert(node_name.clone(), vec![]);
                    tree.get_mut(curr_dir.as_str()).unwrap().push(node_name.clone());
                    parents.insert(node_name, Some(curr_dir.clone()));
                    i += 1;
                }
            }
            else if tokens[1] == "cd" {
                if tokens[2] == ".." {
                    // go to parent dir 
                    curr_dir = curr_dir.rsplit_once("/").unwrap().0.to_string();
                }
                else if tokens[2] == "/" {
                    curr_dir = String::from("/");
                }
                else {
                    curr_dir.push_str(format!("/{}", tokens[2]).as_str());
                }
                i += 1;
            }
        }
    }

    // traverse and add
    let mut ans: i64 = 0;
    let mut dfs_stack: Vec<String> = vec![];
    let mut dfs_visited: HashMap<String, bool> = HashMap::new();
    for (k, _v) in sizes.iter() {
        dfs_visited.insert(k.clone(), false);
    }
    dfs_stack.push(String::from("/"));

    let mut indent = "".to_string();

    while !dfs_stack.is_empty() {

        let v = &dfs_stack.pop().unwrap();
        if dfs_visited[v] {
            if sizes[v] == 0 {
                let mut newsize = 0;
                for u in &tree[v] {
                    newsize += sizes[u];
                }
                sizes.insert(v.to_string(), newsize);
                indent = indent[..indent.len()-2].to_string();
            }
            continue;
        }
        else {
            if tree[v].len() > 0 {
                indent = format!("{}  ", indent);
                dfs_stack.push(v.to_string());
            }
            dfs_visited.insert(v.to_string(), true);
        }

        for u in tree.get(v).unwrap() {
            dfs_stack.push(u.to_string());
        }
    }

    let unused_space = 70000000-sizes["/"];
    let mut min_dir_size = sizes["/"];

    for (k, s) in sizes.iter() {
        if tree[k].len() > 0 && unused_space + s >= 30000000 && s < &min_dir_size {
            min_dir_size = *s;
        }
    }
    println!("{}", min_dir_size);

    Ok(())
}
