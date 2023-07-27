use crate::parsing::{CXXMethodDecl, Node};

use super::{
    draw_algorithm_subgraph,
    utils::{draw_box, draw_edge},
    Context, DrawAlgorithmSubgraph, GraphId, Subgraph,
};

impl DrawAlgorithmSubgraph for CXXMethodDecl {
    fn draw_algorithm_subgraph(&self, node: &Node, ctx: &Context) -> Option<Subgraph> {
        let input = GraphId(node.id.to_string());
        let output = GraphId(format!("{}_out", node.id));
        let mut representation = String::new();

        representation += &draw_box(&input, &self.name);
        let ctx = Context {
            return_label: Some(&output),
            ..*ctx
        };
        if let Some(inner) = draw_algorithm_subgraph(&node.inner[0], &ctx) {
            representation += &draw_edge(Some(&input), inner.input.as_ref());
            representation += &inner.representation;
            representation += &draw_edge(inner.output.as_ref(), Some(&output));
        } else {
            representation += &draw_edge(Some(&input), Some(&output));
        }
        representation += &draw_box(&output, "end");
        Some(Subgraph::new(representation, Some(input), Some(output)))
    }
}

#[test]
fn feature() {
    use super::Source;
    use crate::parsing::Clang;

    let source = Source::new("src/mocks/rich_method.cpp");
    let ctx = Context::new(&source);
    let node: Node = serde_json::from_str(include_str!("mocks/full_method.json")).unwrap();
    let Clang::CXXMethodDecl(inner) = &node.kind else {panic!()};
    println!("{}", inner.draw_algorithm_subgraph(&node, &ctx).unwrap().representation);
}
