use crate::parsing::{IfStmt, Node};

use super::{
    draw_algorithm_subgraph,
    utils::{draw_dot, draw_edge, draw_edge_comment, draw_rhombus},
    Context, DrawAlgorithmSubgraph, GraphId, Subgraph,
};

impl DrawAlgorithmSubgraph for IfStmt {
    fn draw_algorithm_subgraph(&self, node: &Node, ctx: &Context) -> Option<Subgraph> {
        let id = format!("{}", node.id);
        let input = GraphId(id.clone());
        let output = GraphId(id + "_out");

        let mut iter = node.inner.iter();
        let condition = iter.next().expect("No condition");
        let if_clause = iter.next().expect("No if clause");
        let else_clause = iter.next();
        let mut representation = String::new();
        representation += &draw_rhombus(
            &input,
            ctx.source.get(condition.kind.range().as_ref().unwrap())?,
        );

        representation += &draw_dot(&output);

        let if_clause_subgraph = draw_algorithm_subgraph(if_clause, ctx);
        let else_clause_subgraph =
            else_clause.and_then(|clause| draw_algorithm_subgraph(clause, ctx));

        if let Some(subgraph) = if_clause_subgraph {
            representation += &draw_edge_comment(Some(&input), subgraph.input.as_ref(), "Yes");
            representation += &subgraph.representation;
            representation += &draw_edge(subgraph.output.as_ref(), Some(&output));
        } else {
            representation += &draw_edge_comment(Some(&input), Some(&output), "Yes");
        }

        if let Some(subgraph) = else_clause_subgraph {
            representation += &draw_edge_comment(Some(&input), subgraph.input.as_ref(), "No");
            representation += &subgraph.representation;
            representation += &draw_edge(subgraph.output.as_ref(), Some(&output));
        } else {
            representation += &draw_edge_comment(Some(&input), Some(&output), "No");
        }

        Some(Subgraph::new(representation, Some(input), Some(output)))
    }
}

#[test]
fn feature() {
    use super::Source;
    use crate::parsing::Clang;

    let source = Source::new("src/mocks/rich_method.cpp");
    let ctx = Context::new(&source);
    let node: Node = serde_json::from_str(include_str!("mocks/if_stmt.json")).unwrap();
    let Clang::IfStmt(inner) = &node.kind else {panic!()};
    println!(
        "{}",
        inner
            .draw_algorithm_subgraph(&node, &ctx)
            .unwrap()
            .representation
    );
}
