use clap::Parser;

#[derive(Parser)]
#[command(name = "gdsr-viewer", version, about = "A test viewer binary")]
struct Cli {
    /// File to open
    file: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    if let Some(file) = cli.file {
        println!("Opening: {file}");
    } else {
        println!("{}", gdsr::hello());
    }
}
