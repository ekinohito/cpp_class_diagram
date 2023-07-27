use crate::parsing::{ForStmt, Node};

use super::{
    draw_algorithm_subgraph,
    utils::{draw_dot, draw_edge, draw_hexagon},
    Context, DrawAlgorithmSubgraph, GraphId, Subgraph,
};

impl DrawAlgorithmSubgraph for ForStmt {
    fn draw_algorithm_subgraph(&self, node: &Node, ctx: &Context) -> Option<Subgraph> {
        let id = node.id.to_string();
        let continue_label = GraphId(id.clone() + "_con");
        let input = continue_label.clone();
        let break_label = GraphId(id.clone() + "_brk");
        let output = break_label.clone();
        let id = GraphId(id);

        let mut representation = String::new();
        let mut iter = node.inner.iter();
        let initial = iter.next();
        let _ = iter.next();
        let condition = iter.next();
        let iteration = iter.next();
        let loop_body = iter.next().expect("no loop body");
        fn extract<'a>(option: Option<&'a Node>, ctx: &'a Context<'a>) -> &'a str {
            option
                .and_then(|option| ctx.source.get(option.kind.range().as_ref().unwrap()))
                .unwrap_or("")
        }
        let label = format!(
            "{} {}; {}",
            extract(initial, ctx),
            extract(condition, ctx),
            extract(iteration, ctx)
        );
        representation += &draw_dot(&continue_label);
        representation += &draw_dot(&break_label);
        representation += &draw_hexagon(&id, &label);
        representation += &draw_edge(Some(&continue_label), Some(&id));
        representation += &draw_edge(Some(&id), Some(&break_label));

        let loop_ctx = Context {
            source: ctx.source,
            break_label: Some(&break_label),
            continue_label: Some(&continue_label),
            return_label: ctx.return_label,
            switch_label: ctx.switch_label,
        };

        if let Some(subgraph) = draw_algorithm_subgraph(loop_body, &loop_ctx) {
            representation += &draw_edge(Some(&id), subgraph.input.as_ref());
            representation += &subgraph.representation;
            representation += &draw_edge(subgraph.output.as_ref(), Some(&continue_label));
        } else {
            representation += &draw_edge(Some(&id), Some(&continue_label));
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
    let node: Node = serde_json::from_str(include_str!("mocks/for_stmt.json")).unwrap();
    let Clang::ForStmt(inner) = &node.kind else {panic!()};
    println!(
        "{}",
        inner
            .draw_algorithm_subgraph(&node, &ctx)
            .unwrap()
            .representation
    );
}
