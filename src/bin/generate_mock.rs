use std::env::args;

use cpp_class_diagrams::clang::get_source_ast;

fn main() {
    for file_name in args().skip(1) {
        let source = get_source_ast(&file_name);
        println!("{source}");
    }
}