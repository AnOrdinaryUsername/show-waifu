use colored::Colorize;
use reqwest::Error;
use serde::Deserialize;

use crate::api::reformat_search_tags;
use crate::app::Danbooru;

pub fn grab_random_image(args: Danbooru) -> String {
    let request_url = evaluate_arguments(&args);
    let data = match fetch_api_data(request_url) {
        Ok(json_data) => json_data,
        Err(error) => {
            eprintln!("{}\n", error);
            std::process::exit(1);
        }
    };

    let image = &data[0];
    let image_url = &data[0].file_url;

    if args.details {
        match print_image_details(image) {
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

    image_url.to_string()
}

fn check_env_variables() -> (Option<String>, Option<String>) {
    use std::env;

    let mut login_info = (None, None);

    for (key, value) in env::vars() {
        let key = key.as_str();
        match key {
            "DANBOORU_USERNAME" => {
                login_info.0 = Some(value);
            }
            "DANBOORU_API_KEY" => {
                login_info.1 = Some(value);
            }
            &_ => (),
        }
    }

    login_info
}

fn evaluate_arguments(args: &Danbooru) -> String {
    let mut api = String::from("https://danbooru.donmai.us/posts.json?random=true");

    if let Some(username) = &args.username {
        if let Some(api_key) = &args.key {
            let login_info = format!("?login={}&api_key={}", username, api_key);
            api.push_str(login_info.as_str());
        }
    } else if let (Some(username), Some(api_key)) = check_env_variables() {
        let login_info = format!("?login={}&api_key={}", username, api_key);
        api.push_str(login_info.as_str());
    }

    let Danbooru {
        safe,
        questionable,
        explicit,
        tags,
        ..
    } = args;

    let tags = match tags {
        Some(search_items) => search_items,
        None => "",
    };

    let search_tags = String::from(tags);
    let mut tags = reformat_search_tags(search_tags);

    if *safe {
        tags.push_str("%20rating:s");
    } else if *questionable {
        tags.push_str("%20rating:q");
    } else if *explicit {
        tags.push_str("%20rating:e");
    }

    let tags = format!("&tags={}", tags);
    api.push_str(&tags);

    api
}

#[derive(Deserialize, Debug)]
struct ImageData {
    source: String,
    file_url: String,
    tag_string_character: String,
    tag_string_artist: String,
    rating: char,
    image_width: u32,
    image_height: u32,
    tag_string: String,
}

fn fetch_api_data(url: String) -> Result<Vec<ImageData>, Error> {
    let response = reqwest::blocking::get(&url)?;
    let data: Vec<ImageData> = response.json()?;

    Ok(data)
}

fn print_image_details(info: &ImageData) -> Result<(), Box<dyn std::error::Error>> {
    use std::io::{self, Write};

    let ImageData {
        source,
        file_url,
        tag_string_character,
        tag_string_artist,
        rating,
        image_height,
        image_width,
        tag_string,
    } = info;

    if !tag_string_character.is_empty() {
        println!(
            "{title}: {}",
            tag_string_character,
            title = "Character".red()
        );
    }

    if !source.is_empty() {
        println!("{title}: {}", source, title = "Source".red());
    }

    if !tag_string_artist.is_empty() {
        println!("{title}: {}", tag_string_artist, title = "Artist".red());
    }

    println!("{title}: {}", file_url, title = "Link".red());

    match rating {
        's' => println!("{title}: safe", title = "Rating".red()),
        'q' => println!("{title}: questionable", title = "Rating".red()),
        'e' => println!("{title}: explicit", title = "Rating".red()),
        _ => (),
    }

    println!(
        "{title}: {w} x {h}",
        title = "Dimensions".red(),
        w = image_width,
        h = image_height
    );

    let tags: Vec<&str> = tag_string.split(' ').collect();
    let stdout = io::stdout();
    let lock = stdout.lock();
    let mut buffer = io::BufWriter::new(lock);

    write!(buffer, "🏷️ {}:", "Tags".red())?;
    tags.iter().try_for_each(|tag| write!(buffer, " {}", tag))?;

    writeln!(buffer)?;

    Ok(())
}
