use crate::app::Danbooru;

pub fn grab_random_image(options: Danbooru) -> String {
    check_env_variables();
    let image_url = String::from("https://danbooru.donmai.us/data/sample/a4/86/__fairy_saratoga_gambier_bay_intrepid_and_hornet_kantai_collection_drawn_by_nakaaki_masashi__sample-a486e9aa705cafa15266703a274b4210.jpg");

    image_url
}

// TODO: Check for Danbooru API key and username
fn check_env_variables() -> bool {
    use std::env;

    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }

    true
}
