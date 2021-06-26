use std::path::PathBuf;
use std::process::{Command, Stdio};
use clap::Clap;


#[derive(Clap, Debug)]
#[clap(name = "show-waifu", about = "View random anime fanart in your terminal!")]
struct Cli {
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
pub struct Random {
    /// Show data related to image (url, rating, width, height, tags)
    #[clap(short, long)]
    pub details: bool,

    /// Display only suggestive images
    #[clap(short, long)]
    pub suggestive: bool,

    /// Search for an image based on Safebooru tags.
    /// Pass as a string separated by spaces or commas.         
    /// Look at Safebooru;s cheatsheet for a full list of search options
    #[clap(short, long)]
    pub tags: Option<String>,
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
            Subcommand::Random(options) => {
                result = show_random_image(options);
            },
            Subcommand::File(file) => { 
                result = show_image_with_path(file.file_path);
            },
            Subcommand::Url(url) => {
                result = show_image_with_url(url.image_url);
            },
        };
    } else {
        std::process::exit(1)
    }

    result
}

fn show_random_image(args: Random) -> Result<(), Box<dyn std::error::Error>> {
    let image_url = crate::api::grab_random_image(args);
    let image_url = String::from(image_url);


    show_image_with_url(image_url)
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