use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

extern crate pcre;
use pcre::Pcre;

extern crate libc;
use libc::c_char;
use std::ffi::{CString, CStr};

#[no_mangle]
pub extern "C" fn ffi_find_actors(filename_ptr: *const c_char, skip_lines: i32, target_movie_ptr: *const c_char) -> *const c_char {
    let filename = unsafe {
        CStr::from_ptr(filename_ptr)
    };
    let target_movie = unsafe {
        CStr::from_ptr(target_movie_ptr)
    };
    let result = find_actors(
        std::str::from_utf8(filename.to_bytes()).unwrap().to_string(),
        skip_lines as usize,
        std::str::from_utf8(target_movie.to_bytes()).unwrap().to_string()
        );
    CString::new(result).unwrap().as_ptr()
}

pub fn find_actors(filename: String, skip_lines: usize, target_movie: String) -> String {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(&file).lines().skip(skip_lines);

    let mut actor = String::new();
    let mut actors : Vec<String> = Vec::new();
    let regex = Pcre::compile("^(.*?)\t+(.*?)$").unwrap();

    loop {
        let line = match reader.next() {
            Some(line) => match line {
                Ok(line) => line,
                Err(_)   => String::new(),
            },
            None => break,
        };

        match regex.exec(&line) {
            Some(captures) => {
                let actor_buffer = captures.group(1);
                let movie        = captures.group(2);

                if actor.is_empty() && !actor_buffer.is_empty() {
                    actor = actor_buffer.to_string();
                }

                if !movie.is_empty() && movie.contains(&target_movie) && !actors.contains(&actor) {
                    actors.push(actor.to_string());
                }
            },
            None => {
                actor = String::new();
            }
        };
    }
    actors.connect("\n")
}
