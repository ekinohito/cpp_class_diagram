use crate::parsing::{AccessSpecDecl, CXXMethodDecl, CXXRecordDecl, Clang, FieldDecl, Node};

pub fn get_inheritance_diagram(class_nodes: &Vec<Node>) -> String {
    let mut result: String = "```mermaid\nclassDiagram\nx\n".to_owned();

    for node in class_nodes {
        let Clang::CXXRecordDecl(CXXRecordDecl { name, bases, .. }) = &node.kind else {panic!("Node isn't class")};
        result += &format!("class {name}\n");
        for base in bases {
            result += &format!("{} <|-- {}\n", base.cxx_type.qual_type, name);
        }
        let mut current_visibility = "-";
        for inner_node in &node.inner {
            match &inner_node.kind {
                Clang::AccessSpecDecl(AccessSpecDecl { access, .. }) => {
                    current_visibility = match access.as_str() {
                        "public" => "+",
                        "protected" => "#",
                        "private" => "-",
                        _ => current_visibility,
                    }
                }
                Clang::CXXMethodDecl(CXXMethodDecl {
                    name: method_name, ..
                }) => {
                    result += &format!("{name} : {current_visibility}{method_name}()\n");
                }
                Clang::FieldDecl(FieldDecl {
                    name: field_name,
                    cxx_type,
                    ..
                }) => {
                    result += &format!(
                        "{name} : {current_visibility}{} {field_name}\n",
                        cxx_type.qual_type
                    );
                }
                _ => (),
            }
        }
    }
    result += "```";
    result
}

#[test]
fn inheritance() {
    use crate::{clang::get_source_ast, parsing::collect_classes};

    let input = get_source_ast("./src/mocks/inheritance.cpp");
    println!("{input}");
    let node: Node = serde_json::from_str(&input).unwrap();
    let class_nodes = collect_classes(node, "./src/mocks/inheritance.cpp");
    println!("{}", get_inheritance_diagram(&class_nodes));
}
