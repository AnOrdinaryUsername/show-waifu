use atty::Stream;
use clap::Clap;
use std::error::Error;
use std::path::PathBuf;
use viuer::{print, print_from_file};

#[derive(Clap, Debug)]
#[clap(
    name = "show-waifu",
    about = "View random anime fanart in your terminal!

    ┏━━ Rating Colors ━━━━━━┓
    ┃                       ┃  
    ┃ safe = \x1b[0;36mcyan\x1b[0m           ┃ 
    ┃ questionable = \x1b[0;33myellow\x1b[0m ┃
    ┃ explicit = \x1b[0;31mred\x1b[0m        ┃   
    ┃                       ┃   
    ┗━━━━━━━━━━━━━━━━━━━━━━━┛
    
NOTE: Each rating has a different meaning depending on site. See
the respective subcommand's help for more details."
)]
struct Cli {
    /// Resize the image to a provided height
    #[clap(short, long)]
    height: Option<u32>,

    /// Resize the image to a provided width
    #[clap(short, long)]
    width: Option<u32>,

    #[clap(subcommand)]
    subcommand: Option<Subcommand>,
}

#[derive(Clap, Debug)]
enum Subcommand {
    #[clap(name = "safe")]
    Safebooru(Safebooru),

    #[clap(name = "dan")]
    Danbooru(Danbooru),

    #[clap(name = "url")]
    Url(Url),

    #[clap(name = "file")]
    File(File),
}

/// View a random image from Safebooru
#[derive(Clap, Debug)]
pub struct Safebooru {
    /// Show data related to image (url, rating, dimensions, tags)
    #[clap(short, long)]
    pub details: bool,

    /// Only display images with suggestive content
    #[clap(short, long)]
    pub questionable: bool,

    /// Search for an image based on Safebooru tags.
    /// Pass as a string separated by spaces or commas.         
    /// Look at Safebooru's cheatsheet for a full list of search options
    #[clap(short, long)]
    pub tags: Option<String>,
}

/// View a random image from Danbooru
#[derive(Clap, Debug)]
pub struct Danbooru {
    /// Show data related to image (artist, source, character, url, rating, dimensions, tags)
    #[clap(short, long)]
    pub details: bool,

    /// Only display images lacking sexual content
    #[clap(short, long)]
    pub safe: bool,

    /// Only display images with some nudity or sexual content
    #[clap(short, long)]
    pub questionable: bool,

    /// Only display images with explicit sexual content
    #[clap(short, long)]
    pub explicit: bool,

    /// Search for an image based on Danbooru tags.
    /// Pass as a string separated by spaces or commas.         
    /// Look at Danbooru's cheatsheet for a full list of search options
    #[clap(short, long)]
    pub tags: Option<String>,
}

/// View an image from a url
#[derive(Clap, Debug)]
struct Url {
    /// The URL of an image (e.g. https://i.redd.it/7tycieudz3c61.png)
    image_url: String,
}

/// View an image from your file system
#[derive(Clap, Debug)]
struct File {
    /// The path to an image file (e.g. ~/Pictures/your-image.jpg)
    #[clap(parse(from_os_str), value_hint = clap::ValueHint::FilePath)]
    file_path: PathBuf,
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    let result: Result<(), Box<dyn Error>>;

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

    if let Some(subcommand) = args.subcommand {
        match subcommand {
            Subcommand::Danbooru(args) => {
                let dan_args = Danbooru { ..args };
                let dan_args = Subcommand::Danbooru(dan_args);
                result = show_random_image(dan_args, config);
            }
            Subcommand::Safebooru(args) => {
                let safe_args = Safebooru { ..args };
                let safe_args = Subcommand::Safebooru(safe_args);
                result = show_random_image(safe_args, config);
            }
            Subcommand::File(file) => {
                result = show_image_with_path(file.file_path, config);
            }
            Subcommand::Url(url) => {
                result = show_image_with_url(url.image_url, config);
            }
        };
    } else {
        let default_options = Safebooru {
            details: false,
            questionable: false,
            tags: None,
        };

        let default = Subcommand::Safebooru(default_options);

        result = show_random_image(default, config);
    }

    result
}

fn show_random_image(args: Subcommand, config: viuer::Config) -> Result<(), Box<dyn Error>> {
    use crate::api::{danbooru, safebooru};

    let image_url = match args {
        Subcommand::Danbooru(args) => danbooru::grab_random_image(args),
        Subcommand::Safebooru(args) => safebooru::grab_random_image(args),
        _ => panic!(
            "Invalid subcommand passed to show_random_image. \
                Only valid ones are 'Danbooru' and 'Safebooru'."
        ),
    };

    show_image_with_url(image_url, config)
}

fn show_image_with_url(image_url: String, config: viuer::Config) -> Result<(), Box<dyn Error>> {
    let image_bytes = reqwest::blocking::get(&image_url)?.bytes()?;
    let image = image::load_from_memory(&image_bytes)?;

    print(&image, &config)?;

    Ok(())
}

fn show_image_with_path(image_path: PathBuf, config: viuer::Config) -> Result<(), Box<dyn Error>> {
    print_from_file(image_path, &config)?;

    Ok(())
}

fn show_image_from_stdin(config: viuer::Config) -> Result<(), Box<dyn Error>> {
    use std::io::{stdin, Read};

    let stdin = stdin();
    let mut handle = stdin.lock();

    let mut buffer: Vec<u8> = Vec::new();
    let _ = handle.read_to_end(&mut buffer)?;

    let image = image::load_from_memory(&buffer)?;
    print(&image, &config)?;

    Ok(())
}
