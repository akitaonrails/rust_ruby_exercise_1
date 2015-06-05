extern crate imdb;

#[test]
fn it_parses_file() {
    let filename     = "actors_slice.list"; // 10_000 lines sample from the original 938MB file
    let target_movie = "The X Factor";

    let expected = "145, Lyric
3, Utai
4 Real
4Shore
4Sure
4th Ba5e
4Tune
";
    let result = imdb::find_actors(filename.to_string(), 239, target_movie.to_string());

    assert_eq!(expected, result);
}
