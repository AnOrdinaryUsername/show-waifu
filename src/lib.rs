use std::path::PathBuf;
use std::process::{Command, Stdio};
use clap::Clap;


#[derive(Clap, Debug)]
#[clap(name = "show-waifu", about = "View random anime fanart in your terminal!")]
pub struct Cli {
    /// Resize the image to a provided height
    #[clap(short, long)]
    height: Option<u16>,

    /// Resize the image to a provided width
    #[clap(short, long)]
    width: Option<u16>,

    #[clap(subcommand)]
    command: Option<Subcommand>,
}

#[derive(Clap, Debug)]
enum Subcommand {
    #[clap(name = "random")]
    Random(Random),

    #[clap(name = "url")]
    Url(Url),

    #[clap(name = "file")]
    File(File),
}

/// View a random image from Safebooru
#[derive(Clap, Debug)]
struct Random {
    /// Show data related to image (url, width, height, tags)
    #[clap(short, long)]
    details: bool,

    /// Search for an image based on Safebooru tags.
    /// Pass as a string separated by spaces or commas.         
    /// You can also use negation with minus (-)
    #[clap(short, long)]
    tags: Option<String>,
}

/// Pass a link for viewing
#[derive(Clap, Debug)]
struct Url {
    image_url: String,
}

/// Pass a local file for viewing
#[derive(Clap, Debug)]
 struct File {
    #[clap(parse(from_os_str), value_hint = clap::ValueHint::FilePath)]
    file_path: PathBuf,
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let result: Result<(), Box<dyn std::error::Error>>;

    let Cli { width, height, .. } = args;

    if let Some(command) = args.command {
        match command {
            // TODO: Check for Some() and None() (Custom error)
            // for each subcommand
            Subcommand::Random(options) => {
                result = show_random_image(options.tags);
            },
            Subcommand::File(file) => { 
                result = show_image_with_path(file.file_path);
            },
            Subcommand::Url(url) => {
                result = show_image_with_url(url.image_url);
            },
        };
    } else {
        result = show_random_image(None);
    }

    result
}


fn grab_random_image() -> String {
    // No key needed for access
    let api = "https://safebooru.org/index.php?page=dapi&s=post&q=index&limit=100&json=1";
    let api = String::from(api);

    api
}


fn show_random_image(tags: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    // let url = String::from(grab_random_image());

    let url = String::from("https://i.redd.it/h7dae4o0uk461.jpg");

    show_image_with_url(url)
}


fn show_image_with_url(image_url: String) -> Result<(), Box<dyn std::error::Error>> {
    let curl = Command::new("curl")
            .arg("-s")
            .arg(image_url)
            .stdout(Stdio::piped())
            .spawn()?;

    Command::new("viu")
        .arg("-")
        .stdin(curl.stdout.unwrap())
        .status()?;
    
    Ok(())
}

fn show_image_with_path(image_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    Command::new("viu")
        .arg(image_path)
        .status()?;

    Ok(())
}