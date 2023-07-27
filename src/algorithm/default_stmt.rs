use crate::parsing::{DefaultStmt, Node};

use super::{
    draw_algorithm_subgraph,
    utils::{draw_dot, draw_edge, draw_edge_comment},
    Context, DrawAlgorithmSubgraph, GraphId, Subgraph,
};

impl DrawAlgorithmSubgraph for DefaultStmt {
    fn draw_algorithm_subgraph(&self, node: &Node, ctx: &Context) -> Option<Subgraph> {
        let id = GraphId(node.id.to_string());
        let mut output = Some(id.clone());
        let input = id.clone();

        let mut representation = String::new();
        representation += &draw_dot(&id);

        let mut iter = node.inner.iter();
        let body = iter.next().expect("No case body");
        representation += &draw_edge_comment(
            ctx.switch_label,
            Some(&id),
            "Default",
        );

        if let Some(subgraph) = draw_algorithm_subgraph(body, ctx) {
            representation += &draw_edge(Some(&id), subgraph.input.as_ref());
            representation += &subgraph.representation;
            output = subgraph.output;
        }

        Some(Subgraph::new(representation, Some(input), output))
    }
}

