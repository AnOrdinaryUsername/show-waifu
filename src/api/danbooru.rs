use colored::Colorize;
use reqwest::StatusCode;
use serde::Deserialize;
use std::error::Error;
use std::fmt;

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

    let valid_data: Vec<&ImageData> = data
        .iter()
        .filter(|image| !image.file_url.is_empty())
        .collect();
    let image = &valid_data[0];
    let image_url = &valid_data[0].file_url;

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
            let login_info = format!("&login={}&api_key={}", username, api_key);
            api.push_str(login_info.as_str());
        }
    } else if let (Some(username), Some(api_key)) = check_env_variables() {
        let login_info = format!("&login={}&api_key={}", username, api_key);
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
    pixiv_id: Option<u32>,

    /*
     * file_url can potentially not be present under certain conditions
     * (e.g. banned artists and censored tags for users below Gold-level).
     *
     * When not present, it returns an empty string due to Default::default().
     * https://serde.rs/field-attrs.html#default
     */
    #[serde(default)]
    file_url: String,

    tag_string_character: String,
    tag_string_artist: String,
    rating: char,
    image_width: u32,
    image_height: u32,
    tag_string: String,
}

#[derive(Deserialize, Debug)]
struct FailureResponse {
    message: String,
}

#[derive(Debug)]
struct ResponseError(String);

impl fmt::Display for ResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for ResponseError {}

fn fetch_api_data(url: String) -> Result<Vec<ImageData>, Box<dyn Error>> {
    let response = reqwest::blocking::get(&url)?;
    let status_code = &response.status().to_string();

    match response.status() {
        StatusCode::OK => (),
        StatusCode::NO_CONTENT => {
            let message = format!(
                "{}: Request succeeded, but it contains no content.",
                status_code.yellow()
            );
            return Err(Box::new(ResponseError(message)));
        }
        _ => {
            let data: FailureResponse = response.json()?;
            let message = format!("{}: {}", &status_code.red(), data.message);
            return Err(Box::new(ResponseError(message)));
        }
    }

    let data: Vec<ImageData> = response.json()?;

    if data.is_empty() {
        let message = format!(
            "{}: Although the request succeeded, \
            there is no images associated with your tags.",
            status_code.green()
        );
        return Err(Box::new(ResponseError(message)));
    }

    Ok(data)
}

fn print_image_details(info: &ImageData) -> Result<(), Box<dyn std::error::Error>> {
    use std::io::{self, Write};

    let ImageData {
        source,
        pixiv_id,
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
            "✨ {title}: {}",
            tag_string_character,
            title = "Character".purple()
        );
    }

    if !source.is_empty() {
        // source sometimes contains a direct link to pixiv images, and if
        // you click on them it gives a 403 Forbidden
        if source.contains("pixiv") || source.contains("pximg") {
            let id = match pixiv_id {
                Some(id) => id,
                None => unreachable!(),
            };

            let pixiv_source = format!("https://pixiv.net/en/artworks/{}", id);

            println!("ℹ️ {title}: {}", pixiv_source, title = "Source".purple());
        } else {
            println!("ℹ️ {title}: {}", source, title = "Source".purple());
        }
    }

    if !tag_string_artist.is_empty() {
        println!(
            "🎨 {title}: {}",
            tag_string_artist,
            title = "Artist".purple()
        );
    }

    println!("✉️ {title}: {}", file_url, title = "Link".purple());

    match rating {
        's' => println!("⚖️ {title}: safe", title = "Rating".purple()),
        'q' => println!("⚖️ {title}: questionable", title = "Rating".purple()),
        'e' => println!("⚖️ {title}: explicit", title = "Rating".purple()),
        _ => (),
    }

    println!(
        "📐 {title}: {w} x {h}",
        title = "Dimensions".purple(),
        w = image_width,
        h = image_height
    );

    let tags: Vec<&str> = tag_string.split(' ').collect();
    let stdout = io::stdout();
    let lock = stdout.lock();
    let mut buffer = io::BufWriter::new(lock);

    write!(buffer, "🏷️ {}:", "Tags".purple())?;
    tags.iter().try_for_each(|tag| write!(buffer, " {}", tag))?;

    writeln!(buffer)?;

    Ok(())
}
