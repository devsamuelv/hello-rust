use std::io;

struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no path given");
    let args = Cli {
        path: std::path::PathBuf::from(path),
        pattern: pattern,
    };
    let path_str = args.path.as_os_str().to_str();

    println!("{path_str:?}");
}

fn iterate(array: &[i32]) {
    for i in array {
        println!("{i}")
    }
}
