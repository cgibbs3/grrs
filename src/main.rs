use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it
#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    // Pathbuf is like a string but for file system paths that is cross platform
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();

    //The expect function will cause the program to exit if the value could not be read
    let content = std::fs::read_to_string(&args.path)
        .expect("Could not read file");

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

}
