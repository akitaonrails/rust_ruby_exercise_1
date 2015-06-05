extern crate imdb;
use imdb::find_actors;

fn main() {
    let filename     = "actors_slice.list";
    let target_movie = "The X Factor";

    println!("{}", find_actors(filename.to_string(), 239, target_movie.to_string()));
}
