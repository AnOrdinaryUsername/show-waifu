pub mod danbooru;
pub mod safebooru;

pub trait API<T> {
    fn grab_random_image(options: T) -> String;
}
