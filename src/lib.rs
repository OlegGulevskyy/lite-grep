use clap::{App, Arg};
use colored::Colorize;
use regex::Regex;

pub fn get_user_defined_pattern() -> String {
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

pub fn get_search_results(regexp: Regex, text: &Vec<String>) -> Vec<(usize, usize, usize, &String)> {
	let mut results: Vec<(usize, usize, usize, &String)> = vec![];
	let context_lines_amount = 2;
	let lines_count = text.len();

	for (index, line) in text.iter().enumerate() {
		let found_re = regexp.find(line);

		match found_re {
			Some(_) => {
				let hit_text = line;
				let line = index + 1;
				let lower_bound = line.saturating_sub(context_lines_amount + 1);
				let upper_bound = if (line + context_lines_amount) > lines_count {
					lines_count
				} else {
					line + context_lines_amount
				};

				results.push((line, lower_bound, upper_bound, hit_text));
			},
			None => (),
		}
	}

	results
}

pub fn print_results_to_console(text: &Vec<String>, search_results: Vec<(usize, usize, usize, &String)>) {
	for (_, (_, lower_bound, upper_bound, hit_text)) in search_results.iter().enumerate() {
		let sliced = text[*lower_bound..*upper_bound].iter().enumerate();

		for (line_index, line_text) in sliced {

			let line_message = get_line_message(line_index, *hit_text == line_text, line_text);
			println!("{}", line_message);
		}
	}
}

fn get_line_message(line_number: usize, is_primary_line: bool, text: &String) -> String {
	let prefix = String::from(format!("L{}", line_number));

	let line_announce = if is_primary_line {
		String::from(format!("{}: {}", prefix.blue(), text.trim()))
	} else {
		String::from(format!("{}: {}", prefix.bright_blue(), text.trim()))
	};

	line_announce.to_string()
}