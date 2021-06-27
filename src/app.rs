use std::path::PathBuf;
use viuer::{print_from_file, print};
use clap::Clap;
use atty::Stream;

#[derive(Clap, Debug)]
#[clap(name = "show-waifu", about = "View random anime fanart in your terminal!")]
struct Cli {
    /// Resize the image to a provided height
    #[clap(short, long)]
    height: Option<u32>,

    /// Resize the image to a provided width
    #[clap(short, long)]
    width: Option<u32>,

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
    /// Look at Safebooru's cheatsheet for a full list of search options
    #[clap(short, long)]
    pub tags: Option<String>,
}

/// View an image from a url
#[derive(Clap, Debug)]
struct Url {
    image_url: String,
}

/// View an image from your file system
#[derive(Clap, Debug)]
 struct File {
    #[clap(parse(from_os_str), value_hint = clap::ValueHint::FilePath)]
    file_path: PathBuf,
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let result: Result<(), Box<dyn std::error::Error>>;

    let Cli { width, height, .. } = args;

    let config = viuer::Config {
        width,
        height,
        absolute_offset: false,
        ..Default::default()
    };

    // Read from stdin
    if atty::isnt(Stream::Stdin) {
        result = show_image_from_stdin(config);
        return result;
    }

    if let Some(command) = args.command {
        match command {
            Subcommand::Random(options) => {
                result = show_random_image(options, config);
            },
            Subcommand::File(file) => { 
                result = show_image_with_path(file.file_path, config);
            },
            Subcommand::Url(url) => {
                result = show_image_with_url(url.image_url, config);
            },
        };
    } else {
        let default_options = Random {
            details: false,
            suggestive: false,
            tags: None,
        };

        result = show_random_image(default_options, config);
    }

    result
}

fn show_random_image(args: Random, config: viuer::Config) -> Result<(), Box<dyn std::error::Error>> {
    let image_url = crate::api::grab_random_image(args);
    let image_url = String::from(image_url);

    show_image_with_url(image_url, config)
}

fn show_image_with_url(image_url: String, config: viuer::Config) -> Result<(), Box<dyn std::error::Error>> {
    let image_bytes = reqwest::blocking::get(&image_url)?.bytes()?;
    let image = image::load_from_memory(&image_bytes)?;

    print(&image, &config)?;

    Ok(())
}

fn show_image_with_path(image_path: PathBuf, config: viuer::Config) -> Result<(), Box<dyn std::error::Error>> {
    print_from_file(image_path, &config)?;

    Ok(())
}

fn show_image_from_stdin(config: viuer::Config) -> Result<(), Box<dyn std::error::Error>> {
    use std::io::{stdin, Read};

    let stdin = stdin();
    let mut handle = stdin.lock();

    let mut buf: Vec<u8> = Vec::new();
    let _ = handle.read_to_end(&mut buf)?;

    let image = image::load_from_memory(&buf)?;
    print(&image, &config)?;

    Ok(())
}