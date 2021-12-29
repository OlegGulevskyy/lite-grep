use regex::Regex;
use tests::{get_search_results, get_user_arguments, print_results_to_console};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
	let user_arguments = get_user_arguments();
	let re = Regex::new(&user_arguments.search_pattern).unwrap();
	let file_path = user_arguments.file_path;
	
	let file = File::open(file_path).unwrap();
	let reader = BufReader::new(file);
	let lines: Vec<String> = reader.lines().map(|f|f.unwrap()).collect();

	let search_results = get_search_results(re, &lines);
	if search_results.is_empty() {
		return;
	}

	print_results_to_console(&lines, search_results);
}