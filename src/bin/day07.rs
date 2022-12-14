use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, PartialEq, Clone)]
enum Line {
    Ls,
    Cd(String),
    File(String, usize),
    Directory(String),
}

impl From<&str> for Line {
    fn from(s: &str) -> Self {
        let mut word_iter = s.split(' ');
        let first_word = word_iter.next().unwrap_or_default();
        match first_word {
            "$" => {
                let second_word = word_iter.next().unwrap_or_default();
                match second_word {
                    "cd" => Line::Cd(word_iter.next().unwrap_or_default().to_string()),
                    "ls" => Line::Ls,
                    _ => panic!("unknown line"),
                }
            }
            "dir" => {
                let second_word = word_iter.next().unwrap_or_default();
                Line::Directory(second_word.to_string())
            }
            _ => {
                let size = first_word.parse::<usize>().unwrap_or_default();
                let second_word = word_iter.next().unwrap_or_default();
                Line::File(second_word.to_string(), size)
            }
        }
    }
}

const DATA: &str = include_str!("../../data/day07.txt");

fn collect_lines(lines: &[Line]) -> (BTreeSet<String>, BTreeMap<String, usize>) {
    let mut directory_stack: Vec<String> = vec![];
    let mut files: BTreeMap<String, usize> = BTreeMap::new();
    let mut dirs: BTreeSet<String> = BTreeSet::new();
    dirs.insert("/".to_string());
    for line in lines {
        match &line {
            Line::Cd(name) => match name.as_str() {
                ".." => {
                    directory_stack.pop();
                }
                _ => {
                    let path_component = name.to_string();
                    directory_stack.push(path_component);
                }
            },
            Line::Directory(name) => {
                let path = format!("{}/{}", &directory_stack.join("/")[1..], name);
                dirs.insert(path);
            }
            Line::File(name, size) => {
                let path = format!("{}/{}", &directory_stack.join("/")[1..], name);
                files.insert(path, *size);
            }
            _ => {}
        }
    }
    (dirs, files)
}

const SIZE_LIMIT: usize = 100_000;

fn find_sum_of_smalls(dirs: &BTreeSet<String>, files: &BTreeMap<String, usize>) -> usize {
    let total: usize = dirs
        .iter()
        .filter_map(|dir_path| {
            let dir_with_delim = if dir_path != "/" {
                format!("{dir_path}/",)
            } else {
                "/".to_string()
            };
            let dir_size: usize = files
                .iter()
                .filter_map(|(k, v)| {
                    if k.starts_with(dir_with_delim.as_str()) {
                        Some(v)
                    } else {
                        None
                    }
                })
                .sum();
            if dir_size <= SIZE_LIMIT {
                Some(dir_size)
            } else {
                None
            }
        })
        .sum();

    total
}

fn find_candidates(
    dirs: &BTreeSet<String>,
    files: &BTreeMap<String, usize>,
    needed: usize,
) -> Vec<(usize, String)> {
    let candidates: Vec<(usize, String)> = dirs
        .iter()
        .filter_map(|dir_path| {
            let dir_with_delim = if dir_path != "/" {
                format!("{dir_path}/")
            } else {
                "/".to_string()
            };
            let dir_size: usize = files
                .iter()
                .filter_map(|(k, v)| {
                    if k.starts_with(dir_with_delim.as_str()) {
                        Some(v)
                    } else {
                        None
                    }
                })
                .sum();
            if dir_size >= needed {
                Some((dir_size, dir_path.to_string()))
            } else {
                None
            }
        })
        .collect();

    candidates
}

const CAPACITY: usize = 70_000_000;
const SPACE_NEEDED: usize = 30_000_000;

fn main() {
    let lines: Vec<_> = DATA.lines().map(Line::from).collect();
    let (dirs, files) = collect_lines(&lines);
    let total = find_sum_of_smalls(&dirs, &files);
    println!("total of smalls = {total}");

    let used_size: usize = files.values().sum();
    println!("used_size ={used_size}");
    let free_size = CAPACITY - used_size;
    println!("free_size ={free_size}");
    let target_min_size = SPACE_NEEDED - free_size;
    println!("target_min_size ={target_min_size}");

    let mut candidates = find_candidates(&dirs, &files, target_min_size);
    candidates.sort();

    println!("candidate size = {}", candidates[0].0);
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;

    #[test]
    fn test_parse_line() {
        assert_eq!(Line::from("$ ls"), Line::Ls);
        assert_eq!(Line::from("$ cd .."), Line::Cd("..".to_string()));
        assert_eq!(Line::from("$ cd a"), Line::Cd("a".to_string()));
        assert_eq!(Line::from("0 a"), Line::File("a".to_string(), 0));
        assert_eq!(Line::from("dir b"), Line::Directory("b".to_string()));
    }

    #[test]
    fn test_parse_sample() {
        let lines: Vec<_> = SAMPLE.lines().map(Line::from).collect();
        assert_eq!(lines.len(), 23);
        assert_eq!(lines[0], Line::Cd("/".to_string()));
        assert_eq!(lines[22], Line::File("k".to_string(), 7214296));

        let (dirs, files) = collect_lines(&lines);
        let total_size: usize = files.values().sum();
        assert_eq!(total_size, 48381165);

        let e_size: usize = files
            .iter()
            .filter_map(|(k, v)| {
                if k.starts_with("/a/e/") {
                    Some(v)
                } else {
                    None
                }
            })
            .sum();
        assert_eq!(e_size, 584);
        let a_size: usize = files
            .iter()
            .filter_map(|(k, v)| if k.starts_with("/a/") { Some(v) } else { None })
            .sum();
        assert_eq!(a_size, 94853);
        let d_size: usize = files
            .iter()
            .filter_map(|(k, v)| if k.starts_with("/d/") { Some(v) } else { None })
            .sum();
        assert_eq!(d_size, 24933642);

        let total = find_sum_of_smalls(&dirs, &files);

        assert_eq!(total, 95437);

        let used_size: usize = files.values().sum();
        println!("used_size ={}", used_size);
        let free_size = CAPACITY - used_size;
        println!("free_size ={}", free_size);
        let target_min_size = SPACE_NEEDED - free_size;
        println!("target_min_size ={}", target_min_size);

        let mut candidates = find_candidates(&dirs, &files, target_min_size);
        candidates.sort();

        dbg!(&candidates);

        assert_eq!(candidates[0].0, 24933642);
        assert_eq!(candidates[0].1, "/d");
    }
}
