use crate::parsing::{ContinueStmt, Node};

use super::{utils::{draw_box, draw_edge}, Context, DrawAlgorithmSubgraph, GraphId, Subgraph};

impl DrawAlgorithmSubgraph for ContinueStmt {
    fn draw_algorithm_subgraph(&self, node: &Node, ctx: &Context) -> Option<Subgraph> {
        let id = format!("{}", node.id);
        let input = GraphId(id);
        let mut representation = draw_box(&input, ctx.source.get(node.kind.range().as_ref()?)?);
        representation += &draw_edge(Some(&input), ctx.continue_label);
        Some(Subgraph::new(representation, Some(input), None))
    }
}
