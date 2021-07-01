use colored::Colorize;
use rand::distributions::{Distribution, Uniform};
use regex::Regex;
use reqwest::Error;
use serde::Deserialize;

use crate::app::Random;

pub fn grab_random_image(options: Random) -> String {
    let data = match fetch_api_data(&options) {
        Ok(json_data) => json_data,
        Err(error) => {
            eprintln!("{}\n", error);
            if options.suggestive {
                println!(
                    "{}: Couldn't fetch API data. There's probably no \
                            suggestive images associated with your tag(s).",
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

    if options.details {
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
        };
    }

    image_url
}

fn evaluate_arguments(options: &Random) -> String {
    let Random {
        suggestive, tags, ..
    } = options;

    let tags = match tags {
        Some(search_items) => search_items,
        None => "",
    };

    let mut search_tags = String::from(tags);

    if *suggestive {
        if search_tags.is_empty() {
            search_tags.push_str("rating:questionable");
        } else {
            search_tags.push_str("%20rating:questionable");
        }
    }

    let extra_spaces = Regex::new(r"\s{2,}").unwrap();
    let delimiters = Regex::new(r"[,\s]").unwrap();

    // Remove excess spaces (2 or more)
    let search_tags = extra_spaces.replace_all(&search_tags, "");
    // Replace commas and spaces with %20
    let search_tags = delimiters.replace_all(&search_tags, "%20");

    let tags = format!("&tags={}", search_tags);
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

fn fetch_api_data(options: &Random) -> Result<Vec<ImageData>, Error> {
    let request_url = evaluate_arguments(options);

    let response = reqwest::blocking::get(&request_url)?;
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
