use structopt::StructOpt;
use std::io::{BufRead, BufReader};

/// Search for a pattern in a file and display the lines that contain it
#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    // Pathbuf is like a string but for file system paths that is cross platform
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> std::io::Result<()> {
    //let args = Cli::from_args();

    //The expect function will cause the program to exit if the value could not be read
    //let content = std::fs::read_to_string(&args.path)
    //   .expect("Could not read file");

    let f = std::fs::File::open("C:\\Users\\Conley\\Code\\Rust\\grrs\\src\\main.rs")?;
    let mut reader = BufReader::new(f);

    let mut line = String::new();
    let len = reader.read_line(&mut line)?;
    println!("First line is {} bytes long", len);

    //let mut line = String::new();
    
    //for line in f.
    /*
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
    */
    Ok(())
}
