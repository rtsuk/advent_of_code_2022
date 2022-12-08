use std::collections::BTreeSet;

const DATA: &str = include_str!("../../data/day8.txt");

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct TreePosition {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Grid {
    tree_heights: Vec<Vec<u32>>,
    width: usize,
    height: usize,
}

impl Grid {
    pub fn parse(s: &str) -> Self {
        let tree_heights: Vec<_> = s
            .lines()
            .map(|s| {
                s.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect();
        let width = tree_heights[0].len();
        let height = tree_heights.len();

        Self {
            tree_heights,
            width,
            height,
        }
    }

    pub fn visible_trees(&self) -> usize {
        let mut visible: BTreeSet<TreePosition> = BTreeSet::new();
        let mut last_height: Option<u32> = None;

        for row in 0..self.height {
            for col in 0..self.width {
                let height = self.tree_heights.get(row).unwrap().get(col).unwrap();
                if let Some(last) = last_height.as_ref() {
                    if last > height {
                        break;
                    }
                    last_height = Some(*height);
                    visible.insert(TreePosition { y: row, x: col });
                } else {
                    last_height = Some(*height);
                    visible.insert(TreePosition { y: row, x: col });
                }
            }

            last_height = None;
            for col in (0..self.width).rev() {
                let height = self.tree_heights.get(row).unwrap().get(col).unwrap();
                if let Some(last) = last_height.as_ref() {
                    if last > height {
                        break;
                    }
                    last_height = Some(*height);
                    visible.insert(TreePosition { y: row, x: col });
                } else {
                    last_height = Some(*height);
                    visible.insert(TreePosition { y: row, x: col });
                }
            }
        }

        for col in 0..self.width {
            last_height = None;
            for row in 0..self.height {
                let height = self.tree_heights.get(row).unwrap().get(col).unwrap();
                if let Some(last) = last_height.as_ref() {
                    if last > height {
                        break;
                    }
                    last_height = Some(*height);
                    visible.insert(TreePosition { y: row, x: col });
                } else {
                    last_height = Some(*height);
                    visible.insert(TreePosition { y: row, x: col });
                }
            }
            last_height = None;
            for row in (0..self.height).rev() {
                let height = self.tree_heights.get(row).unwrap().get(col).unwrap();
                if let Some(last) = last_height.as_ref() {
                    if last > height {
                        break;
                    }
                    last_height = Some(*height);
                    visible.insert(TreePosition { y: row, x: col });
                } else {
                    last_height = Some(*height);
                    visible.insert(TreePosition { y: row, x: col });
                }
            }
        }

        dbg!(&visible);

        visible.len()
    }
}

fn main() {
    let grid = Grid::parse(DATA);
    // That's not the right answer; your answer is too low.  (You guessed 591.)
    println!("trees visible = {}", grid.visible_trees());
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = r#"30373
25512
65332
33549
35390"#;

    #[test]
    fn test_parse() {
        let grid = Grid::parse(SAMPLE);
        assert_eq!(grid.width, 5);
        assert_eq!(grid.height, 5);
        assert_eq!(grid.tree_heights.len(), 5);
        assert_eq!(grid.tree_heights[0].len(), 5);
        assert_eq!(grid.visible_trees(), 21);
    }
}
