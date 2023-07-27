use crate::parsing::{DoStmt, Node};

use super::{
    draw_algorithm_subgraph,
    utils::{draw_dot, draw_edge, draw_edge_comment, draw_rhombus},
    Context, DrawAlgorithmSubgraph, GraphId, Subgraph,
};

impl DrawAlgorithmSubgraph for DoStmt {
    fn draw_algorithm_subgraph(&self, node: &Node, ctx: &Context) -> Option<Subgraph> {
        let id = node.id.to_string();
        let continue_label = GraphId(id.clone() + "_con");
        let input = continue_label.clone();
        let break_label = GraphId(id.clone() + "_brk");
        let output = break_label.clone();
        let id = GraphId(id);

        let mut representation = String::new();
        let mut iter = node.inner.iter();
        let loop_body = iter.next().expect("No loop body");
        let condition = iter.next().expect("No condition");
        representation += &draw_dot(&continue_label);
        representation += &draw_dot(&break_label);
        representation += &draw_rhombus(
            &id,
            ctx.source.get(condition.kind.range().as_ref().unwrap())?,
        );
        // representation += &draw_edge(Some(&continue_label), Some(&id));
        representation += &draw_edge_comment(Some(&id), Some(&continue_label), "Yes");
        representation += &draw_edge_comment(Some(&id), Some(&break_label), "No");

        let loop_ctx = Context {
            source: ctx.source,
            break_label: Some(&break_label),
            continue_label: Some(&continue_label),
            return_label: ctx.return_label,
            switch_label: ctx.switch_label,
        };

        if let Some(subgraph) = draw_algorithm_subgraph(loop_body, &loop_ctx) {
            representation += &draw_edge(Some(&continue_label), subgraph.input.as_ref());
            representation += &subgraph.representation;
            representation += &draw_edge(subgraph.output.as_ref(), Some(&id));
        } else {
            representation += &draw_edge(Some(&continue_label), Some(&id));
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
    let node: Node = serde_json::from_str(include_str!("mocks/do_stmt.json")).unwrap();
    let Clang::DoStmt(inner) = &node.kind else {panic!()};
    println!(
        "{}",
        inner
            .draw_algorithm_subgraph(&node, &ctx)
            .unwrap()
            .representation
    );
}
