use crate::parsing::{CaseStmt, Node};

use super::{
    draw_algorithm_subgraph,
    utils::{draw_dot, draw_edge, draw_edge_comment},
    Context, DrawAlgorithmSubgraph, GraphId, Subgraph,
};

impl DrawAlgorithmSubgraph for CaseStmt {
    fn draw_algorithm_subgraph(&self, node: &Node, ctx: &Context) -> Option<Subgraph> {
        let id = GraphId(node.id.to_string());
        let mut output = Some(id.clone());
        let input = id.clone();

        let mut representation = String::new();
        representation += &draw_dot(&id);
        // representation += &draw_box(&id, ctx.source.get(node.kind.range().as_ref()?)?);

        let mut iter = node.inner.iter();
        let condition = iter.next().expect("No condition");
        let body = iter.next().expect("No case body");
        representation += &draw_edge_comment(
            ctx.switch_label,
            Some(&id),
            condition
                .kind
                .range()
                .as_ref()
                .and_then(|range| ctx.source.get(range))
                .unwrap_or_default(),
        );

        if let Some(subgraph) = draw_algorithm_subgraph(body, ctx) {
            representation += &draw_edge(Some(&id), subgraph.input.as_ref());
            representation += &subgraph.representation;
            output = subgraph.output;
        }

        Some(Subgraph::new(representation, Some(input), output))
    }
}
