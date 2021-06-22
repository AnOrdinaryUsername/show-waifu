use std::path::PathBuf;
use std::process::{Command, Stdio};
use clap::Clap;

#[derive(Clap, Debug)]
#[clap(name = "show-waifu", about = "View random anime fanart in your terminal!")]
struct Cli {
    /// Show data related to image (tags, width, height)
    #[clap(short, long)]
    details: bool,

    /// Use a path to a locally stored image
    #[clap(short, long, parse(from_os_str), value_hint = clap::ValueHint::FilePath)]
    file: Option<PathBuf>,

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

    if let Some(url) = args.url {
        show_image_with_url(url);
    } else if let Some(file) = args.file {
        show_image_with_path(file);
    }
}


fn show_image_with_url(image_url: String) -> () {
    let curl = match Command::new("curl")
            .arg(image_url)
            .stdout(Stdio::piped())
            .spawn() {
                Err(reason) => panic!("Couldn't spawn curl: {}", reason),
                Ok(process) => process,  
            };

    match Command::new("viu")
        .arg("-")
        .stdin(curl.stdout.unwrap())
        .status() {
            Err(reason) => panic!("Couldn't spawn viu: {}", reason),
            Ok(process) => process,  
        };
}

fn show_image_with_path(image_path: PathBuf) -> () {
    match Command::new("viu")
        .arg(image_path)
        .status() {
            Err(reason) => panic!("Couldn't spawn viu: {}", reason),
            Ok(process) => process,  
        };
}