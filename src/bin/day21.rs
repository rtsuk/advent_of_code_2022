use anyhow::Error;
use evalexpr::{eval_with_context_mut, Context, HashMapContext};
use id_tree::{
    InsertBehavior::{AsRoot, UnderNode},
    Node, NodeId, Tree, TreeBuilder,
};
use std::collections::{HashSet,HashMap};
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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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

type NodeIdMap = HashMap<String, NodeId>;

fn add_children(
    tree: &mut Tree<usize>,
    list: &ExpressionList,
    exp_map: &HashMap<String, usize>,
    identifier: &str,
    parent: &NodeId,
    node_id_map: &mut NodeIdMap,
) {
    let exp_index = exp_map
        .get(identifier)
        .unwrap_or_else(|| panic!("identifier {identifier}"));
    let my_node = tree
        .insert(Node::new(*exp_index), UnderNode(parent))
        .unwrap();
    node_id_map.insert(identifier.to_owned(), my_node.clone());
    for reffed in list[*exp_index].references() {
        add_children(tree, list, exp_map, &reffed, &my_node, node_id_map);
    }
}

fn parse(s: &str) -> (Tree<usize>, ExpressionList, Vec<usize>, NodeIdMap) {
    let list: ExpressionList = s.lines().map(job).collect();
    let mut node_id_map = NodeIdMap::new();
    let exp_map: HashMap<String, usize> = list
        .iter()
        .enumerate()
        .map(|(index, exp)| (exp.0.clone(), index))
        .collect();
    let mut tree: Tree<usize> = TreeBuilder::new().with_node_capacity(list.len()).build();
    let root_index = exp_map.get("root").expect("root");
    let root_id: NodeId = tree.insert(Node::new(*root_index), AsRoot).unwrap();
    node_id_map.insert("root".to_owned(), root_id.clone());
    for reffed in list[*root_index].references() {
        add_children(
            &mut tree,
            &list,
            &exp_map,
            &reffed,
            &root_id,
            &mut node_id_map,
        );
    }
    let order: Vec<usize> = tree
        .traverse_post_order(&root_id)
        .unwrap()
        .map(Node::data)
        .copied()
        .collect();
    (tree, list, order, node_id_map)
}

fn setup_context(
    context: &mut HashMapContext,
    expression_list: &ExpressionList,
    order: &Vec<usize>,
) {
    for index in order.iter() {
        let expr = &expression_list[*index];
        let exp = format!("{} = {}", expr.0, expr.1);
        eval_with_context_mut(&exp, context).expect("eval_with_context");
    }
}

fn solve_part_1(_tree: Tree<usize>, expression_list: ExpressionList, order: Vec<usize>) -> isize {
    let mut context = HashMapContext::new();
    setup_context(&mut context, &expression_list, &order);
    context
        .get_value("root")
        .expect("root value")
        .as_int()
        .expect("as_int") as isize
}

fn solve_part_2(
    tree: Tree<usize>,
    expression_list: ExpressionList,
    order: Vec<usize>,
    map: &NodeIdMap,
) -> isize {
    let root_id = map.get("root").expect("root");
    let hmnd_id = map.get("humn").expect("humn");
    let ancestors: Vec<_> = tree.ancestor_ids(hmnd_id).expect("ancestors").collect();
	let ancestors_set: HashSet<_> = ancestors.iter().collect();
    let human_pen_ancestor = ancestors[ancestors.len() - 2];
    let other_ancestor_id = tree
        .children_ids(root_id)
        .expect("children_ids")
        .find(|id| id != &human_pen_ancestor)
        .expect("other_ancestor");

    let other_ancestor = tree.get(other_ancestor_id).expect("other_ancestor").data();
    let other_ancestor_identifier = expression_list[*other_ancestor].0.to_owned();
    println!("other_ancestor = {:#?}", other_ancestor_identifier);

    let mut context = HashMapContext::new();
    setup_context(&mut context, &expression_list, &order);

    let other_ancestor_val = context
        .get_value(&other_ancestor_identifier)
        .expect("root value")
        .as_int()
        .expect("as_int") as isize;

    println!("other_ancestor_val = {}", other_ancestor_val);

    let mut other_expression_list = expression_list.clone();

    for an in ancestors.iter() {
        let other_ancestor_id = tree
            .children_ids(root_id)
            .expect("children_ids")
            .find(|id| id != an)
            .expect("other_ancestor");
        let other_ancestor = tree.get(other_ancestor_id).expect("other_ancestor").data();
        let other_ancestor_identifier = expression_list[*other_ancestor].0.to_owned();
        let other_ancestor_val = context
            .get_value(&other_ancestor_identifier)
            .expect("root value")
            .as_int()
            .expect("as_int") as isize;
        let exp = format!("{} = {}", other_ancestor_identifier, other_ancestor_val);
		other_expression_list[*other_ancestor].1 = exp;
    }

    println!("other_expression_list = {:#?}", other_expression_list);
	
	let human_anc = ancestors[0];
	let human_anc_idx = tree.get(human_anc).expect("human_anc").data();

    println!("human_anc = {:#?}", expression_list[*human_anc_idx].1);

    todo!();
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();

    let file_contents = parse(if opt.puzzle_input { DATA } else { SAMPLE });

    println!(
        "part 1 root = {}",
        solve_part_1(file_contents.0, file_contents.1, file_contents.2)
    );

    let file_contents = parse(if opt.puzzle_input { DATA } else { SAMPLE });

    println!(
        "part 2 root = {}",
        solve_part_2(
            file_contents.0,
            file_contents.1,
            file_contents.2,
            &file_contents.3
        )
    );

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let file_contents = parse(SAMPLE);
        assert_eq!(file_contents.1.len(), 15);
        assert_eq!(file_contents.2.len(), 15);
    }

    #[test]
    fn test_part_1() {
        let file_contents = parse(SAMPLE);
        let root = solve_part_1(file_contents.0, file_contents.1, file_contents.2);
        assert_eq!(root, 152);
    }

    #[test]
    fn test_part_2() {
        let file_contents = parse(SAMPLE);
        let root = solve_part_2(
            file_contents.0,
            file_contents.1,
            file_contents.2,
            &file_contents.3,
        );
        assert_eq!(root, 301);
    }
}
