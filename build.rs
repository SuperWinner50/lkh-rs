fn main() {
    println!("cargo:rerun-if-changed=LKH/SRC");
    println!("cargo:rerun-if-changed=build.rs");

    let files = std::fs::read_dir("LKH/SRC")
        .unwrap()
        .filter_map(|f| {
            let f = f.unwrap();
            (f.file_type().unwrap().is_file() && f.file_name() != "Makefile")
                .then_some(f.path())
        });

    cc::Build::new()
        .files(files)
        .include("LKH/SRC/INCLUDE")
        .define("TREE_TYPE", "TWO_LEVEL_TREE")
        .flag("-Wall")
        .compile("lkh");
}