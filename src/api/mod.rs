pub mod danbooru;
pub mod safebooru;
use regex::Regex;

pub fn reformat_search_tags(tags: String) -> String {
    let extra_spaces = Regex::new(r"\s{2,}").unwrap();
    let delimiters = Regex::new(r"[,\s]").unwrap();

    // Remove excess spaces (2 or more)
    extra_spaces.replace_all(&tags, "");
    // Replace commas and spaces with %20
    let search_tags = delimiters.replace_all(&tags, "%20");

    search_tags.to_string()
}
