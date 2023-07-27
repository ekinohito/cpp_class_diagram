use crate::parsing::{CompoundStmt, Node};

use super::{draw_algorithm_subgraph, utils::draw_edge, Context, DrawAlgorithmSubgraph, Subgraph};

impl DrawAlgorithmSubgraph for CompoundStmt {
    fn draw_algorithm_subgraph(&self, node: &Node, ctx: &Context) -> Option<Subgraph> {
        let mut representation = String::new();
        let mut input = None;
        let mut output = None;
        for node in &node.inner {
            let Some(subgraph) = draw_algorithm_subgraph(node, ctx) else {
                continue;
            };
            representation += &subgraph.representation;
            representation += &draw_edge(output.as_ref(), subgraph.input.as_ref());
            input.get_or_insert(subgraph.input);
            output = subgraph.output;
        }
        Some(Subgraph {
            representation,
            input: input?,
            output,
        })
    }
}

#[test]
fn feature() {
    use super::Source;
    use crate::parsing::Clang;

    let source = Source::new("src/mocks/rich_method.cpp");
    let ctx = Context::new(&source);
    let node: Node = serde_json::from_str(include_str!("mocks/compound_stmt.json")).unwrap();
    let Clang::CompoundStmt(inner) = &node.kind else {panic!()};
    println!(
        "{}",
        inner
            .draw_algorithm_subgraph(&node, &ctx)
            .unwrap()
            .representation
    );
}
