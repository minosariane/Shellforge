use std::fs;

pub fn run() {

    println!("--format top: Top 10 reverse shells");
    println!("--format all: Yes\n");

    println!("Available formats:");

    let paths = fs::read_dir("./formats").unwrap();

    for path in paths {
        println!("{}", path.unwrap().file_name().display().to_string().replace(".sf", ""));
    }
}
