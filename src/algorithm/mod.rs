use std::fmt::Display;

use crate::{
    parsing::{Clang, Node},
    source::Source,
};

mod compound_stmt;
mod cxx_method_decl;
mod decl_stmt;
mod if_stmt;
mod other;
mod utils;
mod while_stmt;
mod do_stmt;
mod break_stmt;
mod continue_stmt;
mod return_stmt;
mod for_stmt;
mod switch_stmt;
mod case_stmt;
mod default_stmt;

#[derive(Debug, Clone)]
pub struct GraphId(String);

impl Display for GraphId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct Context<'a> {
    pub source: &'a Source,
    pub break_label: Option<&'a GraphId>,
    pub continue_label: Option<&'a GraphId>,
    pub return_label: Option<&'a GraphId>,
    pub switch_label: Option<&'a GraphId>,
}

impl<'a> Context<'a> {
    pub fn new(source: &'a Source) -> Self {
        Self {
            source,
            break_label: None,
            continue_label: None,
            return_label: None,
            switch_label: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Subgraph {
    representation: String,
    input: Option<GraphId>,
    output: Option<GraphId>,
}

impl Subgraph {
    pub fn new(representation: String, input: Option<GraphId>, output: Option<GraphId>) -> Self {
        Self {
            representation,
            input,
            output,
        }
    }
}

pub trait DrawAlgorithmSubgraph {
    fn draw_algorithm_subgraph(&self, node: &Node, ctx: &Context) -> Option<Subgraph>;
}

pub fn draw_algorithm_subgraph(node: &Node, ctx: &Context) -> Option<Subgraph> {
    match &node.kind {
        Clang::CXXRecordDecl(_) => None,
        Clang::CXXMethodDecl(inner) => inner.draw_algorithm_subgraph(node, ctx),
        Clang::FieldDecl(_) => None,
        Clang::AccessSpecDecl(_) => None,
        Clang::CompoundStmt(inner) => inner.draw_algorithm_subgraph(node, ctx),
        Clang::DeclStmt(inner) => inner.draw_algorithm_subgraph(node, ctx),
        Clang::IfStmt(inner) => inner.draw_algorithm_subgraph(node, ctx),
        Clang::WhileStmt(inner) => inner.draw_algorithm_subgraph(node, ctx),
        Clang::DoStmt(inner) => inner.draw_algorithm_subgraph(node, ctx),
        Clang::BreakStmt(inner) => inner.draw_algorithm_subgraph(node, ctx),
        Clang::ContinueStmt(inner) => inner.draw_algorithm_subgraph(node, ctx),
        Clang::ReturnStmt(inner) => inner.draw_algorithm_subgraph(node, ctx),
        Clang::SwitchStmt(inner) => inner.draw_algorithm_subgraph(node, ctx),
        Clang::CaseStmt(inner) => inner.draw_algorithm_subgraph(node, ctx),
        Clang::DefaultStmt(inner) => inner.draw_algorithm_subgraph(node, ctx),
        Clang::Other(inner) => inner.draw_algorithm_subgraph(node, ctx),
        Clang::ForStmt(inner) => inner.draw_algorithm_subgraph(node, ctx),
    }
}

pub fn get_algorithm_diagram(node: &Node, file_name: &str) -> String {
    let source = Source::new(file_name);
    let ctx = Context::new(&source);
    let algorithm_subgraph: Option<Subgraph> = draw_algorithm_subgraph(node, &ctx);
    algorithm_subgraph.map_or("ERR".to_owned(), |graph| format!("```mermaid\nflowchart TB\n{}```", graph.representation))
}

#[test]
fn feature() {
    let source = Source::new("src/mocks/rich_method.cpp");
    let ctx = Context::new(&source);
    let node: Node = serde_json::from_str(include_str!("mocks/full_method.json")).unwrap();
    println!(
        "{}",
        draw_algorithm_subgraph(&node, &ctx).unwrap().representation
    );
}
