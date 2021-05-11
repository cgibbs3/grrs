use structopt::StructOpt;

struct CLI
{
    pattern: String,
    // Pathbuf is like a string but for file system paths that is cross platform
    path: std::path::PathBuf,
}

fn main() {
    println!("Hello, world!");

    let pattern = std::env::args().nth(1).expect("No pattern given");
    let path = std::env::args().nth(2).expect("No path given");
}
