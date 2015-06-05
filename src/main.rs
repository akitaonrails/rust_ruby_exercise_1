use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
extern crate regex;
use regex::Regex;

fn main() {
    let filename = "actors.list";
    let target_movie = "Star Wars: Episode V - The Empire Strikes Back";

    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(&file).lines().skip(239);

    let mut actor = String::new();
    let mut actors : Vec<String> = Vec::new();
    let regex = Regex::new(r"^(.*?)\t+(.*?)$").unwrap();
    loop {
        let line = match reader.next() {
            Some(line) => match line {
                Ok(line) => line,
                Err(_) => String::new(),
            },
            None => break,
        };

        match regex.captures(&line) {
            Some(captures) => {
                let actor_buffer = captures.at(1).unwrap();
                let movie = captures.at(2).unwrap();

                if actor.is_empty() && !actor_buffer.is_empty() {
                    actor = actor_buffer.to_string();
                }

                if !movie.is_empty() && movie.contains(&target_movie) {
                    actors.push(actor.to_string());
                    println!("{}", actor);
                }
            },
            None => {
                actor = String::new();
            }
        };
    }

    for actor_name in actors {
        println!("{}", actor_name);
    }
}
