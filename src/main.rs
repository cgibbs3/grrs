use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it
#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    // Pathbuf is like a string but for file system paths that is cross platform
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();

    let pattern = std::env::args().nth(1).expect("No pattern given");
    let path = std::env::args().nth(2).expect("No path given");
}
