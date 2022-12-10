use std::collections::BTreeSet;

const DATA: &str = include_str!("../../data/day08.txt");

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct TreePosition {
    row: usize,
    col: usize,
}

#[derive(Debug)]
struct Grid {
    tree_heights: Vec<Vec<isize>>,
    width: usize,
    height: usize,
}

impl Grid {
    pub fn parse(s: &str) -> Self {
        let tree_heights: Vec<_> = s
            .lines()
            .map(|s| {
                s.chars()
                    .map(|c| c.to_digit(10).unwrap() as isize)
                    .collect::<Vec<isize>>()
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

    fn get_height(&self, position: TreePosition) -> isize {
        *self
            .tree_heights
            .get(position.row)
            .unwrap()
            .get(position.col)
            .unwrap()
    }

    pub fn check_height(
        &self,
        position: TreePosition,
        last_height: &mut isize,
        visible: &mut BTreeSet<TreePosition>,
    ) -> bool {
        let height = self.get_height(position);
        if *last_height >= height {
            false
        } else {
            *last_height = height;
            visible.insert(position);
            true
        }
    }

    pub fn visible_trees(&self) -> usize {
        let mut visible: BTreeSet<TreePosition> = BTreeSet::new();

        for row in 0..self.height {
            let mut last_height = -1;
            for col in 0..self.width {
                self.check_height(TreePosition { row, col }, &mut last_height, &mut visible);
            }

            let mut last_height = -1;
            for col in (0..self.width).rev() {
                self.check_height(TreePosition { row, col }, &mut last_height, &mut visible);
            }
        }

        for col in 0..self.width {
            let mut last_height = -1;
            for row in 0..self.height {
                self.check_height(TreePosition { row, col }, &mut last_height, &mut visible);
            }

            let mut last_height = -1;
            for row in (0..self.height).rev() {
                self.check_height(TreePosition { row, col }, &mut last_height, &mut visible);
            }
        }

        visible.len()
    }

    pub fn scenic_score(&self, position: TreePosition) -> usize {
        let house_height = self.get_height(position);
        let mut count = [0; 4];

        for i in (0..position.col).rev() {
            count[3] += 1;
            let height = self.get_height(TreePosition { col: i, ..position });
            if house_height <= height {
                break;
            }
        }

        for j in (0..position.row).rev() {
            count[1] += 1;
            let height = self.get_height(TreePosition { row: j, ..position });
            if house_height <= height {
                break;
            }
        }

        for j in position.row + 1..self.height {
            count[0] += 1;
            let height = self.get_height(TreePosition { row: j, ..position });
            if house_height <= height {
                break;
            }
        }

        for i in (position.col + 1)..self.width {
            count[2] += 1;
            let height = self.get_height(TreePosition { col: i, ..position });
            if house_height <= height {
                break;
            }
        }

        count.iter().product()
    }
}

fn main() {
    let grid = Grid::parse(DATA);
    // That's not the right answer; your answer is too low.  (You guessed 591.)
    println!("trees visible = {}", grid.visible_trees());

    let mut best_scenic_score = 0;
    for row in 1..grid.height - 1 {
        for col in 1..grid.width - 1 {
            let scenic_score = grid.scenic_score(TreePosition { row, col });
            if scenic_score > best_scenic_score {
                best_scenic_score = scenic_score;
            }
        }
    }
    println!("best_scenic_score = {}", best_scenic_score);
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
    }

    #[test]
    fn test_part_1() {
        let grid = Grid::parse(SAMPLE);
        assert_eq!(grid.visible_trees(), 21);
    }

    #[test]
    fn test_part_2() {
        let grid = Grid::parse(SAMPLE);
        assert_eq!(grid.scenic_score(TreePosition { row: 1, col: 2 }), 4);
        assert_eq!(grid.scenic_score(TreePosition { row: 3, col: 2 }), 8);
    }
}
