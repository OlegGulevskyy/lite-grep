use regex::Regex;
use clap::{App, Arg};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
	
	let pattern = get_user_defined_pattern();
	let re = Regex::new(&pattern).unwrap();

	let file = File::open("readme.md").unwrap();
	let reader = BufReader::new(file);
	let lines: Vec<String> = reader.lines().map(|f|f.unwrap()).collect();

	let search_results = get_search_results(re, &lines);
	if search_results.is_empty() {
		return;
	}

	print_results_to_console(&lines, search_results);
}

fn get_user_defined_pattern() -> String {
	let args = App::new("VeryLiteGrep")
		.version("1.0")
		.about("Searches for stuff. There are plenty of good stuff, this is just for learning purposes")
		.arg(Arg::with_name("pattern")
			.help("Pattern is what we will use to find stuff for you in text.")
			.takes_value(true)
			.required(true)
		)
		.get_matches();

	let pattern = args.value_of("pattern").unwrap();
	pattern.to_string()
}

fn get_search_results(regexp: Regex, text: &Vec<String>) -> Vec<(usize, usize, usize)> {
	let mut results: Vec<(usize, usize, usize)> = vec![];
	let context_lines_amount = 2;
	let lines_count = text.len();

	for (index, line) in text.iter().enumerate() {
		let found_re = regexp.find(line);

		match found_re {
			Some(_) => {
				let line = index + 1;
				let lower_bound = line.saturating_sub(context_lines_amount + 1);
				let upper_bound = if (line + context_lines_amount) > lines_count {
					lines_count
				} else {
					line + context_lines_amount
				};

				results.push((line, lower_bound, upper_bound));
			},
			None => (),
		}
	}

	results
}

fn print_results_to_console(text: &Vec<String>, search_results: Vec<(usize, usize, usize)>) {
	for (_, (_, lower_bound, upper_bound)) in search_results.iter().enumerate() {
		let sliced = text[*lower_bound..*upper_bound].iter().enumerate();

		for (line_index, line_text) in sliced {
			println!("Line {}: {}", line_index + 1, line_text.trim());
		}
	}
}