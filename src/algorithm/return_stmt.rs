use crate::parsing::{ReturnStmt, Node};

use super::{utils::{draw_box, draw_edge}, Context, DrawAlgorithmSubgraph, GraphId, Subgraph};

impl DrawAlgorithmSubgraph for ReturnStmt {
    fn draw_algorithm_subgraph(&self, node: &Node, ctx: &Context) -> Option<Subgraph> {
        let id = format!("{}", node.id);
        let input = GraphId(id);
        let mut representation = draw_box(&input, ctx.source.get(node.kind.range().as_ref()?)?);
        representation += &draw_edge(Some(&input), ctx.return_label);
        Some(Subgraph::new(representation, Some(input), None))
    }
}
