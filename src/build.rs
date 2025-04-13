fn main() {
    println!("cargo:rerun-if-changed=tree-sitter-python");
    cc::Build::new()
        .include("tree-sitter-python/src")
        .file("tree-sitter-python/src/parser.c")
        .file("tree-sitter-python/src/scanner.cc")
        .cpp(true)
        .compile("tree-sitter-python");
}
