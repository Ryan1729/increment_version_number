use std::env;
use std::fs::File;
use std::io::{Read, Write, Seek, SeekFrom};
extern crate regex;

use regex::Regex;

fn main() {
    let mut args = env::args();

    args.next();

    if let (Some(filename), Some(regex), Some(capture_indicator)) = (args.next(), args.next(), args.next()) {
        let capture_num = capture_indicator.parse::<usize>().unwrap();

        let re = Regex::new(&regex).unwrap();

        let mut file = File::options().read(true).write(true).open(&filename).unwrap();
        let mut file_string = String::new();
        file.read_to_string(&mut file_string).unwrap();

        if let Some(cap) = re.captures_iter(&file_string).next() {
            let version_str = &cap[capture_num];
            let mut version = version_str.parse::<usize>().unwrap();
            version += 1;

            let mut replacement_string = String::new();

            // The entire match is at index 0; that's why we skip it.
            for i in 1..capture_num {
                // This outputs somthing like "${1}". We use braces to make sure
                // that the proper group is used, no matter what the input file
                // looks like.
                // See https://docs.rs/regex/latest/regex/struct.Regex.html#replacement-string-syntax

                replacement_string.push_str(&format!("${{{i}}}"));
            }

            replacement_string.push_str(&format!("{version}"));

            if cap.len() - 1 > capture_num {
                for i in (capture_num + 1)..cap.len() {
                    replacement_string.push_str(&format!("${{{i}}}"));
                }
            }

            let replacement_str : &str = &replacement_string;

            let replacement_result = re.replace(&file_string, replacement_str);

            file.seek(SeekFrom::Start(0)).unwrap();
            file.write_all(replacement_result.as_bytes()).unwrap();

            println!("overwrote {filename}");
        } else {
            println!("No captures found!")
        };
    } else {
        println!("usage: increment_version_number filename regex capture_indicator")
    }
}
