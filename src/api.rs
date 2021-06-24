use regex::Regex;
use std::collections::HashMap;

pub struct RandomArgs {
    has_details: bool,
    has_suggestive_content: bool,
    tags: String,
}


fn evaluate_arguments(options: RandomArgs) -> String {
    let RandomArgs { has_suggestive_content, tags, .. } = options;
    
    let search_tags = String::from(tags);

    if has_suggestive_content {
        search_tags.push_str("%20rating:suggestive");
    }

    let extra_spaces = Regex::new(r"\s{2,}").unwrap();
    let delimiters = Regex::new(r"[,\s]").unwrap();

    // Remove excess spaces (2 or more)
    let search_tags = extra_spaces.replace_all(&search_tags, "");
    // Replace commas and spaces with %
    let search_tags = delimiters.replace_all(&search_tags, "%20");

    let tags = format!("&tags={}", search_tags);
    // No key needed for access
    let api = String::from("https://safebooru.org/index.php?page=dapi&s=post&q=index&limit=100&json=1");
    api.push_str(&tags);

    api
}

// Maybe return all json images instead of just 1 specific image?
fn fetch_api_data(options: RandomArgs) -> ApiData {
    let api = evaluate_arguments(options);

    let res = match reqwest::blocking::get(api) {
        Ok(res) => {
            res.json::<HashMap<String, String>>()
        },
        Err(err) => { 
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };


    // TODO: Fix below later
    let res = match res {
        Ok(data) => data,
        Err(_) => std::process::exit(1),
    };

    let directory = res.get(&"directory");

    ApiData {
        directory: res.directory,
        image: res.image,
        id: res.image,
        image_info: None,
    }
}

fn print_image_details(info: ImageInfo) -> () {
    let ImageInfo { url, width, height, tags } = info;

    println!("{}", url);
}

fn grab_random_image(options: RandomArgs) -> String{
    let data = fetch_api_data(options);
    let ApiData { directory, image, id, image_info } = data;

    let image_url = format!("https://safebooru.org//images/{dir}/{img}?{id}",
                        dir = directory,
                        img = image,
                        id = id
                    );
    
    if options.has_details {
        let Some(image_info) = image_info;
        print_image_details(image_info)
    }
    

    image_url
}

struct ApiData {
    directory: String,
    image: String,
    id: String,
    image_info: Option<ImageInfo>,
}

struct ImageInfo {
    url: String,
    width: u32,
    height: u32,
    tags: Vec<String>,
}