





use std::env;
use std::fs::OpenOptions;
use std::io::{Read, Write};
extern crate regex;

use regex::Regex;

fn main() {
    let mut args = env::args();

	args.next();

	if let (Some(filename), Some(regex), Some(capture_indicator)) = (args.next(), args.next(), args.next()) {
		let capture_num = capture_indicator.parse::<usize>().unwrap();

		let re = Regex::new(&regex).unwrap();

		let mut file = OpenOptions::new().read(true).write(true).open(&filename).unwrap();
		let mut file_string = String::new();
		file.read_to_string(&mut file_string).unwrap();

		if let Some(cap) = re.captures_iter(&file_string).next() {
			let version_str = &cap[capture_num];
			let mut version = version_str.parse::<usize>().unwrap();
			version += 1;

			let mut replacement_string = String::new();

			for i in 1..capture_num {
				replacement_string.push_str(&format!("${{{}}}", i));
			}

			replacement_string.push_str(&format!("{}", version));

			if cap.len() - 1 > capture_num { 
				for i in (capture_num + 1)..cap.len() {
					replacement_string.push_str(&format!("${{{}}}", i));
				}
			}

			let replacement_str : &str = &replacement_string;
			println!("{:?}", replacement_str);
			file.write_all(re.replace(&file_string, replacement_str).as_bytes()).unwrap();

			println!("overwrote {}", filename);
		} else {
			println!("No captures found!")
		}
	} else {
		println!("usage: increment_version_number filename regex capture_indicator")
	}
 
}
