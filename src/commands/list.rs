use crate::FORMATS_DIR;

pub fn run() {
    println!("--format top: Top 10 reverse shells");
    println!("--format all: Yes\n");

    println!("Available formats:");

    for file in FORMATS_DIR.files() {                                            //Return every filename without the extension
        println!("{}", file.path().file_stem().unwrap().to_string_lossy());
    }
}
