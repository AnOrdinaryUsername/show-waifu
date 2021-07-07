use colored::Colorize;
use rand::distributions::{Distribution, Uniform};
use reqwest::Error;
use serde::Deserialize;

use crate::api::reformat_search_tags;
use crate::app::Safebooru;

pub fn grab_random_image(args: Safebooru) -> String {
    let request_url = evaluate_arguments(&args);
    let data = match fetch_api_data(request_url) {
        Ok(json_data) => json_data,
        Err(error) => {
            eprintln!("{}\n", error);
            if args.questionable {
                println!(
                    "{}: Couldn't fetch API data. There's probably no \
                            questionable images associated with your tag(s).",
                    "help".green()
                );
            } else {
                println!(
                    "{}: Couldn't fetch API data. Try checking your \
                            tag(s) for errors.",
                    "help".green()
                );
            }

            std::process::exit(1);
        }
    };

    let mut rng = rand::thread_rng();
    let random_number = Uniform::from(0..data.len());
    let index = random_number.sample(&mut rng);

    let image = &data[index];

    let image_url = format!(
        "https://safebooru.org//images/{dir}/{img}?{id}",
        dir = image.directory,
        img = image.image,
        id = image.id
    );

    if args.details {
        let ImageData {
            rating,
            width,
            height,
            tags,
            ..
        } = image;

        let details = ImageInfo {
            url: &image_url,
            rating,
            width: *width,
            height: *height,
            tags: tags.split(' ').collect(),
        };

        match print_image_details(details) {
            Ok(_) => (),
            Err(error) => {
                eprintln!("{}\n", error);
                println!(
                    "{}: There was an error when printing the tags. Please try again later.",
                    "help".green()
                );
                std::process::exit(1);
            }
        }
    }

    image_url
}

fn evaluate_arguments(args: &Safebooru) -> String {
    let Safebooru {
        questionable, tags, ..
    } = args;

    let tags = match tags {
        Some(search_items) => search_items,
        None => "",
    };

    let search_tags = String::from(tags);
    let mut tags = reformat_search_tags(search_tags);

    if *questionable {
        tags.push_str("%20rating:questionable");
    }

    let tags = format!("&tags={}", tags);
    // No key needed for access
    let mut api =
        String::from("https://safebooru.org/index.php?page=dapi&s=post&q=index&limit=100&json=1");
    api.push_str(&tags);

    api
}

#[derive(Deserialize, Debug)]
struct ImageData {
    // Image URL
    directory: String,
    image: String,
    id: u32,

    // Image details
    rating: String,
    width: u32,
    height: u32,
    tags: String,
}

fn fetch_api_data(url: String) -> Result<Vec<ImageData>, Error> {
    let response = reqwest::blocking::get(&url)?;
    let data: Vec<ImageData> = response.json()?;

    Ok(data)
}

struct ImageInfo<'a> {
    url: &'a str,
    rating: &'a str,
    width: u32,
    height: u32,
    tags: Vec<&'a str>,
}

fn print_image_details(info: ImageInfo) -> Result<(), Box<dyn std::error::Error>> {
    use std::io::{self, Write};

    let ImageInfo {
        url,
        rating,
        width,
        height,
        tags,
    } = info;

    println!("✉️ {title}: {}", url, title = "Link".cyan());
    println!("⚖️ {title}: {}", rating, title = "Rating".cyan());
    println!(
        "📐 {title}: {w} x {h}",
        title = "Dimensions".cyan(),
        w = width,
        h = height
    );

    let stdout = io::stdout();
    let lock = stdout.lock();
    let mut buffer = io::BufWriter::new(lock);

    write!(buffer, "🏷️ {}:", "Tags".cyan())?;
    tags.iter().try_for_each(|tag| write!(buffer, " {}", tag))?;

    writeln!(buffer)?;

    Ok(())
}
