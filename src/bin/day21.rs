use anyhow::Error;
use evalexpr::{eval_with_context_mut, Context, HashMapContext};
use id_tree::{
    InsertBehavior::{AsRoot, UnderNode},
    Node, NodeId, Tree, TreeBuilder,
};
use std::collections::HashMap;
use structopt::StructOpt;

const DATA: &str = include_str!("../../data/day21.txt");
const SAMPLE: &str = r#"root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32"#;

#[derive(Debug, StructOpt)]
#[structopt(name = "day21", about = "Monkey Math")]
struct Opt {
    /// Use puzzle input instead of the sample
    #[structopt(short, long)]
    puzzle_input: bool,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Expression(String, String);

impl Expression {
    fn references(&self) -> Vec<String> {
        self.1
            .split(['+', '-', '/', '*', '='])
            .map(str::trim)
            .map(str::to_string)
            .filter_map(|s| (s.parse::<isize>().is_err().then_some(s)))
            .collect()
    }
}

type ExpressionList = Vec<Expression>;

fn job(s: &str) -> Expression {
    let mut parts = s.split(": ");
    let identifier = parts.next().unwrap().to_string();

    Expression(identifier, parts.next().unwrap().to_string())
}

fn add_children(
    tree: &mut Tree<usize>,
    list: &ExpressionList,
    exp_map: &HashMap<String, usize>,
    identifier: &str,
    parent: &NodeId,
) {
    let exp_index = exp_map
        .get(identifier)
        .unwrap_or_else(|| panic!("identifier {identifier}"));
    let my_node = tree
        .insert(Node::new(*exp_index), UnderNode(parent))
        .unwrap();

    for reffed in list[*exp_index].references() {
        add_children(tree, list, exp_map, &reffed, &my_node);
    }
}

fn parse(s: &str) -> (ExpressionList, Vec<usize>) {
    let list: ExpressionList = s.lines().map(job).collect();
    let exp_map: HashMap<String, usize> = list
        .iter()
        .enumerate()
        .map(|(index, exp)| (exp.0.clone(), index))
        .collect();
    let mut tree: Tree<usize> = TreeBuilder::new().with_node_capacity(list.len()).build();
    let root_index = exp_map.get("root").expect("root");
    let root_id: NodeId = tree.insert(Node::new(*root_index), AsRoot).unwrap();
    for reffed in list[*root_index].references() {
        add_children(&mut tree, &list, &exp_map, &reffed, &root_id);
    }
    let order: Vec<usize> = tree
        .traverse_post_order(&root_id)
        .unwrap()
        .map(Node::data)
        .copied()
        .collect();
    (list, order)
}

fn solve_part_1(expression_list: ExpressionList, order: Vec<usize>) -> isize {
    let mut context = HashMapContext::new();
    for index in order.into_iter() {
        let expr = &expression_list[index];
        let exp = format!("{} = {}", expr.0, expr.1);
        eval_with_context_mut(&exp, &mut context).expect("eval_with_context");
    }
    context
        .get_value("root")
        .expect("root value")
        .as_int()
        .expect("as_int") as isize
}

fn solve_part_2(_expression_list: ExpressionList, _order: Vec<usize>) -> isize {
    todo!();
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();

    let file_contents = parse(if opt.puzzle_input { DATA } else { SAMPLE });

    println!(
        "part 1 root = {}",
        solve_part_1(file_contents.0, file_contents.1)
    );

    let file_contents = parse(if opt.puzzle_input { DATA } else { SAMPLE });

    println!(
        "part 2 root = {}",
        solve_part_2(file_contents.0, file_contents.1)
    );

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let file_contents = parse(SAMPLE);
        assert_eq!(file_contents.0.len(), 15);
        assert_eq!(file_contents.1.len(), 15);
    }

    #[test]
    fn test_part_1() {
        let file_contents = parse(SAMPLE);
        let root = solve_part_1(file_contents.0, file_contents.1);
        assert_eq!(root, 152);
    }

    #[test]
    fn test_part_2() {
        let file_contents = parse(SAMPLE);
        let root = solve_part_2(file_contents.0, file_contents.1);
        assert_eq!(root, 301);
    }
}
