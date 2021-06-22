use std::path::PathBuf;
use std::process::{Child, Command, Stdio, Output};
use clap::Clap;

#[derive(Clap, Debug)]
#[clap(name = "show-waifu", about = "View random anime fanart in your terminal!")]
struct Cli {
    /// Use a path to a locally stored image
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

    if let Some(url) = args.url {
        let value = use_viu(url);
        eprintln!("{}", String::from_utf8_lossy(&value.stderr));
    }

}

fn use_curl(url: String) -> Child {
    let cmd_curl = match Command::new("curl")
            .arg(url)
            .stdout(Stdio::piped())
            .spawn() {
                Err(reason) => panic!("Couldn't spawn curl: {}", reason),
                Ok(process) => process,  
            };
    
    cmd_curl
}

fn use_viu(image_url: String) -> Output {
    let curl = use_curl(image_url);

    let cmd_viu = Command::new("kitty")
        .arg("kitten")
        .arg("icat")
        .stdin(curl.stdout.unwrap())
        .output()
        .unwrap_or_else(|reason| { 
            panic!("Couldn't spawn viu: {}", reason)
        });

    cmd_viu
}