use colored::Colorize;

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