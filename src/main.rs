use std::env::args;

use cpp_class_diagrams::{
    algorithm::get_algorithm_diagram,
    clang::get_source_ast,
    inheritance_diagram::get_inheritance_diagram,
    parsing::{collect_classes, collect_methods, CXXRecordDecl, Clang, Node},
};

fn main() {
    for file_name in args().skip(1) {
        let source = get_source_ast(&file_name);
        let node: Node = serde_json::from_str(&source).unwrap();
        let classes = collect_classes(node, &file_name);
        let inheritance_diagram = get_inheritance_diagram(&classes);
        println!("## {file_name}:\n");
        println!("### Inheritance:\n");
        println!("{inheritance_diagram}");

        println!("### Methods:\n");
        for class in classes {
            let Clang::CXXRecordDecl(CXXRecordDecl {name: class_name, ..}) = &class.kind else {panic!()};
            println!("#### {class_name}:\n");
            let methods = collect_methods(class);
            for method in methods {
                let algorithm_diagram = get_algorithm_diagram(&method, &file_name);
                println!("{algorithm_diagram}");
            }
        }
    }
}
