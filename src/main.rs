use regex::Regex;
use clap::{App, Arg};

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

fn get_search_results(regexp: Regex, text: &str) -> Vec<(usize, usize, usize)> {
	let mut results: Vec<(usize, usize, usize)> = vec![];
	let context_lines_amount = 2;
	let lines_count = text.lines().count();

	for (index, line) in text.lines().enumerate() {
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

fn print_results_to_console(text: &str, search_results: Vec<(usize, usize, usize)>) {
	let lines: Vec<(usize, &str)> = text.lines().enumerate().map(|(index, item)|(index, item)).collect();

	for (_, (_, lower_bound, upper_bound)) in search_results.iter().enumerate() {
		let sliced = &lines[*lower_bound..*upper_bound];

		for (line_index, line_text) in sliced {
			println!("Line {}: {}", line_index + 1, line_text.trim());
		}
	}
}

fn main() {
	
	let pattern = get_user_defined_pattern();
	let re = Regex::new(&pattern).unwrap();
	
	let haystack = "\
		Every face, every shop,
		bedroom window, public-house, and
		dark square is a picture
		feverishly turned--in search of what?
		It is the same with books.
		What do we seek
		through millions of pages?";

	let search_results = get_search_results(re, haystack);
	if search_results.is_empty() {
		return;
	}

	print_results_to_console(haystack, search_results);
}