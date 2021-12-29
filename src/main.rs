use regex::Regex;
use tests::{get_search_results, get_user_defined_pattern, print_results_to_console};
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