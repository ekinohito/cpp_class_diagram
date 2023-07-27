use serde::Deserialize;

use clang_ast::{SourceLocation, SourceRange};

use super::{
    AccessSpecDecl, Base, BreakStmt, CXXMethodDecl, CXXRecordDecl, CaseStmt, CompoundStmt,
    ContinueStmt, DeclStmt, DoStmt, FieldDecl, ForStmt, IfStmt, Other, ReturnStmt, SwitchStmt,
    WhileStmt, default_stmt::DefaultStmt,
};

pub type Node = clang_ast::Node<Clang>;

#[derive(Deserialize, Debug, Clone)]
pub enum Clang {
    /// Represents a C++ struct/union/class.
    CXXRecordDecl(CXXRecordDecl),
    /// Represents a static or instance method of a struct/union/class.
    CXXMethodDecl(CXXMethodDecl),
    /// Represents a member of a struct/union/class.
    FieldDecl(FieldDecl),
    /// Represents an access specifier followed by colon ':'.
    AccessSpecDecl(AccessSpecDecl),

    /// This represents a group of statements like { stmt stmt }.
    CompoundStmt(CompoundStmt),
    /// Adaptor class for mixing declarations with statements and expressions.
    DeclStmt(DeclStmt),
    /// This represents an if/then/else.
    IfStmt(IfStmt),
    /// This represents a 'while' stmt.
    WhileStmt(WhileStmt),
    /// This represents a 'do/while' stmt.
    DoStmt(DoStmt),
    /// This represents a break.
    BreakStmt(BreakStmt),
    /// This represents a continue.
    ContinueStmt(ContinueStmt),
    /// This represents a return, optionally of an expression: return; return 4;.
    ReturnStmt(ReturnStmt),
    /// This represents a 'switch' stmt.
    SwitchStmt(SwitchStmt),
    /// Represent a case statement.
    CaseStmt(CaseStmt),
    /// Represent a default case statement.
    DefaultStmt(DefaultStmt),
    /// This represents a 'for (init;cond;inc)' stmt.
    ForStmt(ForStmt),

    /// Catch-all class for useless nodes
    Other(Other),
}

impl Clang {
    fn base(&self) -> &Base {
        match self {
            Clang::CXXRecordDecl(CXXRecordDecl { base, .. })
            | Clang::CXXMethodDecl(CXXMethodDecl { base, .. })
            | Clang::FieldDecl(FieldDecl { base, .. })
            | Clang::AccessSpecDecl(AccessSpecDecl { base, .. })
            | Clang::CompoundStmt(CompoundStmt { base, .. })
            | Clang::DeclStmt(DeclStmt { base, .. })
            | Clang::IfStmt(IfStmt { base, .. })
            | Clang::WhileStmt(WhileStmt { base })
            | Clang::DoStmt(DoStmt { base })
            | Clang::BreakStmt(BreakStmt { base })
            | Clang::ContinueStmt(ContinueStmt { base })
            | Clang::ReturnStmt(ReturnStmt { base })
            | Clang::SwitchStmt(SwitchStmt { base })
            | Clang::CaseStmt(CaseStmt { base })
            | Clang::DefaultStmt(DefaultStmt { base })
            | Clang::ForStmt(ForStmt { base })
            | Clang::Other(Other { base }) => base,
        }
    }

    pub fn loc(&self) -> &Option<SourceLocation> {
        &self.base().loc
    }

    pub fn range(&self) -> &Option<SourceRange> {
        &self.base().range
    }

    fn matches_file_name(&self, file_name: &str) -> bool {
        let loc = self.loc();
        let Some(loc) = loc else {
            return false
        };
        loc.spelling_loc
            .as_ref()
            .map_or(false, |loc| *loc.file == *file_name)
    }
}

pub fn filter_nodes_by_file(mut root_node: Node, file_name: &str) -> Node {
    root_node
        .inner
        .retain(|node| node.kind.matches_file_name(file_name));
    root_node
}

fn collect_classes_from_node(node: Node) -> Vec<Node> {
    let mut result: Vec<Node> = vec![];
    if let Clang::CXXRecordDecl(CXXRecordDecl { tag_used, .. }) = &node.kind {
        // record must actually be a class (not a union or struct)
        let is_correct_tag = tag_used == "class";
        // class must be complete (not just declared)
        let is_complete = !node.inner.is_empty();

        if is_correct_tag && is_complete {
            result.push(node.clone());
        }
    };
    for node in node.inner {
        result.extend(collect_classes_from_node(node))
    }
    result
}

pub fn collect_classes(root_node: Node, file_name: &str) -> Vec<Node> {
    let filtered_node = filter_nodes_by_file(root_node, file_name);
    collect_classes_from_node(filtered_node)
}

pub fn collect_methods(node: Node) -> Vec<Node> {
    let mut result: Vec<Node> = vec![];
    for node in node.inner {
        if let Clang::CXXMethodDecl(_) = &node.kind {
            result.push(node.clone());
        };
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::clang::get_source_ast;

    use super::*;

    #[test]
    fn class_parsing() {
        let input = get_source_ast("./src/mocks/class.cpp");
        println!("{input}");
        let node: Node = serde_json::from_str(&input).unwrap();
        println!("{node:?}");
    }

    #[test]
    fn method_parsing() {
        let input = get_source_ast("./src/mocks/rich_method.cpp");
        println!("{input}");
        let node: Node = serde_json::from_str(&input).unwrap();
        println!("{node:?}");
    }

    #[test]
    fn class_extraction() {
        let input = get_source_ast("./src/mocks/class_struct_union.cpp");
        let node: Node = serde_json::from_str(&input).unwrap();
        let classes = collect_classes(node, "./src/mocks/class_struct_union.cpp");
        classes.iter().for_each(|class| println!("{class:?}\n"));
    }

    #[test]
    fn incomplete_class() {
        let input = get_source_ast("./src/mocks/incomplete_class.cpp");
        let node: Node = serde_json::from_str(&input).unwrap();
        let classes = collect_classes(node, "./src/mocks/incomplete_class.cpp");
        assert_eq!(classes.len(), 1);
        classes.iter().for_each(|class| println!("{class:?}\n"));
    }

    #[test]
    fn inner_class() {
        let input = get_source_ast("./src/mocks/inner_class.cpp");
        println!("{input}");
        let node: Node = serde_json::from_str(&input).unwrap();
        let classes = collect_classes(node, "./src/mocks/inner_class.cpp");
        assert_eq!(classes.len(), 1);
        classes.iter().for_each(|class| println!("{class:?}\n"));
    }

    #[test]
    fn inheritance() {
        let input = get_source_ast("./src/mocks/inheritance.cpp");
        println!("{input}");
        let node: Node = serde_json::from_str(&input).unwrap();
        let classes = collect_classes(node, "./src/mocks/inheritance.cpp");
        assert_eq!(classes.len(), 4);
        classes.iter().for_each(|class| println!("{class:?}\n"));
    }

    #[test]
    fn stress_test() {
        let input = get_source_ast("./src/mocks/hello_world.cpp");
        let node: Node = serde_json::from_str(&input).unwrap();
        let classes = collect_classes(node, "./src/mocks/inheritance.cpp");
        assert_eq!(classes.len(), 0);
    }
}
