use crate::{
    algorithm::utils::draw_box,
    parsing::{Node, Other},
};

use super::{Context, DrawAlgorithmSubgraph, GraphId, Subgraph};

impl DrawAlgorithmSubgraph for Other {
    fn draw_algorithm_subgraph(&self, node: &Node, ctx: &Context) -> Option<Subgraph> {
        let id = format!("{}", node.id);
        let input = GraphId(id);
        let output = input.clone();
        let representation = draw_box(&input, ctx.source.get(node.kind.range().as_ref()?)?);
        Some(Subgraph::new(representation, Some(input), Some(output)))
    }
}

#[test]
fn feature() {
    use super::Source;
    use crate::parsing::Clang;

    let source = Source::new("src/mocks/rich_method.cpp");
    let ctx = Context::new(&source);
    let node: Node = serde_json::from_str(include_str!("mocks/decl_stmt.json")).unwrap();
    let Clang::DeclStmt(inner) = &node.kind else {panic!()};
    println!("{:?}", inner.draw_algorithm_subgraph(&node, &ctx));
}
