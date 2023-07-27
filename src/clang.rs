use std::process::{Command, Stdio};

pub fn get_source_ast(input_file_name: &str) -> String {
    let clang = Command::new("clang")
        .args([
            "-Xclang",
            "-ast-dump=json",
            "-fsyntax-only",
            input_file_name,
        ])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start clang");
    let output = clang.wait_with_output().expect("Failed to wait on clang");
    String::from_utf8(output.stdout).expect("Invalid UTF-8")
}