use std::path::PathBuf;
use clap::Clap;

#[derive(Clap, Debug)]
#[clap(name = "show-waifu", about = "View random anime fanart in your terminal!")]
struct Cli {
    /// Use a locally stored image path
    #[clap(short, long, parse(from_os_str), value_hint = clap::ValueHint::FilePath)]
    output: Option<PathBuf>,

    /// Use a link to an image
    #[clap(short, long)]
    url: Option<String>,

    /// Search for an image based on Safebooru tags.
    /// Pass as a string separated by spaces or commas.         
    /// You can also use negation with minus (-)
    #[clap(short, long)]
    tags: Option<String>,

    /// Resize the image to a provided height
    #[clap(short, long)]
    height: Option<u16>,

    /// Resize the image to a provided width
    #[clap(short, long)]
    width: Option<u16>,
}

fn main() {
    let args = Cli::parse();

    let mut image = std::process::Command::new("sh");

    if let Some(url) = args.url {
            let command = format!{"curl -s {} | viu -w 60 -", url};
            image.arg(command).output().expect("Error: Didn't work.");
    }
}