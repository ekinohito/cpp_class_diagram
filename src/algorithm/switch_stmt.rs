use crate::parsing::{Node, SwitchStmt};

use super::{
    draw_algorithm_subgraph,
    utils::{draw_dot, draw_rhombus},
    Context, DrawAlgorithmSubgraph, GraphId, Subgraph,
};

impl DrawAlgorithmSubgraph for SwitchStmt {
    fn draw_algorithm_subgraph(&self, node: &Node, ctx: &Context) -> Option<Subgraph> {
        let id = node.id.to_string();
        let break_label = GraphId(id.clone() + "_brk");
        let output = Some(break_label.clone());
        let id = GraphId(id);
        let input = id.clone();

        let mut representation = String::new();
        let mut iter = node.inner.iter();
        let condition = iter.next().expect("No condition");
        let switch_body = iter.next().expect("No switch body");
        representation += &draw_dot(&break_label);
        representation += &draw_rhombus(
            &id,
            ctx.source.get(condition.kind.range().as_ref().unwrap())?,
        );

        let loop_ctx = Context {
            source: ctx.source,
            break_label: Some(&break_label),
            continue_label: ctx.continue_label,
            return_label: ctx.return_label,
            switch_label: Some(&id),
        };

        if let Some(subgraph) = draw_algorithm_subgraph(switch_body, &loop_ctx) {
            representation += &subgraph.representation;
        }

        Some(Subgraph::new(representation, Some(input), output))
    }
}

#[test]
fn feature() {
    use super::Source;
    use crate::parsing::Clang;

    let source = Source::new("src/mocks/rich_method.cpp");
    let ctx = Context::new(&source);
    let node: Node = serde_json::from_str(include_str!("mocks/switch_stmt.json")).unwrap();
    let Clang::SwitchStmt(inner) = &node.kind else {panic!()};
    println!(
        "{}",
        inner
            .draw_algorithm_subgraph(&node, &ctx)
            .unwrap()
            .representation
    );
}
